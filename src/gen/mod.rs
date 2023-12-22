mod bash;
mod nu;
mod util;
mod zsh;

use std::{fs, io, path::Path};

use clap::ValueEnum;

use crate::CommandInfo;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum OutputFormat {
  /// Generate completions for Zsh
  Zsh,
  /// Generate completions for Bash
  Bash,
  /// Generate completions for Nushell
  Nu,
  /// Output parsed options as KDL
  Kdl,
  /// Output parsed options as JSON
  Json,
  /// Output parsed options as YAML
  Yaml,
}

pub fn generate(
  cmd: &CommandInfo,
  format: OutputFormat,
  out_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
  let out_dir = out_dir.as_ref();
  match format {
    OutputFormat::Bash => bash::generate(cmd, out_dir)?,
    OutputFormat::Zsh => zsh::generate(cmd, out_dir)?,
    OutputFormat::Nu => nu::generate(cmd, out_dir)?,
    OutputFormat::Kdl => todo!(),
    OutputFormat::Json => write(
      out_dir.join(format!("{}.json", cmd.name)),
      &serde_json::to_string(cmd)?,
    )?,
    OutputFormat::Yaml => write(
      out_dir.join(format!("{}.yaml", cmd.name)),
      &serde_yaml::to_string(cmd)?,
    )?,
  };
  Ok(())
}

fn write(out_dir: impl AsRef<Path>, text: &str) -> io::Result<()> {
  fs::write(out_dir, text)
}
