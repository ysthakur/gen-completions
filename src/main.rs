mod gen;
mod parse;

use crate::{
  gen::{BashCompletions, Completions, JsonCompletions, ZshCompletions},
  parse::{CommandInfo, ManParseConfig},
};
use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, ValueEnum)]
enum Shell {
  /// Generate completions for Zsh
  Zsh,
  /// Generate completions for Bash
  Bash,
  /// Not a shell, but output the parsed options as JSON
  Json,
}

/// Generate completions from manpages
#[derive(Debug, Parser)]
#[command(version, about, long_about)]
struct CLI {
  /// Directory to output completions to
  #[arg(short, long)]
  out: PathBuf,

  /// Directories to exclude from search
  #[arg(short = 'i', long = "ignore", value_delimiter = ',')]
  dirs_exclude: Option<Vec<PathBuf>>,

  /// Manpage sections to exclude (1-8)
  #[arg(short = 'S', long, value_parser = section_num_parser, value_delimiter = ',')]
  sections_exclude: Vec<u8>,

  /// Particular commands to generate completions for. If omitted, generates
  /// completions for all found commands.
  #[arg(short, long)]
  cmds: Option<Regex>,

  /// Commands to exclude (regex).
  #[arg(short = 'C', long)]
  exclude_cmds: Option<Regex>,

  /// Commands that should not be treated as subcommands. This is to help deal
  /// with false positives when detecting subcommands.
  #[arg(short, long, value_delimiter = ',')]
  not_subcmds: Vec<String>,

  /// Shell to generate completions for
  shell: Shell,
}

fn section_num_parser(s: &str) -> core::result::Result<u8, String> {
  match str::parse::<u8>(s) {
    Ok(num) => {
      if 1 <= num && num <= 8 {
        Ok(num)
      } else {
        Err("must be between 1 and 8".to_string())
      }
    }
    _ => Err(format!("should be an int between 1 and 8")),
  }
}

fn gen_shell(
  shell: Shell,
  manpages: HashMap<String, CommandInfo>,
  out_dir: &Path,
) -> Result<()> {
  match shell {
    Shell::Zsh => <ZshCompletions as Completions>::generate_all(manpages, out_dir),
    Shell::Json => <JsonCompletions as Completions>::generate_all(manpages, out_dir),
    Shell::Bash => <BashCompletions as Completions>::generate_all(manpages, out_dir),
  }
}

fn main() -> Result<()> {
  env_logger::init();

  let args = CLI::parse();

  let mut cfg = ManParseConfig::new()
    .exclude_dirs(args.dirs_exclude.unwrap_or_default())
    .exclude_sections(args.sections_exclude)
    .not_subcommands(args.not_subcmds);
  if let Some(exclude_cmds) = args.exclude_cmds {
    cfg = cfg.exclude_commands(exclude_cmds);
  }

  if let Some(cmds) = args.cmds {
    cfg = cfg.restrict_to_commands(cmds);
  }

  let res = cfg.parse()?;
  gen_shell(args.shell, res, &args.out)?;
  Ok(())
}
