mod gen;
mod parse;

use std::{
  path::{Path, PathBuf},
  process::Command,
};

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use log::{debug, error, info, warn};
use parse::{detect_subcommands, parse_from};
use regex::Regex;

use crate::{
  gen::{
    BashCompletions, Completions, JsonCompletions, NuCompletions,
    ZshCompletions,
  },
  parse::{get_cmd_name, CommandInfo},
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

  /// Shell(s) to generate completions for
  #[arg(short, long, value_delimiter = ',', required = true)]
  shells: Vec<Shell>,

  /// Directories to search for man pages in, e.g.
  /// `--dirs=/usr/share/man/man1,/usr/share/man/man6`.
  #[arg(short, long, value_delimiter = ',')]
  dirs: Option<Vec<PathBuf>>,

  /// Particular commands to generate completions for (regex). If omitted,
  /// generates completions for all found commands. If you want to match the
  /// whole name, use `^...$`.
  #[arg(short, long, value_name = "REGEX")]
  cmds: Option<Regex>,

  /// Commands to exclude (regex). If you want to match the whole name, use
  /// `^...$`.
  #[arg(short = 'C', long, value_name = "REGEX")]
  exclude_cmds: Option<Regex>,

  /// Commands that should not be treated as subcommands. This is to help deal
  /// with false positives when detecting subcommands.
  #[arg(long, value_name = "CMD_NAMES", value_delimiter = ',')]
  not_subcmds: Vec<String>,

  /// Explicitly tell man-completions which man pages are for which
  /// subcommands, in case it can't detect them. e.g. `git-commit=git
  /// commit,foobar=foo bar`.
  #[arg(long, value_name = "MAN-PAGE=SUB CMD...", value_parser=subcmd_map_parser, value_delimiter = ',')]
  subcmds: Vec<(String, Vec<String>)>,
}

fn subcmd_map_parser(
  s: &str,
) -> core::result::Result<(String, Vec<String>), String> {
  let Some((page_name, as_subcmd)) = s.split_once('=') else {
    return Err(String::from(
      "subcommand mapping should be in the form 'manpage-name=sub command'",
    ));
  };
  let as_subcmd = as_subcmd.split(' ').map(String::from).collect();
  Ok((String::from(page_name), as_subcmd))
}

fn gen_shell(
  shell: &Shell,
  cmd_name: &str,
  parsed: &CommandInfo,
  out_dir: &Path,
) -> Result<()> {
  match shell {
    Shell::Zsh => {
      <ZshCompletions as Completions>::generate(cmd_name, parsed, out_dir)
    }
    Shell::Json => {
      <JsonCompletions as Completions>::generate(cmd_name, parsed, out_dir)
    }
    Shell::Bash => {
      <BashCompletions as Completions>::generate(cmd_name, parsed, out_dir)
    }
    Shell::Nu => {
      <NuCompletions as Completions>::generate(cmd_name, parsed, out_dir)
    }
  }
}

fn main() -> Result<()> {
  env_logger::init();

  let args = Cli::parse();

  let search_dirs = match args.dirs {
    Some(dirs) => dirs.into_iter().collect::<Vec<_>>(),
    None => enumerate_dirs(get_manpath()?),
  };

  let manpages = enumerate_manpages(search_dirs, args.cmds, args.exclude_cmds);

  let all_cmds = detect_subcommands(manpages, args.subcmds);
  let total = all_cmds.len();
  for (i, (cmd_name, cmd_info)) in all_cmds.into_iter().enumerate() {
    info!("Parsing '{}' ({}/{})", cmd_name, i + 1, total);

    let (res, errors) = parse_from(&cmd_name, cmd_info);

    for error in errors {
      error!("{}", error);
    }

    for shell in &args.shells {
      info!("Generating completions for '{}' ({:?})", cmd_name, &shell);
      gen_shell(shell, &cmd_name, &res, &args.out)?;
    }
  }

  Ok(())
}

/// Find the search path for man by `manpath`, then `man --path`.
fn get_manpath() -> Result<Vec<PathBuf>> {
  if let Ok(manpath) = std::env::var("MANPATH") {
    Ok(split_paths(&manpath))
  } else {
    debug!("Running 'manpath' to find MANPATH...");
    if let Some(manpath) = from_cmd(&mut Command::new("manpath")) {
      Ok(manpath)
    } else {
      warn!("Could not get path from 'manpath'. Trying 'man --path'");
      if let Some(manpath) = from_cmd(Command::new("man").arg("--path")) {
        Ok(manpath)
      } else {
        error!("Could not get path from 'man --path'");
        Err(anyhow!("Please provide either the --dirs flag or set the MANPATH environment variable."))
      }
    }
  }
}

/// Interpret the output of `manpath`/`man --path` as a list of paths
fn from_cmd(cmd: &mut Command) -> Option<Vec<PathBuf>> {
  cmd
    .output()
    .ok()
    .map(|out| split_paths(std::str::from_utf8(&out.stdout).unwrap()))
}

fn split_paths(paths: &str) -> Vec<PathBuf> {
  paths.split(':').map(PathBuf::from).collect()
}

/// Enumerate all directories containing manpages given the MANPATH (the list of
/// directories in which man search for man pages). It looks for `man1`, `man2`,
/// etc. folders inside each of the given directories and returns those inner
/// `man<n>` folders.
fn enumerate_dirs(manpath: Vec<PathBuf>) -> Vec<PathBuf> {
  let section_names: Vec<_> = (1..=8).map(|n| format!("man{n}")).collect();

  let mut res = Vec::new();

  for parent_path in manpath {
    if parent_path.is_dir() {
      if let Ok(parent_path) = std::fs::canonicalize(parent_path) {
        for section_name in &section_names {
          res.push(parent_path.join(section_name));
        }
      }
    }
  }

  res
}

/// Enumerate all directories containing manpages given the MANPATH (the list of
/// directories in which man search for man pages). It looks for `man1`, `man2`,
/// etc. folders inside each of the given directories and returns those inner
/// `man<n>` folders.
fn enumerate_manpages(
  dirs: Vec<PathBuf>,
  include_re: Option<Regex>,
  exclude_re: Option<Regex>,
) -> Vec<PathBuf> {
  let mut res = Vec::new();
  for dir in dirs {
    if let Ok(manpages) = std::fs::read_dir(dir) {
      for manpage in manpages.flatten() {
        let path = manpage.path();
        let cmd_name = get_cmd_name(&path);
        let &include = &include_re
          .as_ref()
          .map(|re| re.is_match(&cmd_name))
          .unwrap_or(true);
        let &exclude = &exclude_re
          .as_ref()
          .map(|re| re.is_match(&cmd_name))
          .unwrap_or(false);
        if include && exclude && include_re.is_some() {
          warn!("Command {} was both included and excluded explicitly, will exclude", cmd_name)
        }
        if include && !exclude {
          res.push(path)
        }
      }
    }
  }

  res
}
