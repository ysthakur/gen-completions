use std::path::PathBuf;

use clap::Parser;
use man_completions::{get_manpath, parse_manpage, parse_all_manpages};

#[derive(Parser)]
struct CLI {
  #[arg(short = 'p', long)]
  manpath: Option<Vec<PathBuf>>,
  #[arg(short, long)]
  exclude: Option<Vec<PathBuf>>,
  /// A particular command to generate completions for
  cmd: Option<String>,
}

#[derive(Debug)]
struct Error(String);

fn main() -> Result<(), Error> {
  let args = CLI::parse();

  match args.manpath.or_else(get_manpath) {
    Some(mut manpath) => {
      let excluded = args.exclude.unwrap_or_default();
      // These directories we can search in
      let included: Vec<PathBuf> = manpath
        .into_iter()
        .filter(|path| !excluded.contains(path))
        .collect();

      if let Some(cmd) = &args.cmd {
        match man_completions::find_manpage(cmd, included) {
          Some(manpage) => {
            todo!()
          }
          None => Err(Error(format!("No manpage found for {}", cmd))),
        }
      } else {
        let all_manpages = man_completions::enumerate_manpages(included);
        let all_parsed = parse_all_manpages(all_manpages);
        todo!()
      }
    }
    None => Err(Error(
      "Unable to find manpages. Please use --manpath.".to_string(),
    )),
  }
}
