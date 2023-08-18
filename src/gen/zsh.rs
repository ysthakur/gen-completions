use std::{fs, path::Path};

use anyhow::Result;

use super::util::Output;
use crate::{gen::util, parse::CommandInfo};

/// Generate a completion file for Zsh
///
/// A shortened example with git
/// ```
/// #compdef _git git
///
/// function _git {
///     local line
///
///     _argument -C \
///         '-h[Show help]' \
///         '--help[Show help]' \
///         ': :(pull checkout)' \ # Assume only git pull and checkout exist
///         '*::args->args'
///
///     case $line[1] in
///         pull) _git_pull;;
///         checkout) _git_checkout;;
///     esac
/// }
///
/// function _git_pull {
///     _arguments \
///         '-v[Output additional information]'
/// }
///
/// function _git_checkout {
///     _arguments \
///         '-b[Make new branch]'
/// }
/// ```
pub fn generate(cmd: &CommandInfo, out_dir: &Path) -> Result<()> {
  // TODO make option to not overwrite file
  let comp_name = format!("_{}", cmd.name);
  let mut res = Output::new(String::from("\t"));
  res.writeln(format!("#compdef {comp_name} {}", cmd.name));
  generate_fn(cmd, &mut res, 0, &comp_name);
  fs::write(out_dir.join(format!("{comp_name}.zsh")), res.text())?;
  Ok(())
}

/// Generate a completion function for a command/subcommand
///
/// ## Arguments
/// * `pos` - If this is a top-level command, 0. Otherwise, if this is a
///   subcommand, which argument number the subcommand is (how deep it is)
/// * `fn` - What to name the completion function. If you have a command `foo`
///   with subcommand `bar`, the completion function for `foo bar` would be
///   named `_foo_bar`
fn generate_fn(cmd: &CommandInfo, out: &mut Output, pos: usize, fn_name: &str) {
  out.writeln("");
  out.writeln(format!("function {fn_name} {{"));
  out.indent();

  if !cmd.subcommands.is_empty() {
    out.writeln("local line");
  }
  if cmd.subcommands.is_empty() {
    out.write("_arguments");
  } else {
    out.write("_arguments -C");
  }

  out.indent();
  for flag in &cmd.flags {
    let desc = if let Some(desc) = &flag.desc {
      desc
    } else {
      ""
    };
    for form in &flag.forms {
      let text = util::quote_bash(format!("{form}[{desc}]"));
      out.writeln(" \\");
      out.write(text);
    }
  }

  if cmd.subcommands.is_empty() {
    out.dedent();
    out.write("\n");
  } else {
    let sub_cmds = cmd
      .subcommands
      .iter()
      .map(|c| c.name.to_string())
      .collect::<Vec<_>>()
      .join(" ");
    out.writeln(" \\");
    out.writeln(format!("': :({sub_cmds})' \\"));
    out.writeln("'*::arg:->args'");
    out.dedent();

    out.writeln(format!("case $line[{}] in", pos + 1));
    out.indent();
    for sub_cmd in &cmd.subcommands {
      out.writeln(format!("{}) {fn_name}_{};;", sub_cmd.name, sub_cmd.name));
    }
    out.dedent();
    out.writeln("esac");
  }

  out.dedent();
  out.writeln("}");

  for sub_cmd in &cmd.subcommands {
    generate_fn(
      sub_cmd,
      out,
      pos + 1,
      &format!("{fn_name}_{}", sub_cmd.name),
    );
  }
}
