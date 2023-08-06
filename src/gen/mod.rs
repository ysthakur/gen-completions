use std::path::Path;

use crate::{parse::CommandInfo, Result};

pub mod zsh;

pub trait Completions {
  fn generate<P>(cmd_name: String, cmd_info: CommandInfo, out_dir: P) -> Result<()>
  where
    P: AsRef<Path>;

  fn generate_all<I, P>(cmds: I, out_dir: P) -> ()
  where
    I: Iterator<Item = (String, CommandInfo)>,
    P: AsRef<Path>,
  {
    for (cmd_name, cmd_info) in cmds {
      // TODO collect errors instead of discarding
      <Self as Completions>::generate(cmd_name, cmd_info, &out_dir);
    }
  }
}
