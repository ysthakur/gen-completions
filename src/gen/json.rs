use std::{fs, path::Path};

use anyhow::Result;

use crate::{gen::util::Output, parse::CommandInfo};

/// Generate JSON representing the parsed options
///
/// This should probably use a real JSON library but whatever
pub fn generate(
  cmd_name: &str,
  cmd_info: &CommandInfo,
  out_dir: &Path,
) -> Result<()> {
  let mut res = Output::new(String::from("  "));
  res.writeln("{");
  res.indent();
  generate_cmd(cmd_name, cmd_info, true, &mut res);
  res.dedent();
  res.writeln("}");
  fs::write(out_dir.join(format!("{cmd_name}.json")), res.text())?;
  Ok(())
}

/// Helper to generate JSON
///
/// ## Arguments
/// * `indent` - The indentation level (how many subcommands in we are)
/// * `last` - Whether this is the last command at this level. Used for deciding
///   whether or not to put a trailing comma
fn generate_cmd(
  cmd: &str,
  cmd_info: &CommandInfo,
  last: bool,
  out: &mut Output,
) {
  let cmd = quote(cmd);
  // Avoid trailing commas
  let end = if last { "}" } else { "}," };
  let mut flags = cmd_info.flags.iter();
  if let Some(mut flag) = flags.next() {
    out.writeln(format!("{cmd}: {{"));
    out.indent();
    out.writeln("\"flags\": [");
    out.indent();

    loop {
      out.writeln("{");
      out.indent();

      let forms = flag
        .forms
        .iter()
        .map(|a| quote(a))
        .collect::<Vec<_>>()
        .join(", ");
      out.write(format!("\"forms\": [{forms}]"));
      if let Some(desc) = &flag.desc {
        out.writeln(",");
        out.writeln(format!("\"description\": {}", quote(desc)));
      } else {
        out.writeln("");
      }

      out.dedent();
      if let Some(next) = flags.next() {
        out.writeln("},");
        flag = next;
      } else {
        // Avoid trailing comma
        out.writeln("}");
        break;
      }
    }

    out.dedent();
    out.writeln("],");

    let mut subcmds = cmd_info.subcommands.iter();
    if let Some((mut name, mut info)) = subcmds.next() {
      out.writeln("\"subcommands\": {");
      out.indent();
      loop {
        if let Some(next) = subcmds.next() {
          generate_cmd(name, info, false, out);
          name = next.0;
          info = next.1;
        } else {
          generate_cmd(name, info, true, out);
          break;
        }
      }
      out.dedent();
      out.writeln("}");
    } else {
      out.writeln("\"subcommands\": {}");
    }

    out.dedent();
    out.writeln(end);
  } else {
    // If no arguments, print `"cmd": {}` on a single line
    out.writeln(format!("{cmd}: {{{end}"));
  }
}

fn quote(s: &str) -> String {
  format!("\"{}\"", s.replace('\\', r"\\").replace('"', "\\\""))
}
