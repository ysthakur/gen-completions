mod type1;
mod type2;
pub(self) mod util;

use anyhow::{anyhow, Result};
use flate2::bufread::GzDecoder;
use log::{debug, error, trace, warn};
use regex::Regex;
use std::{
  collections::{hash_map::Entry, HashMap, HashSet},
  fs::File,
  io::{BufReader, Read},
  path::{Path, PathBuf},
  process::Command,
};

#[derive(Debug)]
pub struct CommandInfo {
  pub args: Vec<Arg>,
  pub subcommands: HashMap<String, CommandInfo>,
}

#[derive(Debug)]
pub struct Arg {
  pub forms: Vec<String>,
  pub desc: Option<String>,
}

pub fn parse_manpage_at_path<P>(cmd_name: &str, path: P) -> Result<Option<Vec<Arg>>>
where
  P: AsRef<Path>,
{
  let text = read_manpage(path)?;
  Ok(parse_manpage_text(cmd_name, text))
}

pub fn parse_manpage_text<S: AsRef<str>>(cmd_name: &str, text: S) -> Option<Vec<Arg>> {
  let text = text.as_ref();
  type1::parse(cmd_name, text).or_else(|| type2::parse(cmd_name, text))
}

/// Configuration for parsing the man pages
///
/// Note: The properties concerning matching commands try to match the file
/// names of the man pages, not the command names. If you're trying to match
/// `git log`, you need to instead try to match `git-log` (since the man page is
/// named `git-log.1`)
pub struct ManParseConfig {
  manpath: Option<HashSet<PathBuf>>,
  excluded_sections: Vec<u8>,
  excluded_dirs: Vec<PathBuf>,
  include_commands: Option<Regex>,
  exclude_commands: Option<Regex>,
  not_subcommands: Vec<String>,
}

impl ManParseConfig {
  /// Create a new [Config] with the defaults
  pub fn new() -> ManParseConfig {
    ManParseConfig {
      manpath: None,
      excluded_sections: Vec::new(),
      excluded_dirs: Vec::new(),
      include_commands: None,
      exclude_commands: None,
      not_subcommands: Vec::new(),
    }
  }

  /// Set the manpath explicitly.
  ///
  /// This method tries to canonicalize the paths, so it may fail.
  pub fn manpath<I, P>(mut self, manpath: I) -> Result<Self>
  where
    I: Iterator<Item = P>,
    P: AsRef<Path>,
  {
    self.manpath = Some(
      manpath
        .into_iter()
        .map(|p| std::fs::canonicalize(p))
        .collect::<Result<_, _>>()?,
    );
    Ok(self)
  }

  // TODO figure out why fish only seems to use man1, man6, and man8
  pub fn exclude_sections<I>(mut self, sections: I) -> Self
  where
    I: IntoIterator<Item = u8>,
  {
    for section in sections {
      if 1 <= section && section <= 8 {
        self.excluded_sections.push(section);
      } else {
        error!("Tried excluding invalid section (must be 1-8): {}", section);
      }
    }
    self
  }

  pub fn exclude_dirs<I, P>(mut self, dirs: I) -> Self
  where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
  {
    for dir in dirs {
      let path = PathBuf::from(dir.as_ref());
      if path.exists() {
        self.excluded_dirs.push(path)
      } else {
        error!("Excluded directory does not exist: {}", path.display());
      }
    }
    self
  }

  /// Only search for specific commands (by default, all commands are searched
  /// for).
  ///
  /// If a command has been explicitly marked as not being a subcommand
  /// using [not_subcommands()], then the regex must match the entire man page
  /// file name stem. Otherwise, it need only match a part at the start, as long
  /// as there is a `'-'` right after the match.
  pub fn restrict_to_commands(mut self, regex: Regex) -> Self {
    self.include_commands = Some(regex);
    self
  }

  /// Exclude certain commands from being searched for. The regex must match the
  /// entire man page file name stem (e.g. `/git/` will not exclude `git-log`).
  pub fn exclude_commands(mut self, regex: Regex) -> Self {
    self.exclude_commands = Some(regex);
    self
  }

  /// Mark commands that you don't want being seen as subcommands
  pub fn not_subcommands<I>(mut self, cmds: I) -> Self
  where
    I: IntoIterator<Item = String>,
  {
    for cmd in cmds {
      self.not_subcommands.push(cmd);
    }
    self
  }

  /// Actually do the parsing
  pub fn parse(self) -> anyhow::Result<HashMap<String, CommandInfo>> {
    let manpath = self.manpath.or_else(|| get_manpath()).ok_or(anyhow!(
      "No manpages found. Please explicitly give manpath or set $MANPATH."
    ))?;

    let included_dirs: Vec<PathBuf> = manpath
      .into_iter()
      .filter(|path| !self.excluded_dirs.contains(path))
      .collect();
    if included_dirs.is_empty() {
      return Err(anyhow!("All directories excluded, nowhere to search"));
    }

    let included_sections = (1..8)
      .into_iter()
      .filter(|s| !self.excluded_sections.contains(s))
      .collect::<Vec<_>>();
    if included_sections.is_empty() {
      return Err(anyhow!("All sections excluded, nowhere to search"));
    }

    let all_manpages = enumerate_manpages(included_dirs, included_sections);
    let filtered = filter_pages(
      all_manpages,
      self.include_commands,
      self.exclude_commands,
      &self.not_subcommands,
    )?;

    let parsed = parse_all_manpages(filtered);

    Ok(parsed)
  }
}

fn insert_cmd(
  subcommands: &mut HashMap<String, CommandInfo>,
  mut cmd_parts: Vec<String>,
  mut args: Vec<Arg>,
) {
  let head = cmd_parts.remove(0);
  let cmd = match subcommands.entry(head) {
    Entry::Occupied(o) => o.into_mut(),
    Entry::Vacant(v) => v.insert(CommandInfo {
      args: Vec::new(),
      subcommands: HashMap::new(),
    }),
  };
  if cmd_parts.is_empty() {
    cmd.args.append(&mut args);
  } else {
    insert_cmd(&mut cmd.subcommands, cmd_parts, args);
  }
}

fn parse_all_manpages(manpages: Vec<(String, PathBuf)>) -> HashMap<String, CommandInfo> {
  let mut res = HashMap::new();

  for (cmd, manpage) in manpages {
    if let Ok(text) = read_manpage(&manpage) {
      let cmd_name = get_cmd_name(&manpage);
      match parse_manpage_text(&cmd_name, &text) {
        Some(parsed) => {
          trace!("Parsing man page for {} at {}", cmd_name, manpage.display());
          match detect_subcommand(&cmd_name, &text) {
            Some(cmd_parts) => insert_cmd(&mut res, cmd_parts, parsed),
            None => insert_cmd(&mut res, vec![cmd_name], parsed),
          }
        }
        None => {
          error!("Could not parse manpage for {}", cmd_name);
        }
      }
    } else {
      error!(
        "Could not read manpage for {} at {}",
        cmd,
        manpage.display()
      )
    }
  }

  res
}

fn filter_pages(
  all_manpages: Vec<(String, PathBuf)>,
  include_commands: Option<Regex>,
  exclude_commands: Option<Regex>,
  not_subcommands: &[String],
) -> Result<Vec<(String, PathBuf)>> {
  let filtered = all_manpages
    .into_iter()
    .filter(|(cmd, path)| {
      let include = match &include_commands {
        Some(re) => {
          match re.find(cmd) {
            Some(mat) if mat.start() == 0 => {
              if not_subcommands.contains(cmd) {
                mat.end() == cmd.len()
              } else {
                // If it's a subcommand, then it might only match the start and
                // have a hyphen after
                mat.end() == cmd.len() || cmd.chars().nth(mat.end() + 1).unwrap() == '-'
              }
            }
            _ => false,
          }
        }
        None => true,
      };
      let exclude = match &exclude_commands {
        Some(re) => re
          .find(cmd)
          .map(|mat| mat.start() == 0 && mat.end() == cmd.len())
          .unwrap_or(false),
        None => false,
      };

      if include {
        debug!("Found man page for {} at {}", cmd, path.display());
      }
      if exclude && include_commands.is_some() {
        warn!("Command was both explicitly included and excluded: {}", cmd);
      }

      include && !exclude
    })
    .collect::<Vec<_>>();

  if filtered.is_empty() {
    Err(anyhow!("All commands were either excluded or not included"))
  } else {
    Ok(filtered)
  }
}

pub fn read_manpage<P>(manpage_path: P) -> Result<String>
where
  P: AsRef<Path>,
{
  let path = manpage_path.as_ref();
  match path.extension() {
    Some(ext) => {
      if ext == "gz" {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut decoder = GzDecoder::new(reader);
        let mut str = String::new();
        // TODO this only works with UTF-8
        decoder.read_to_string(&mut str)?;
        Ok(str)
      } else {
        let contents = std::fs::read_to_string(path)?;
        Ok(contents)
      }
    }
    None => todo!(),
  }
}

/// Get the command that a manpage is for, given its path
///
/// e.g. `/foo/cowsay.1.txt -> "cowsay"`
fn get_cmd_name(manpage_path: &Path) -> String {
  let file_name = manpage_path
    .file_name()
    .unwrap()
    .to_string_lossy()
    .replace(std::char::REPLACEMENT_CHARACTER, "");
  // The file name will be something like foo.1.gz, we only want foo
  file_name
    .split(".")
    .nth(0)
    .unwrap_or_else(|| &file_name)
    .to_string()
}

/// Try to detect if the given command is actually a subcommand.
///
/// Given command `git-log`, the result would be `Some(vec!["git", "log"])`
fn detect_subcommand(cmd_name: &str, text: &str) -> Option<Vec<String>> {
  let parts = cmd_name.split("-").collect::<Vec<_>>();
  let as_sub_cmd = parts.join(" ");
  if text.contains(&as_sub_cmd) {
    Some(parts.iter().map(|s| s.to_string()).collect())
  } else {
    None
  }
}

/// Find the search path for man
///
/// Looks at `$MANPATH` first, then tries running `manpath`, then `man --path`.
fn get_manpath() -> Option<HashSet<PathBuf>> {
  fn split_path(path: &str) -> HashSet<PathBuf> {
    path
      .split(":")
      .filter_map(|path| {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() {
          Some(std::fs::canonicalize(path_buf).unwrap())
        } else {
          None
        }
      })
      .collect()
  }

  if let Ok(manpath) = std::env::var("MANPATH") {
    Some(split_path(&manpath))
  } else {
    fn from_cmd(cmd: &mut Command) -> Option<HashSet<PathBuf>> {
      cmd
        .output()
        .ok()
        .map(|out| split_path(std::str::from_utf8(&out.stdout).unwrap()))
    }
    from_cmd(&mut Command::new("manpath"))
      .or_else(|| from_cmd(Command::new("man").arg("--path")))
  }
}

/// Enumerate all manpages given a list of directories to search in. Returns the
/// command names and their paths.
///
/// ## Arguments
/// * `manpath` - Directories that man searches in (`$MANPATH/manpath/man
///   --path`). Inside each of these directories should be `man1`, `man2`, etc.
///   folders. The paths should be canonical.
/// * `include_sections` - Man sections to include (1-8)
fn enumerate_manpages<I, P>(
  manpath: I,
  include_sections: Vec<u8>,
) -> Vec<(String, PathBuf)>
where
  I: IntoIterator<Item = P>,
  P: AsRef<Path>,
{
  let mut res = vec![];

  let section_names: Vec<_> =
    include_sections.iter().map(|n| format!("man{n}")).collect();

  for parent_path in manpath.into_iter().filter(|p| p.as_ref().is_dir()) {
    for section_name in &section_names {
      let section_dir = parent_path.as_ref().join(section_name);
      if let Ok(manpages) = std::fs::read_dir(section_dir) {
        for manpage in manpages.filter_map(|p| p.ok()) {
          let path = manpage.path();
          res.push((get_cmd_name(&path), path));
        }
      }
    }
  }

  res.sort();

  res
}
