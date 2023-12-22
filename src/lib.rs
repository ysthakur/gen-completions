pub mod gen;
pub mod parse_man;
pub mod parse_deser;

use serde::{Deserialize, Serialize};

/// Flags parsed from a command, as well as its parsed subcommands
#[derive(Debug, Deserialize, Serialize)]
pub struct CommandInfo {
  pub name: String,
  pub flags: Vec<Flag>,
  pub subcommands: Vec<CommandInfo>,
}

/// A parsed flag
#[derive(Debug, Deserialize, Serialize)]
pub struct Flag {
  /// The different short and long forms of a flag
  pub forms: Vec<String>,
  /// Optional description for the flag
  pub desc: Option<String>,
}
