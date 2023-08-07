use std::{
  collections::{HashMap, HashSet},
  fs::File,
  io::{BufReader, Read},
};

use flate2::bufread::GzDecoder;

use crate::Result;
use std::path::Path;

mod parse_man;

#[derive(Debug)]
pub struct CommandInfo {
  pub args: Vec<Arg>,
  pub subcommands: HashMap<String, CommandInfo>,
}

#[derive(Debug)]
pub struct Arg {
  pub forms: HashSet<String>,
  pub desc: String,
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
        todo!()
      }
    }
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
  let res = parse_man::parse(cmd_name, text)?;
  Ok(res.unwrap())
}
