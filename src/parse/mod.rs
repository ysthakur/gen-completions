mod type1;
mod type2;
mod util;

use std::{
  collections::{hash_map::Entry, HashMap},
  fs::File,
  io::{BufReader, Read},
  path::{Path, PathBuf},
};

use anyhow::{anyhow, Error, Result};
use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use log::{debug, trace};

/// Flags parsed from a command, as well as its parsed subcommands
#[derive(Debug)]
pub struct CommandInfo {
  pub flags: Vec<Flag>,
  pub subcommands: HashMap<String, CommandInfo>,
}

/// A parsed flag
#[derive(Debug)]
pub struct Flag {
  /// The different short and long forms of a flag
  pub forms: Vec<String>,
  pub desc: Option<String>,
}

/// Information about a command and its subcommands before being parsed
pub struct CmdPreInfo {
  path: Option<PathBuf>,
  subcmds: HashMap<String, CmdPreInfo>,
}

/// Get the command that a manpage is for, given its path
///
/// e.g. `/foo/cowsay.1.txt -> "cowsay"`
pub fn get_cmd_name<P>(manpage_path: P) -> String
where
  P: AsRef<Path>,
{
  let file_name = manpage_path
    .as_ref()
    .file_name()
    .unwrap()
    .to_string_lossy()
    .replace(std::char::REPLACEMENT_CHARACTER, "");
  // The file name will be something like foo.1.gz, we only want foo
  file_name
    .split('.')
    .next()
    .unwrap_or(&file_name)
    .to_string()
}

pub fn parse_manpage_text<S>(text: S) -> Option<Vec<Flag>>
where
  S: AsRef<str>,
{
  let text = text.as_ref();
  type1::parse(text).or_else(|| type2::parse(text))
}

/// Decompress a manpage if necessary
pub fn read_manpage<P>(manpage_path: P) -> Result<String>
where
  P: AsRef<Path>,
{
  let path = manpage_path.as_ref();
  trace!("Reading man page at {}", path.display());
  match path.extension() {
    Some(ext) => {
      let file = File::open(path)?;
      let mut reader = BufReader::new(file);
      let mut str = String::new();
      // TODO GzDecoder and BzDecoder seem to only work with UTF-8?
      if ext == "gz" {
        GzDecoder::new(reader).read_to_string(&mut str)?;
      } else if ext == "bz2" {
        BzDecoder::new(reader).read_to_string(&mut str)?;
      } else {
        reader.read_to_string(&mut str)?;
      }
      Ok(str)
    }
    None => todo!(),
  }
}

/// Take a `CmdPreInfo` representing the path to a command and its subcommands
/// and try parsing that command and its subcommands. Also returns a list of
/// errors encountered along the way.
pub fn parse_from(
  cmd_name: &str,
  pre_info: CmdPreInfo,
) -> (CommandInfo, Vec<Error>) {
  let mut flags = Vec::new();
  let mut subcommands = HashMap::new();
  let mut errors = Vec::new();

  if let Some(path) = pre_info.path {
    match read_manpage(path) {
      Ok(text) => {
        if let Some(mut parsed) = parse_manpage_text(text) {
          flags.append(&mut parsed);
        } else {
          errors.push(anyhow!("Could not parse man page for '{}'", cmd_name));
        }
      }
      Err(e) => {
        errors.push(e);
      }
    }
  } else {
    errors.push(anyhow!(
      "Man page for parent command '{}' not found",
      cmd_name
    ));
  }

  for (sub_name, sub_info) in pre_info.subcmds {
    let (subcmd, mut sub_errors) =
      parse_from(&format!("{cmd_name} {sub_name}"), sub_info);
    subcommands.insert(sub_name, subcmd);
    errors.append(&mut sub_errors);
  }

  (CommandInfo { flags, subcommands }, errors)
}

/// Make a tree relating commands to their subcommands
pub fn detect_subcommands<I, P, S>(
  manpages: I,
  explicit_subcmds: S,
) -> HashMap<String, CmdPreInfo>
where
  I: IntoIterator<Item = P>,
  P: AsRef<Path>,
  S: IntoIterator<Item = (String, Vec<String>)>,
{
  let mut explicit_subcmds: HashMap<_, _> =
    explicit_subcmds.into_iter().collect();

  let mut res = HashMap::new();

  for page in manpages {
    let page = PathBuf::from(page.as_ref());
    let cmd_name = get_cmd_name(&page);
    match explicit_subcmds.remove(&cmd_name) {
      Some(as_subcmd) => insert_subcmd(&mut res, as_subcmd, page),
      None => {
        if let Ok(text) = read_manpage(&page) {
          insert_subcmd(&mut res, detect_subcommand(&cmd_name, &text), page);
        }
      }
    }
  }

  res
}

/// Insert a subcommand into a tree of subcommands
fn insert_subcmd(
  subcommands: &mut HashMap<String, CmdPreInfo>,
  mut cmd_parts: Vec<String>,
  path: PathBuf,
) {
  let head = cmd_parts.remove(0);
  let cmd = match subcommands.entry(head) {
    Entry::Occupied(o) => o.into_mut(),
    Entry::Vacant(v) => v.insert(CmdPreInfo {
      path: None,
      subcmds: HashMap::new(),
    }),
  };
  if cmd_parts.is_empty() {
    cmd.path = Some(path);
  } else {
    insert_subcmd(&mut cmd.subcmds, cmd_parts, path);
  }
}

/// Try to detect if the given command is actually a subcommand and break it up
/// into its pieces.
///
/// Given command `git-log`, the result would be `vec!["git", "log"]`. A single
/// command like `git` would be `vec!["git"]`.
fn detect_subcommand(cmd_name: &str, text: &str) -> Vec<String> {
  let mut chars = cmd_name.chars();
  let mut hyphens = vec![0];
  for i in 0..cmd_name.len() {
    if chars.next().unwrap() == '-' {
      hyphens.push(i + 1);
    }
  }
  hyphens.push(cmd_name.len() + 1);

  if hyphens.len() > 2 {
    for poss in all_possible_subcommands(&hyphens, cmd_name) {
      let as_sub_cmd = poss.join(" ").replace('-', r"\-");
      if text.contains(&as_sub_cmd) {
        debug!("Detected {} as subcommand {}", cmd_name, as_sub_cmd);
        return poss.into_iter().map(String::from).collect();
      }
    }
  }

  vec![cmd_name.to_string()]
}

/// Find all possible subcommands that might have the given hyphenated man page
/// name
///
/// ## Arguments
/// * `hyphens` - The locations of the hyphens in the string (also, the first
///   element is the index of the start of the current substring, and the last
///   element is the index of the end of the current substring)
fn all_possible_subcommands<'a>(
  hyphens: &[usize],
  cmd: &'a str,
) -> Vec<Vec<&'a str>> {
  if hyphens.len() == 2 {
    Vec::new()
  } else {
    let mut res = Vec::new();

    for i in 1..hyphens.len() - 1 {
      let mid = hyphens[i];
      let mut all_right = all_possible_subcommands(&hyphens[i..], cmd);
      all_right.push(vec![&cmd[mid..hyphens[hyphens.len() - 1] - 1]]);
      for right in all_right {
        let mut all_left = all_possible_subcommands(&hyphens[..=i], cmd);
        all_left.push(vec![&cmd[hyphens[0]..mid - 1]]);
        for mut left in all_left {
          left.extend_from_slice(&right);
          res.push(left);
        }
      }
    }

    res
  }
}
