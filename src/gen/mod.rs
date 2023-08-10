mod bash;
mod json;
mod nu;
mod util;
mod zsh;

use std::path::Path;

use anyhow::Result;

pub use self::{bash::*, json::*, nu::*, zsh::*};
use crate::parse::CommandInfo;

pub trait Completions {
  fn generate<P>(cmd_name: &str, cmd_info: &CommandInfo, out_dir: P) -> Result<()>
  where
    P: AsRef<Path>;

  fn generate_all<'a, I, P>(cmds: I, out_dir: P) -> Result<()>
  where
    I: Iterator<Item = (&'a String, &'a CommandInfo)>,
    P: AsRef<Path>,
  {
    cmds.into_iter().try_for_each(|(cmd_name, cmd_info)| {
      <Self as Completions>::generate(cmd_name, cmd_info, &out_dir)
    })
  }
}
