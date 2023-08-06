use std::collections::HashMap;

use crate::Result;
use std::{io, path::Path};

pub mod parse_man;

pub struct CommandInfo {
  opts: Vec<Opt>,
  subcommands: HashMap<String, CommandInfo>,
}

pub struct Opt {
  forms: Vec<String>,
  desc: String,
}

pub fn read_manpage<P>(manpage_path: P) -> Result<String>
where
  P: AsRef<Path>,
{
  let path_ref = manpage_path.as_ref();
  match path_ref.extension() {
    Some(ext) => todo!(),
    None => todo!(),
  }
}

pub fn parse_manpage_at_path<P>(cmd_name: &str, path: P) -> Result<CommandInfo>
where
  P: AsRef<Path>,
{
  read_manpage(path).and_then(|text| parse_manpage_text(cmd_name, &text))
}

pub fn parse_manpage_text(cmd_name: &str, text: &str) -> Result<CommandInfo> {
  todo!()
}
