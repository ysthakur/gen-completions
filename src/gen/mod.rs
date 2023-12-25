mod bash;
mod kdl;
mod nu;
mod util;
mod zsh;

use std::{fs, path::Path};

use clap::ValueEnum;

use self::kdl::to_kdl_node;
use crate::{CommandInfo, Flag};

/// Maximum length of a description
///
/// After this, `...` will be added
static MAX_DESC_LEN: usize = 80;
static ELLIPSIS: &str = "...";

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
#[must_use]
pub fn generate_to_str(cmd: &CommandInfo, format: OutputFormat) -> String {
  let (_, text) = generate(cmd, format);
  text
}

fn generate(cmd: &CommandInfo, format: OutputFormat) -> (String, String) {
  let cmd = preprocess(cmd);
  match format {
    OutputFormat::Bash => bash::generate(&cmd),
    OutputFormat::Zsh => zsh::generate(&cmd),
    OutputFormat::Nu => nu::generate(&cmd),
    OutputFormat::Kdl => {
      (format!("{}.kdl", cmd.name), to_kdl_node(&cmd).to_string())
    }
    OutputFormat::Json => (
      format!("{}.json", cmd.name),
      serde_json::to_string(&cmd).unwrap(),
    ),
    OutputFormat::Yaml => (
      format!("{}.yaml", cmd.name),
      serde_yaml::to_string(&cmd).unwrap(),
    ),
  }
}

/// Trim descriptions
/// todo pass the max description length as an option
/// possibly have each generator do the trimming separately
fn preprocess(cmd: &CommandInfo) -> CommandInfo {
  let flags: Vec<Flag> = cmd
    .flags
    .iter()
    .map(|flag| {
      // TODO port the sentence-splitting part too
      // https://github.com/fish-shell/fish-shell/blob/master/share/tools/create_manpage_completions.py#L211
      let desc = flag.desc.as_ref().map(|desc| {
        if desc.len() > MAX_DESC_LEN {
          format!("{}{}", &desc[0..MAX_DESC_LEN - ELLIPSIS.len()], ELLIPSIS)
        } else {
          desc.to_owned()
        }
      });
      Flag {
        forms: flag.forms.clone(),
        desc,
        typ: flag.typ.clone(),
      }
    })
    .collect();
  CommandInfo {
    name: cmd.name.clone(),
    flags,
    args: cmd.args.clone(),
    subcommands: cmd.subcommands.iter().map(preprocess).collect(),
  }
}
