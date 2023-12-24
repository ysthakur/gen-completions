mod bash;
mod kdl;
mod nu;
mod util;
mod zsh;

use std::{fs, path::Path};

use clap::ValueEnum;

use self::kdl::to_kdl_node;
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

/// Generate completion for the given shell and write to a file
///
/// # Errors
///
/// Fails if it can't write to a file, or if serde can't serialize the command
/// info (the second case should never happen).
pub fn generate_to_file(
  cmd: &CommandInfo,
  format: OutputFormat,
  out_dir: impl AsRef<Path>,
) -> std::io::Result<()> {
  let out_dir = out_dir.as_ref();
  let (file_name, text) = generate(cmd, format);
  fs::write(out_dir.join(file_name), text)
}

/// Generate completion for the given shell as a string
pub fn generate_to_str(cmd: &CommandInfo, format: OutputFormat) -> String {
  let (_, text) = generate(cmd, format);
  text
}

fn generate(cmd: &CommandInfo, format: OutputFormat) -> (String, String) {
  match format {
    OutputFormat::Bash => bash::generate(cmd),
    OutputFormat::Zsh => zsh::generate(cmd),
    OutputFormat::Nu => nu::generate(cmd),
    OutputFormat::Kdl => {
      (format!("{}.kdl", cmd.name), to_kdl_node(cmd).to_string())
    }
    OutputFormat::Json => (
      format!("{}.json", cmd.name),
      serde_json::to_string(cmd).unwrap(),
    ),
    OutputFormat::Yaml => (
      format!("{}.yaml", cmd.name),
      serde_yaml::to_string(cmd).unwrap(),
    ),
  }
}
