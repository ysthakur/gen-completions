use std::{fs, path::Path};

use crate::gen::{util::Output, CommandInfo};

/// Generate JSON representing the parsed options
///
/// This should probably use a real JSON library but whatever
pub fn generate(cmd: &CommandInfo, out_dir: &Path) -> std::io::Result<()> {
  let mut res = Output::new(String::from("  "));
  res.writeln("{");
  res.indent();
  generate_cmd(cmd, true, &mut res);
  res.dedent();
  res.writeln("}");
  fs::write(out_dir.join(format!("{}.json", cmd.name)), res.text())?;
  Ok(())
}

/// Helper to generate JSON
///
/// ## Arguments
/// * `indent` - The indentation level (how many subcommands in we are)
/// * `last` - Whether this is the last command at this level. Used for deciding
///   whether or not to put a trailing comma
fn generate_cmd(cmd: &CommandInfo, last: bool, out: &mut Output) {
  let cmd_name = quote(&cmd.name);
  // Avoid trailing commas
  let end = if last { "}" } else { "}," };
  let mut flags = cmd.flags.iter();
  if let Some(mut flag) = flags.next() {
    out.writeln(format!("{cmd_name}: {{"));
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

    let mut subcmds = cmd.subcommands.iter();
    if let Some(mut sub_cmd) = subcmds.next() {
      out.writeln("\"subcommands\": {");
      out.indent();

      for next in subcmds {
        generate_cmd(sub_cmd, false, out);
        sub_cmd = next;
      }
      generate_cmd(sub_cmd, true, out);

      out.dedent();
      out.writeln("}");
    } else {
      out.writeln("\"subcommands\": {}");
    }

    out.dedent();
    out.writeln(end);
  } else {
    // If no arguments, print `"cmd": {}` on a single line
    out.writeln(format!("{cmd_name}: {{{end}"));
  }
}

fn quote(s: &str) -> String {
  format!("\"{}\"", s.replace('\\', r"\\").replace('"', "\\\""))
}
