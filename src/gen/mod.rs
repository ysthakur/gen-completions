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
  fn generate<P>(
    cmd_name: &str,
    cmd_info: &CommandInfo,
    out_dir: P,
  ) -> Result<()>
  where
    P: AsRef<Path>;
}
