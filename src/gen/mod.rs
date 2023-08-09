mod bash;
mod json;
pub(self) mod util;
mod zsh;

use std::path::Path;

use crate::parse::CommandInfo;
use anyhow::Result;

pub use bash::*;
pub use json::*;
pub use zsh::*;

pub trait Completions {
  fn generate<P>(cmd_name: String, cmd_info: CommandInfo, out_dir: P) -> Result<()>
  where
    P: AsRef<Path>;

  fn generate_all<I, P>(cmds: I, out_dir: P) -> Result<()>
  where
    I: IntoIterator<Item = (String, CommandInfo)>,
    P: AsRef<Path>,
  {
    cmds.into_iter().try_for_each(|(cmd_name, cmd_info)| {
      <Self as Completions>::generate(cmd_name, cmd_info, &out_dir)
    })
  }
}
