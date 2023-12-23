pub mod gen;
pub mod parse_deser;
pub mod parse_man;

use serde::{Deserialize, Serialize};

/// Flags parsed from a command, as well as its parsed subcommands
#[derive(Debug, Deserialize, Eq, Serialize, PartialEq)]
pub struct CommandInfo {
  pub name: String,
  pub flags: Vec<Flag>,
  pub subcommands: Vec<CommandInfo>,
}

/// A parsed flag
#[derive(Debug, Deserialize, Eq, Serialize, PartialEq)]
pub struct Flag {
  /// The different short and long forms of a flag
  pub forms: Vec<String>,
  /// Optional description for the flag
  pub desc: Option<String>,
}

pub enum Error {
  
}
