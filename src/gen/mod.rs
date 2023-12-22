pub mod bash;
pub mod json;
pub mod nu;
mod util;
pub mod zsh;

/// Flags parsed from a command, as well as its parsed subcommands
#[derive(Debug)]
pub struct CommandInfo {
  pub name: String,
  pub flags: Vec<Flag>,
  pub subcommands: Vec<CommandInfo>,
}

/// A parsed flag
#[derive(Debug)]
pub struct Flag {
  /// The different short and long forms of a flag
  pub forms: Vec<String>,
  pub desc: Option<String>,
}
