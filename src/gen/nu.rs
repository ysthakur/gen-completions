use std::{fs, path::Path};

use crate::gen::{util::Output, CommandInfo};

/// Generate completions for Nushell
pub fn generate(cmd: &CommandInfo, out_dir: &Path) -> std::io::Result<()> {
  let mut res = Output::new(String::from("  "));
  generate_cmd(&cmd.name, cmd, &mut res, true);
  fs::write(
    out_dir.join(format!("{}-completions.nu", cmd.name)),
    res.text(),
  )?;
  Ok(())
}

fn generate_cmd(
  cmd_name: &str,
  cmd: &CommandInfo,
  out: &mut Output,
  first: bool,
) {
  if !first {
    // Avoid an extra line at the beginning of the file
    out.writeln("");
  }
  out.writeln(format!("export extern \"{cmd_name}\" ["));
  out.indent();

  for flag in &cmd.flags {
    let (short, long): (Vec<_>, Vec<_>) =
      flag.forms.iter().partition(|f| f.len() == 2);

    let desc_str = if let Some(desc) = &flag.desc {
      format!(" # {desc}")
    } else {
      String::new()
    };

    // Pair off as many long and short forms as possible
    // It's unlikely there'll be both long and short forms of the same flag, but
    // you never know what kind of horrors a man page may hold
    let mut short = short.into_iter();
    let mut long = long.into_iter();
    while short.len() > 0 && long.len() > 0 {
      let short_str = format!("({})", short.next().unwrap());
      out.writeln(format!("{}{}{}", long.next().unwrap(), short_str, desc_str));
    }

    for flag in long {
      out.writeln(format!("{flag}{desc_str}"));
    }

    for flag in short {
      out.writeln(format!("{flag}{desc_str}"));
    }
  }

  out.dedent();
  out.writeln("]");

  for sub_cmd in &cmd.subcommands {
    generate_cmd(&format!("{cmd_name} {}", sub_cmd.name), sub_cmd, out, false);
  }
}
