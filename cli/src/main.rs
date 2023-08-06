use std::{collections::HashMap, path::PathBuf};

use clap::{Parser, ValueEnum};
use man_completions::{
  gen::{zsh::ZshCompletions, Completions},
  get_manpath,
  parse::{parse_manpage_at_path, CommandInfo},
  parse_all_manpages, Error, Result,
};
use std::path::Path;

#[derive(Debug, Clone, ValueEnum)]
enum Shell {
  Zsh,
}

/// Generate completions from manpages
#[derive(Debug, Parser)]
#[command(version, about, long_about)]
struct CLI {
  /// Shell to generate completions for
  #[arg(short, long)]
  shell: Shell,

  /// Directory to output completions to
  #[arg(short, long)]
  out: PathBuf,

  /// Directories to exclude from search
  #[arg(short = 'D', long, value_delimiter = ',')]
  dirs_exclude: Option<Vec<PathBuf>>,

  /// Manpage sections to exclude (1-8)
  #[arg(short = 'S', long, value_parser = section_num_parser, value_delimiter = ',')]
  sections_exclude: Option<Vec<u8>>,

  /// A particular command to generate completions for. If omitted, generates
  /// completions for all found commands.
  cmd: Option<String>,
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

fn gen_shell(shell: Shell, manpages: HashMap<String, CommandInfo>, out_dir: &Path) {
  match shell {
    Shell::Zsh => <ZshCompletions as Completions>::generate_all(manpages.into_iter(), out_dir),
  }
}

fn main() -> Result<()> {
  let args = CLI::parse();

  println!("{:?}", &args);
  match get_manpath() {
    Some(manpath) => {
      let exclude_dirs = args.dirs_exclude.unwrap_or_default();
      // These directories we can search in
      let included: Vec<PathBuf> = manpath
        .into_iter()
        .filter(|path| !exclude_dirs.contains(path))
        .collect();

      if let Some(cmd) = &args.cmd {
        let manpage = man_completions::find_manpage(cmd, included)?;
        let parsed = parse_manpage_at_path(cmd, manpage)?;
        let mut map = HashMap::new();
        map.insert(cmd.to_string(), parsed);
        gen_shell(args.shell, map, &args.out);
        Ok(())
      } else {
        let all_manpages = man_completions::enumerate_manpages(included, args.sections_exclude);
        let all_parsed = parse_all_manpages(all_manpages);
        gen_shell(args.shell, all_parsed, &args.out);
        Ok(())
      }
    }
    None => Err(Error::NoManPages),
  }
}
