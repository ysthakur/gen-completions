pub mod gen;
pub mod parse;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use parse::{parse_manpage_text, read_manpage, CommandInfo};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("Could not parse manpage")]
  ParseError(String),

  #[error(transparent)]
  IoError(#[from] std::io::Error),

  #[error("No manpages found. Please set the MANPATH environment variable.")]
  NoManPages,
}

/// Find the manpath (search path for man)
///
/// Looks at `$MANPATH` first, then tries running `manpath`, then `man --path`.
pub fn get_manpath() -> Option<HashSet<PathBuf>> {
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
    from_cmd(&mut Command::new("manpath")).or_else(|| from_cmd(Command::new("man").arg("--path")))
  }
}

pub fn find_manpage<P: AsRef<Path>, I>(cmd: &str, manpath: I) -> Result<P>
where
  I: IntoIterator<Item = P>,
{
  todo!()
}

/// Enumerate all manpages given a list of directories to search in
///
/// * manpath - Directories that man searches in (`$MANPATH/manpath/man --path`).
///     Inside each of these directories should be `man1`, `man2`, etc. folders.
///     The paths should be canonical.
/// * exclude_sections - Man sections to exclude, if any (1-8)
pub fn enumerate_manpages<I, P, S>(manpath: I, exclude_sections: Option<S>) -> Vec<PathBuf>
where
  I: IntoIterator<Item = P>,
  P: AsRef<Path>,
  S: IntoIterator<Item = u8>,
{
  let mut res = vec![];

  // TODO figure out why fish only seems to use man1, man6, and man8
  let exclude: Vec<u8> = if let Some(sections) = exclude_sections {
    sections.into_iter().collect()
  } else {
    Vec::new()
  };
  let section_names: Vec<_> = (1u8..8u8)
    .filter(|n| !exclude.contains(&n))
    .map(|n| format!("man{n}"))
    .collect();

  for parent_path in manpath.into_iter().filter(|p| p.as_ref().is_dir()) {
    for section_name in &section_names {
      let section_dir = parent_path.as_ref().join(section_name);
      if let Ok(manpages) = std::fs::read_dir(section_dir) {
        for manpage in manpages.filter_map(|p| p.ok()) {
          res.push(manpage.path())
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
