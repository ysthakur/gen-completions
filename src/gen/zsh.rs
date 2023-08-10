use std::{fs, path::Path};

use anyhow::Result;

use crate::{
  gen::{util, Completions},
  parse::CommandInfo,
};

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
  out.push('\n');
  out.push_str(&format!("function {fn_name} {{\n"));
  if !cmd_info.subcommands.is_empty() {
    out.push_str(&format!("{}{}", INDENT, "local line\n"));
  }
  if cmd_info.subcommands.is_empty() {
    out.push_str(&format!("{INDENT}_arguments"));
  } else {
    out.push_str(&format!("{INDENT}_arguments -C"));
  }
  for opt in cmd_info.args {
    let desc = opt.desc.unwrap_or_default();
    for form in opt.forms {
      let text = util::quote(&format!("{form}[{}]", desc));
      out.push_str(" \\\n");
      out.push_str(&format!("{INDENT}{INDENT}{text}"));
    }
  }

  if !cmd_info.subcommands.is_empty() {
    let sub_cmds = cmd_info
      .subcommands
      .keys()
      .map(|s| s.to_string())
      .collect::<Vec<_>>()
      .join(" ");
    out.push_str(&format!(" \\\n{INDENT}{INDENT}': :({sub_cmds})'"));
    out.push_str(&format!(" \\\n{INDENT}{INDENT}'*::arg:->args'\n"));

    out.push_str(&format!("{INDENT}case $line[{}] in\n", pos + 1));
    for sub_cmd in cmd_info.subcommands.keys() {
      out.push_str(&format!(
        "{INDENT}{INDENT}{sub_cmd}) {}_{};;\n",
        fn_name, sub_cmd
      ))
    }
    out.push_str(&format!("{INDENT}esac\n"));
  } else {
    out.push('\n');
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
