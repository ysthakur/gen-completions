pub mod gen;
pub mod parse;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use parse::{parse_manpage_text, read_manpage, CommandInfo};
use thiserror::Error;

// TODO figure out why fish only seems to use man1, 6, and 8
static SECTIONS: [&str; 8] = [
  "man1", "man2", "man3", "man4", "man5", "man6", "man7", "man8",
];

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("could not parse manpage")]
  ParseError(String),

  #[error(transparent)]
  IoError(#[from] std::io::Error),

  #[error("no manpages found")]
  NoManPages,
}

/// Find the manpath (search path for man)
///
/// Looks at `$MANPATH` first, then tries running `manpath`, then `man --path`.
pub fn get_manpath() -> Option<HashSet<PathBuf>> {
  fn split_path(path: &str) -> HashSet<PathBuf> {
    path
      .split(":")
      .map(|path| std::fs::canonicalize(PathBuf::from(path)).unwrap())
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
    from_cmd(&mut Command::new("manpath")).or_else(|| from_cmd(Command::new("man").arg("--path")))
  }
}

pub fn find_manpage<P: AsRef<Path>, I>(cmd: &str, manpath: I) -> Result<P>
where
  I: IntoIterator<Item = P>,
{
  todo!()
}

pub fn enumerate_manpages<I, P>(manpath: I) -> Vec<PathBuf>
where
  I: IntoIterator<Item = P>,
  P: AsRef<Path>,
{
  let mut res = vec![];

  for parent_path in manpath.into_iter().filter(|p| p.as_ref().is_dir()) {
    for section_name in SECTIONS {
      let section_dir = parent_path.as_ref().join(section_name);
      if let Ok(manpages) = std::fs::read_dir(section_dir) {
        for manpage in manpages.filter_map(|p| p.ok()) {
          if let Ok(path) = std::fs::canonicalize(manpage.path()) {
            res.push(path)
          }
        }
      }
    }
  }

  res.sort();

  res
}

pub fn parse_all_manpages<I, P>(manpages: I) -> HashMap<String, CommandInfo>
where
  I: IntoIterator<Item = P>,
  P: AsRef<Path>,
{
  let mut res = HashMap::new();

  for manpage in manpages {
    if let Ok(text) = read_manpage(&manpage) {
      let file_name = manpage
        .as_ref()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .replace(std::char::REPLACEMENT_CHARACTER, "");
      // The file name will be something like foo.1.gz, we only want foo
      let cmd_name = file_name.split(".").nth(0).unwrap_or_else(|| &file_name);
      match parse_manpage_text(cmd_name, &text) {
        Ok(parsed) => {
          res.insert(cmd_name.to_string(), parsed);
        }
        Err(err) => {
          // TODO implement Display?
          eprintln!("{:?}", err);
        }
      }
    }
  }

  res
}
