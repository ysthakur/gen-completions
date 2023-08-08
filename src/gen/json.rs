use std::{fs, path::Path};

use crate::parse::CommandInfo;

use super::Completions;
use anyhow::Result;

const INDENT: &str = "  ";

pub struct JsonCompletions;

impl Completions for JsonCompletions {
  /// Generate JSON representing the parsed options
  ///
  /// This should probably use a real JSON library but whatever
  fn generate<P>(cmd_name: String, cmd_info: CommandInfo, out_dir: P) -> Result<()>
  where
    P: AsRef<Path>,
  {
    let mut res = String::new();
    res.push_str("{\n");
    generate_cmd(&cmd_name, cmd_info, 1, true, &mut res);
    res.push_str("}\n");
    fs::write(out_dir.as_ref().join(format!("{cmd_name}.json")), res)?;
    Ok(())
  }
}

/// Helper to generate JSON
///
/// ## Arguments
/// * `indent` - The indentation level (how many subcommands in we are)
/// * `last` - Whether this is the last command at this level. Used for deciding
///   whether or not to put a trailing comma
fn generate_cmd(
  cmd: &str,
  cmd_info: CommandInfo,
  indent: usize,
  last: bool,
  out: &mut String,
) {
  let cmd = quote(cmd);
  // Avoid trailing commas
  let end = if last { "]" } else { "]," };
  let mut args = cmd_info.args.into_iter();
  if let Some(mut arg) = args.next() {
    println_indent(indent, out, format!("{cmd}: ["));
    while {
      println_indent(indent + 1, out, "{");
      let forms = arg
        .forms
        .iter()
        .map(|a| quote(&a))
        .collect::<Vec<_>>()
        .join(", ");
      print_indent(indent + 2, out, format!(r#""forms": [{forms}]"#));
      if let Some(desc) = &arg.desc {
        out.push_str(",\n");
        println_indent(
          indent + 2,
          out,
          format!(r#""description": {}"#, quote(desc)),
        );
      } else {
        out.push_str("\n");
      }
      if let Some(next) = args.next() {
        println_indent(indent + 1, out, "},");
        arg = next;
        true
      } else {
        // Avoid trailing comma
        println_indent(indent + 1, out, "}");
        false
      }
    } {}
    println_indent(indent, out, end);
  } else {
    // If no arguments, print `"cmd": []` on a single line
    println_indent(indent, out, format!("{cmd}: [{end}"))
  }
}

fn quote(s: &str) -> String {
  format!("\"{}\"", s.replace('\\', r"\\").replace('"', "\\\""))
}

/// Like print_indent, but with a newline
fn println_indent<S: AsRef<str>>(indent: usize, out: &mut String, text: S) {
  print_indent(indent, out, text);
  out.push_str("\n");
}

/// Helper to print at a specific indentation level with a newline
fn print_indent<S: AsRef<str>>(indent: usize, out: &mut String, text: S) {
  out.push_str(&INDENT.repeat(indent));
  out.push_str(text.as_ref());
}
