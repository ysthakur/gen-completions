use crate::gen::{util::Output, CommandInfo};

/// Generate a completion file for Bash
pub fn generate(cmd: &CommandInfo) -> (String, String) {
  let comp_name = format!("_comp_cmd_{}", cmd.name);

  let mut out = Output::new(String::from("\t"));
  out.writeln("#!/usr/bin/env bash\n");
  out.writeln(format!("function {comp_name} {{"));
  out.indent();
  out.writeln("COMPREPLY=()");

  generate_cmd(cmd, 1, &mut out);

  out.writeln("return 0");
  out.dedent();
  out.writeln("}");
  out.writeln("");

  out.writeln(format!("complete -F _comp_cmd_{} {}", cmd.name, cmd.name));
  out.writeln("");

  (format!("_{}.bash", cmd.name), out.text())
}

fn generate_cmd(cmd: &CommandInfo, pos: usize, out: &mut Output) {
  out.writeln("case $COMP_CWORD in");
  out.indent();

  let flags = cmd
    .flags
    .iter()
    .map(|f| f.forms.join(" "))
    .collect::<Vec<_>>()
    .join(" ");
  let subcmds = cmd
    .subcommands
    .iter()
    .map(|c| c.name.to_string())
    .collect::<Vec<_>>()
    .join(" ");
  let completions = if flags.is_empty() {
    subcmds
  } else if subcmds.is_empty() {
    flags
  } else {
    format!("{flags} {subcmds}")
  };
  // This case is for when the subcommand we're processing is the one to
  // complete
  out.writeln(format!(
    "{pos}) COMPREPLY=($(compgen -W '{completions}' -- $2)) ;;"
  ));

  // This case is in case we need to go further to a deeper subcommand
  if !cmd.subcommands.is_empty() {
    out.writeln("*)");
    out.indent();
    out.writeln(format!("case ${{COMP_WORDS[{pos}]}} in"));
    out.indent();
    for sub_cmd in &cmd.subcommands {
      out.writeln(format!("{})", sub_cmd.name));
      out.indent();
      generate_cmd(sub_cmd, pos + 1, out);
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
