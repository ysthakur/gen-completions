mod gen;
mod parse;

use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;

use crate::{
  gen::{
    BashCompletions, Completions, JsonCompletions, NuCompletions,
    ZshCompletions,
  },
  parse::{CommandInfo, ManParseConfig},
};

#[derive(Debug, Clone, ValueEnum)]
enum Shell {
  /// Generate completions for Zsh
  Zsh,
  /// Generate completions for Bash
  Bash,
  /// Generate completions for Nushell
  Nu,
  /// Output parsed options as JSON
  Json,
}

/// Generate completions from manpages
#[derive(Debug, Parser)]
#[command(version, about, long_about)]
struct Cli {
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

  /// Shell(s) to generate completions for
  #[arg(short, long, value_delimiter = ',', required = true)]
  shells: Vec<Shell>,
}

fn section_num_parser(s: &str) -> core::result::Result<u8, String> {
  match str::parse::<u8>(s) {
    Ok(num) => {
      if (1..=8).contains(&num) {
        Ok(num)
      } else {
        Err(String::from("must be between 1 and 8"))
      }
    }
    _ => Err(String::from("should be an int between 1 and 8")),
  }
}

fn gen_shell(
  shell: Shell,
  manpages: &HashMap<String, CommandInfo>,
  out_dir: &Path,
) -> Result<()> {
  match shell {
    Shell::Zsh => {
      <ZshCompletions as Completions>::generate_all(manpages.iter(), out_dir)
    }
    Shell::Json => {
      <JsonCompletions as Completions>::generate_all(manpages.iter(), out_dir)
    }
    Shell::Bash => {
      <BashCompletions as Completions>::generate_all(manpages.iter(), out_dir)
    }
    Shell::Nu => {
      <NuCompletions as Completions>::generate_all(manpages.iter(), out_dir)
    }
  }
}

fn main() -> Result<()> {
  env_logger::init();

  let args = Cli::parse();

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

  for shell in args.shells {
    gen_shell(shell, &res, &args.out)?;
  }

  Ok(())
}
