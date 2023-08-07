use std::{fs, path::Path};

use crate::parse::CommandInfo;

use super::Completions;
use crate::Result;

/// Indentation to use (for readability)
const INDENT: &str = "    ";

pub struct ZshCompletions;

impl Completions for ZshCompletions {
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
  ///         ':(pull checkout)' \ # Assume only git pull and checkout exist
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
  fn generate<P>(cmd_name: String, cmd_info: CommandInfo, out_dir: P) -> Result<()>
  where
    P: AsRef<Path>,
  {
    // TODO make option to not overwrite file
    let comp_name = format!("_{cmd_name}");
    let mut res = format!("#compdef {comp_name} {cmd_name}\n");
    generate_fn(&cmd_name, cmd_info, &mut res, 0, &comp_name);
    fs::write(out_dir.as_ref().join(format!("{comp_name}.zsh")), res)?;
    Ok(())
  }
}

/// Wrap in single quotes (and escape single quotes inside) so that it's safe
/// for Zsh to read
fn quote(s: &str) -> String {
  format!("'{}'", s.replace("'", r#"'"'"'"#))
}

/// Generate a completion function for a command/subcommand
///
/// ## Arguments
/// * `pos` - If this is a top-level command, 0. Otherwise, if this is a
///   subcommand, which argument number the subcommand is (how deep it is)
/// * `fn` - What to name the completion function. If you have a command `foo`
///   with subcommand `bar`, the completion function for `foo bar` would be
///   named `_foo_bar`
fn generate_fn(
  _cmd_name: &str,
  cmd_info: CommandInfo,
  out: &mut String,
  pos: usize,
  fn_name: &str,
) {
  out.push_str("\n");
  out.push_str(&format!("function {fn_name} {{\n"));
  if !cmd_info.subcommands.is_empty() {
    out.push_str(&format!("{}{}", INDENT, "local line\n"));
  }
  out.push_str(&format!("{INDENT} _arguments -C \\\n"));
  for opt in cmd_info.args {
    for form in opt.forms {
      let text = quote(&format!("{form}[{}]", opt.desc));
      out.push_str(&format!("{INDENT}{INDENT}{text} \\\n"));
    }
  }

  if !cmd_info.subcommands.is_empty() {
    let mut sub_cmds = String::new();
    for sub_cmd in cmd_info.subcommands.keys() {
      sub_cmds.push_str(sub_cmd);
    }
    out.push_str(&format!("{INDENT}{INDENT}':({sub_cmds})' \\\n"))
  }

  out.push_str(&format!("{INDENT}{INDENT}'*::args->args'\n"));

  if !cmd_info.subcommands.is_empty() {
    out.push_str(&format!("{INDENT}case $line[{}] in\n", pos + 1));
    for _sub_cmd in cmd_info.subcommands.keys() {
      todo!()
    }
    out.push_str(&format!("{INDENT}esac\n"));
  }

  out.push_str("}\n");

  for (sub_cmd, sub_cmd_info) in cmd_info.subcommands {
    generate_fn(
      &sub_cmd,
      sub_cmd_info,
      out,
      pos + 1,
      &format!("{fn_name}_{sub_cmd}"),
    );
  }
}
