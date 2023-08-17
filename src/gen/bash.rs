use std::{fs, path::Path};

use anyhow::Result;

use crate::{gen::util::Output, parse::CommandInfo};

/// Generate a completion file for Bash
pub fn generate(
  cmd_name: &str,
  cmd_info: &CommandInfo,
  out_dir: &Path,
) -> Result<()> {
  let comp_name = format!("_comp_cmd_{cmd_name}");

  let mut out = Output::new(String::from("\t"));
  out.writeln("#!/usr/bin/env bash\n");
  out.writeln(format!("function {comp_name} {{"));
  out.indent();
  out.writeln("COMPREPLY=()");

  generate_cmd(cmd_info, 1, &mut out);

  out.writeln("return 0");
  out.dedent();
  out.writeln("}\n");

  out.writeln(format!("complete -F _comp_cmd_{cmd_name} {cmd_name}"));

  fs::write(out_dir.join(format!("_{cmd_name}.bash")), out.text())?;
  Ok(())
}

fn generate_cmd(cmd_info: &CommandInfo, pos: usize, out: &mut Output) {
  out.writeln("case $COMP_CWORD in");
  out.indent();

  let flags = cmd_info
    .flags
    .iter()
    .map(|f| f.forms.join(" "))
    .collect::<Vec<_>>()
    .join(" ");
  let subcmds = cmd_info
    .subcommands
    .keys()
    .map(String::from)
    .collect::<Vec<_>>()
    .join(" ");
  let completions = format!("{flags} {subcmds}");
  // This case is for when the subcommand we're processing is the one to
  // complete
  out.writeln(format!(
    "{pos}) COMPREPLY=($(compgen -W '{completions}' -- $2)) ;;"
  ));

  // This case is in case we need to go further to a deeper subcommand
  if !cmd_info.subcommands.is_empty() {
    out.writeln("*)");
    out.indent();
    out.writeln(format!("case ${{COMP_WORDS[{pos}]}} in"));
    out.indent();
    for (cmd_name, cmd_info) in &cmd_info.subcommands {
      out.writeln(format!("{cmd_name})"));
      out.indent();
      generate_cmd(cmd_info, pos + 1, out);
      out.writeln(";;");
      out.dedent();
    }
    out.dedent();
    out.writeln("esac");
    out.writeln(";;");
    out.dedent();
  }

  out.dedent();
  out.writeln("esac");
}
