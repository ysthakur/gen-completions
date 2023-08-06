pub mod gen;
pub mod parse;
pub mod result;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use parse::{parse_manpage_text, read_manpage, CommandInfo};
use result::Result;

/// Find the search path for man
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

/// Enumerate all manpages given a list of directories to search in
///
/// ## Arguments
/// * `manpath` - Directories that man searches in (`$MANPATH/manpath/man --path`).
///     Inside each of these directories should be `man1`, `man2`, etc. folders.
///     The paths should be canonical.
/// * `exclude_sections` - Man sections to exclude, if any (1-8)
pub fn enumerate_manpages<I, P, S>(manpath: I, exclude_sections: S) -> Vec<PathBuf>
where
  I: IntoIterator<Item = P>,
  P: AsRef<Path>,
  S: IntoIterator<Item = u8>,
{
  let mut res = vec![];

  // TODO figure out why fish only seems to use man1, man6, and man8
  let exclude: Vec<u8> = exclude_sections.into_iter().collect();
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

/// Get the command that a manpage is for, given its path
fn manpage_cmd(manpage_path: &Path) -> String {
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

/// Find the manpage for a specific command
///
/// ## Arguments
/// * `cmd` - The command to find the manpage for
/// * `manpath` - Directories that man searches in  (`$MANPATH/manpath/man --path`).
///     Inside each of these directories should be `man1`, `man2`, etc. folders.
///     The paths should be canonical.
pub fn find_manpage<P, I>(cmd: &str, manpath: I) -> Option<PathBuf>
where
  P: AsRef<Path>,
  I: IntoIterator<Item = P>,
{
  for manpage in enumerate_manpages(manpath, vec![]) {
    if cmd == manpage_cmd(&manpage) {
      return Some(manpage);
    }
  }

  None
}

pub fn parse_all_manpages<I, P>(manpages: I) -> HashMap<String, CommandInfo>
where
  I: IntoIterator<Item = P>,
  P: AsRef<Path>,
{
  let mut res = HashMap::new();

  for manpage in manpages {
    if let Ok(text) = read_manpage(&manpage) {
      let cmd_name = manpage_cmd(manpage.as_ref());
      match parse_manpage_text(&cmd_name, &text) {
        Ok(parsed) => {
          res.insert(cmd_name, parsed);
        }
        Err(err) => {
          // TODO implement Display? or maybe collect errors
          eprintln!("{:?}", err);
        }
      }
    }
  }

  res
}
