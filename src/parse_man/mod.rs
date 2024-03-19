//! For parsing command information from man pages
mod darwin;
pub mod error;
mod podman;
mod scdoc;
mod type1;
mod type2;
mod type3;
mod type4;
mod util;

use std::{
  collections::{hash_map::Entry, HashMap},
  fs::File,
  io::{BufReader, Read},
  path::{Path, PathBuf},
};

use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use log::{debug, trace};

use crate::{parse_man::error::Error, CommandInfo, Flag};

pub type Result<T> = std::result::Result<T, Error>;

/// Information about a command and its detected subcommands before being parsed
pub struct CmdPreInfo {
  path: Option<PathBuf>,
  subcmds: HashMap<String, CmdPreInfo>,
}

/// Get the command that a manpage is for, given its path
///
/// e.g. `/foo/cowsay.1.txt -> "cowsay"`
#[must_use]
pub fn get_cmd_name(manpage_path: impl AsRef<Path>) -> String {
  let file_name = manpage_path
    .as_ref()
    .file_name()
    .expect("Manpage should've had a valid file name")
    .to_string_lossy()
    .replace(std::char::REPLACEMENT_CHARACTER, "");
  // The file name will be something like foo.1.gz, we only want foo
  if let Some(ind) = file_name.find('.') {
    file_name[..ind].to_string()
  } else {
    file_name.to_string()
  }
}

/// Parse flags from a man page, trying all of the different parsers and merging
/// their results if multiple parsers could parse the man page.
pub fn parse_manpage_text(cmd_name: &str, text: impl AsRef<str>) -> Vec<Flag> {
  let text = text.as_ref();

  // TODO remove duplicate flags
  [
    type1::parse(cmd_name, text),
    type2::parse(cmd_name, text),
    type3::parse(cmd_name, text),
    type4::parse(cmd_name, text),
    scdoc::parse(cmd_name, text),
    podman::parse(cmd_name, text),
    darwin::parse(cmd_name, text),
  ]
  .into_iter()
  .flatten()
  .collect::<Vec<_>>()
}

/// Decompress a manpage if necessary
///
/// # Errors
///
/// Fails if the manpage could not beo pened, or if it was a .gz or .bz2 file
/// and could not be decompressed.
pub fn read_manpage(manpage_path: impl AsRef<Path>) -> std::io::Result<String> {
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
#[must_use]
pub fn parse_from(
  cmd_name: &str,
  pre_info: CmdPreInfo,
) -> (Option<CommandInfo>, Vec<Error>) {
  // todo actually parse arg types
  let args = Vec::new();
  let mut subcommands = Vec::new();
  let mut errors = Vec::new();

  let flags = if let Some(path) = pre_info.path {
    match read_manpage(path.clone()) {
      Ok(text) => {
        let all_flags = parse_manpage_text(cmd_name, text);
        if all_flags.is_empty() {
          errors.push(Error::UnsupportedFormat { path });
          Vec::new()
        } else {
          all_flags
        }
      }
      Err(e) => {
        errors.push(e.into());
        Vec::new()
      }
    }
  } else {
    errors.push(Error::ManpageNotFound {
      cmd_name: cmd_name.to_string(),
    });
    Vec::new()
  };

  for (sub_name, sub_info) in pre_info.subcmds {
    let (sub_cmd, mut sub_errors) =
      parse_from(&format!("{cmd_name} {sub_name}"), sub_info);
    if let Some(cmd) = sub_cmd {
      subcommands.push(cmd);
    }
    errors.append(&mut sub_errors);
  }

  let cmd_info = if flags.is_empty() && subcommands.is_empty() {
    None
  } else {
    subcommands.sort_by(|a, b| a.name.cmp(&b.name));
    Some(CommandInfo {
      name: cmd_name.split(' ').last().unwrap().to_string(),
      desc: None,
      flags,
      args,
      subcommands,
    })
  };
  (cmd_info, errors)
}

/// Make a tree relating commands to their subcommands
#[must_use]
pub fn detect_subcommands(
  manpages: impl IntoIterator<Item = impl AsRef<Path>>,
  explicit_subcmds: impl IntoIterator<Item = (String, Vec<String>)>,
) -> HashMap<String, CmdPreInfo> {
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
