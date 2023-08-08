mod gen;
mod parse;

use std::{collections::HashMap, path::PathBuf};

use crate::{
  gen::{Completions, JsonCompletions, ZshCompletions},
  parse::{CommandInfo, ManParseConfig},
};
use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::path::Path;

#[derive(Debug, Clone, ValueEnum)]
enum Shell {
  /// Generate completions for Zsh
  Zsh,
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

  /// Turn on verbose output
  #[arg(short, long)]
  verbose: bool,

  /// Search for subcommands
  /// TODO implement
  #[arg(short = 's', long = "subcommands")]
  search_subcommands: bool,

  /// Directories to exclude from search
  #[arg(short = 'D', long, value_delimiter = ',')]
  dirs_exclude: Option<Vec<PathBuf>>,

  /// Manpage sections to exclude (1-8)
  #[arg(short = 'S', long, value_parser = section_num_parser, value_delimiter = ',')]
  sections_exclude: Vec<u8>,

  /// A particular command to generate completions for. If omitted, generates
  /// completions for all found commands.
  #[arg(short, long)]
  cmd: Option<String>,

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
  }
}

fn main() -> Result<()> {
  let args = CLI::parse();

  let mut cfg = ManParseConfig::new()
    .exclude_dirs(args.dirs_exclude.unwrap_or_default())
    .exclude_sections(args.sections_exclude);
  if let Some(cmd) = args.cmd {
    cfg = cfg.restrict_to_commands(vec![cmd]);
  }
  if args.search_subcommands {
    cfg = cfg.search_subcommands();
  }

  let res = cfg.parse()?;
  gen_shell(args.shell, res, &args.out)?;
  Ok(())
}
