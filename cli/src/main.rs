use std::{collections::HashMap, path::PathBuf};

use clap::{Parser, ValueEnum};
use man_completions::{
  gen::{zsh::ZshCompletions, Completions},
  get_manpath,
  parse::{parse_manpage_at_path, CommandInfo},
  parse_all_manpages, Error, Result,
};
use std::path::Path;

#[derive(Clone, ValueEnum)]
enum Shell {
  Zsh,
}

#[derive(Parser)]
struct CLI {
  #[arg(short, long)]
  exclude: Option<Vec<PathBuf>>,
  #[arg(short, long)]
  out: PathBuf,
  #[arg(short, long)]
  shell: Shell,
  /// A particular command to generate completions for
  cmd: Option<String>,
}

fn gen_shell(shell: Shell, manpages: HashMap<String, CommandInfo>, out_dir: &Path) {
  match shell {
    Shell::Zsh => <ZshCompletions as Completions>::generate_all(manpages.into_iter(), out_dir),
  }
}

fn main() -> Result<()> {
  let args = CLI::parse();

  match get_manpath() {
    Some(manpath) => {
      let excluded = args.exclude.unwrap_or_default();
      // These directories we can search in
      let included: Vec<PathBuf> = manpath
        .into_iter()
        .filter(|path| !excluded.contains(path))
        .collect();

      if let Some(cmd) = &args.cmd {
        let manpage = man_completions::find_manpage(cmd, included)?;
        let parsed = parse_manpage_at_path(cmd, manpage)?;
        let mut map = HashMap::new();
        map.insert(cmd.to_string(), parsed);
        gen_shell(args.shell, map, &args.out);
        Ok(())
      } else {
        let all_manpages = man_completions::enumerate_manpages(included);
        let all_parsed = parse_all_manpages(all_manpages);
        gen_shell(args.shell, all_parsed, &args.out);
        Ok(())
      }
    }
    None => Err(Error::NoManPages),
  }
}
