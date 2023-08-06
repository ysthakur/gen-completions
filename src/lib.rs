mod parse;
mod gen;

use parse::Parsed;
use std::path::PathBuf;

pub fn get_manpath() -> Option<Vec<PathBuf>> {
  if let Ok(manpath) = std::env::var("MANPATH") {
    Some(manpath.split(":").map(|path| PathBuf::from(path)).collect())
  } else {
    todo!()
  }
}

pub fn find_manpage(cmd: &str, manpath: Vec<PathBuf>) -> Option<PathBuf> {
  None
}

// TODO figure out why fish only seems to use man1, 6, and 8
static SECTIONS: [&str; 8] = [
  "man1", "man2", "man3", "man4", "man5", "man6", "man7", "man8",
];

pub fn enumerate_manpages(manpath: Vec<PathBuf>) -> Vec<PathBuf> {
  let mut res = vec![];

  for parent_path in manpath.iter().filter(|p| p.is_dir()) {
    for section_name in SECTIONS {
      let section_dir = parent_path.join(section_name);
      if let Ok(manpages) = std::fs::read_dir(section_dir) {
        for manpage in manpages.filter_map(|p| p.ok()) {
          if let Ok(path) = std::fs::canonicalize(manpage.path()) {
            res.push(path)
          }
        }
      }
    }
  }

  res
}

pub fn parse_all_manpages(manpages: Vec<PathBuf>) -> Vec<Parsed> {
  let mut res = vec![];
  for manpage in manpages {
    if let Ok(text) = std::fs::read_to_string(&manpage) {
      let file_name = manpage
        .file_name()
        .unwrap()
        .to_string_lossy()
        .replace(std::char::REPLACEMENT_CHARACTER, "");
      // The file name will be something like foo.1.gz, we only want foo
      let cmd_name = file_name.split(".").nth(0).unwrap_or_else(|| &file_name);
      if let Some(parsed) = parse_manpage(cmd_name.to_string(), &text) {
        res.push(parsed);
      }
    }
  }

  res
}

pub fn parse_manpage(cmd_name: String, text: &str) -> Option<Parsed> {
  todo!()
}
