use std::{fs, path::Path};

use anyhow::Result;

use crate::{gen::util::Output, parse::CommandInfo};

/// Generate completions for Nushell
pub fn generate(
  cmd_name: &str,
  cmd_info: &CommandInfo,
  out_dir: &Path,
) -> Result<()> {
  let mut res = Output::new(String::from("  "));
  generate_cmd(cmd_name, cmd_info, &mut res, true);
  fs::write(out_dir.join(format!("{}.nu", cmd_name)), res.text())?;
  Ok(())
}

fn generate_cmd(
  cmd_name: &str,
  cmd_info: &CommandInfo,
  out: &mut Output,
  first: bool,
) {
  if !first {
    // Avoid an extra line at the beginning of the file
    out.writeln("");
  }
  out.writeln(format!("export extern \"{}\" [", cmd_name));
  out.indent();

  for flag in cmd_info.flags.iter() {
    let (mut short, mut long): (Vec<_>, Vec<_>) =
      flag.forms.iter().partition(|f| f.len() == 2);

    let desc_str = if let Some(desc) = &flag.desc {
      format!(" # {}", desc)
    } else {
      String::from("")
    };

    // Pair off as many long and short forms as possible
    // It's unlikely there'll be both long and short forms of the same flag, but
    // you never know what kind of horrors a man page may hold
    while !long.is_empty() && !short.is_empty() {
      let short_str = format!("({})", short.pop().unwrap());
      out.writeln(format!("{}{}{}", long.pop().unwrap(), short_str, desc_str));
    }

    while let Some(flag) = long.pop() {
      out.writeln(format!("{}{}", flag, desc_str));
    }

    while let Some(flag) = short.pop() {
      out.writeln(format!("{}{}", flag, desc_str));
    }
  }

  out.dedent();
  out.writeln("]");

  for (subname, subcmd) in &cmd_info.subcommands {
    generate_cmd(&format!("{} {}", cmd_name, subname), subcmd, out, false);
  }
}
