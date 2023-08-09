use std::{fs, path::Path};

use crate::parse::CommandInfo;

use super::Completions;
use anyhow::Result;

pub struct BashCompletions;

impl Completions for BashCompletions {
  /// Generate a completion file for Bash
  fn generate<P>(cmd_name: String, _cmd_info: CommandInfo, out_dir: P) -> Result<()>
  where
    P: AsRef<Path>,
  {
    // TODO make option to not overwrite file
    let comp_name = format!("_comp_cmd_{cmd_name}");

    let mut res = String::from("#!/usr/bin/env bash\n\n");
    res.push_str(&format!("function {comp_name} {{\n"));
    res.push_str("\tCOMPREPLY=()\n");
    res.push_str("\tcase ${COMP_CWORD} in\n");
    // generate_fn(&cmd_name, cmd_info, &mut res, 0, &comp_name);
    res.push_str("\tesac\n");
    res.push_str("\treturn 0\n");
    res.push_str("}\n");

    fs::write(out_dir.as_ref().join(format!("_{cmd_name}.bash")), res)?;
    Ok(())
  }
}
