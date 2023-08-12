use std::{fs, path::Path};

use anyhow::Result;

use crate::{gen::util::Output, parse::CommandInfo};

/// Generate a completion file for Bash
pub fn generate(
  cmd_name: &str,
  _cmd_info: &CommandInfo,
  out_dir: &Path,
) -> Result<()> {
  // TODO make option to not overwrite file
  let comp_name = format!("_comp_cmd_{cmd_name}");

  let mut res = Output::new(String::from("\t"));
  res.writeln("#!/usr/bin/env bash\n");
  res.writeln(&format!("function {comp_name} {{"));
  res.writeln("COMPREPLY=()");
  res.writeln("case ${COMP_CWORD} in");
  // generate_fn(&cmd_name, cmd_info, &mut res, 0, &comp_name);
  res.writeln("esac");
  res.writeln("return 0");
  res.writeln("}");

  fs::write(out_dir.join(format!("_{cmd_name}.bash")), res.text())?;
  Ok(())
}
