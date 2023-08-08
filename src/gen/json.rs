use std::{fs, path::Path};

use crate::parse::CommandInfo;

use super::Completions;
use anyhow::Result;

pub struct JsonCompletions;

impl Completions for JsonCompletions {
  fn generate<P>(cmd_name: String, cmd_info: CommandInfo, out_dir: P) -> Result<()>
  where
    P: AsRef<Path>,
  {
    Ok(())
  }
}
