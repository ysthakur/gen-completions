use std::path::Path;

use crate::parse::CommandInfo;
use anyhow::Result;

pub mod zsh;

pub trait Completions {
  fn generate<P>(cmd_name: String, cmd_info: CommandInfo, out_dir: P) -> Result<()>
  where
    P: AsRef<Path>;

  fn generate_all<I, P>(cmds: I, out_dir: P) -> Result<()>
  where
    I: Iterator<Item = (String, CommandInfo)>,
    P: AsRef<Path>,
  {
    cmds
      .map(|(cmd_name, cmd_info)| <Self as Completions>::generate(cmd_name, cmd_info, &out_dir))
      .collect()
  }
}
