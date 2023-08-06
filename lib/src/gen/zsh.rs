use std::path::Path;

use crate::parse::CommandInfo;

use super::Completions;
use crate::Result;

pub struct ZshCompletions;

impl Completions for ZshCompletions {
  fn generate<P>(cmd_name: String, cmd_info: CommandInfo, out_dir: P) -> Result<()>
  where
    P: AsRef<Path>,
  {
    todo!()
  }
}
