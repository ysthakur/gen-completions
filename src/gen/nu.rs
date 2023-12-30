use regex::Regex;

use crate::{
  gen::{util::Output, CommandInfo},
  ArgType,
};

/// Generate completions for Nushell
pub fn generate(cmd: &CommandInfo) -> (String, String) {
  let mut res = Output::new(String::from("  "));
  generate_cmd(&cmd.name, cmd, &mut res);
  (format!("{}-completions.nu", cmd.name), res.text())
}

fn generate_cmd(cmd_name: &str, cmd: &CommandInfo, out: &mut Output) {
  let valid_flag: Regex = Regex::new("[-A-Za-z]*").unwrap();

  // Instead of immediately writing the flags to the command, build up a list of
  // formatted flags here. If we need to, generate nu-complete commands to
  // complete flags first, then the actual export extern, so that the extern's
  // signature can use the `nu-complete` command for completing flags
  let mut flags_strs = vec![];
  // Flags that will need a nu-complete function to complete them
  let mut complicated_flags = Vec::new();
  for flag in &cmd.flags {
    // Filter out flags that Nu won't like
    let forms = flag
      .forms
      .iter()
      .filter(|form| {
        (form.starts_with("--") || form.len() == 2)
          && (valid_flag.is_match(form))
      })
      .map(|form| form.to_owned())
      .collect::<Vec<_>>();
    let (short_forms, long_forms): (Vec<_>, Vec<_>) =
      forms.iter().partition(|f| f.len() == 2);

    let desc_str = if let Some(desc) = &flag.desc {
      format!(" # {desc}")
    } else {
      String::new()
    };

    let type_str = if let Some(typ) = flag.typ.as_ref() {
      match typ {
        ArgType::Unknown => ": string".to_owned(),
        _ => {
          // Turn it into a valid Nu identifier
          let first_form = if forms[0].starts_with("--") {
            &forms[0][2..]
          } else if forms[0].starts_with('-') {
            &forms[0][1..]
          } else {
            &forms[0]
          };
          // This may cause collisions if there are flags with underscores, but
          // that's unlikely
          let first_form = first_form.replace("-", "_");
          let res =
            format!(r#": string@"nu-complete {} {}""#, cmd_name, &first_form);
          complicated_flags.push((first_form, typ));
          res
        }
      }
    } else {
      String::new()
    };

    // Pair off as many long and short forms as possible
    // It's unlikely there'll be multiple long *and* short forms of the same
    // flag, but you never know what kind of horrors a man page may hold
    let mut short_forms = short_forms.into_iter();
    let mut long_forms = long_forms.into_iter();
    while short_forms.len() > 0 && long_forms.len() > 0 {
      flags_strs.push(format!(
        "{}({}){}{}",
        long_forms.next().unwrap(),
        short_forms.next().unwrap(),
        type_str,
        desc_str
      ));
    }

    for form in long_forms.into_iter().chain(short_forms) {
      flags_strs.push(format!("{form}{type_str}{desc_str}"));
    }
  }

  // Generate functions to complete the more complicated flags. The flag to
  // complete is the last part of the command name rather than an argument.

  for (flag, typ) in complicated_flags {
    out.writeln(format!(r#"def "nu-complete {} {}" [] {{"#, cmd_name, flag));
    out.indent();
    out.writeln(complete_type(typ));
    out.dedent();
    out.writeln("}");
    out.writeln("");
  }

  // Generate the actual `export extern` command
  if let Some(desc) = cmd.desc.as_ref() {
    for line in desc.lines() {
      out.writeln(format!("# {}", line));
    }
  }
  out.writeln(format!("export extern \"{cmd_name}\" ["));
  out.indent();
  out.writeln(flags_strs.join("\n"));
  out.dedent();
  out.writeln("]");
  out.writeln("");

  for sub_cmd in &cmd.subcommands {
    generate_cmd(&format!("{cmd_name} {}", sub_cmd.name), sub_cmd, out);
  }
}

/// Generate Nu code to provide completions for a particular type.
///
/// Generates a list of records with a value field and possibly a description
/// field
fn complete_type(typ: &ArgType) -> String {
  match typ {
    ArgType::Unknown => complete_type(&ArgType::Path),
    ArgType::Run { cmd, sep: desc_sep } => {
      if let Some(sep) = desc_sep {
        format!(
          r#"(({}) | each {{ |it| $it | split row -n 2 '{}' | {{value: $in.0, description: $in.1}} }})"#,
          cmd, sep
        )
      } else {
        format!("(({}) | each {{ |it| {{value: $it}} }})", cmd)
      }
    }
    ArgType::Strings(strs) => {
      format!(
        "[{}]",
        strs
          .iter()
          .map(|(s, desc)| {
            if let Some(desc) = desc {
              format!("{{value: '{}', description: '{}'}}", s, desc)
            } else {
              format!("{{value: '{}'}}", s)
            }
          })
          .collect::<Vec<_>>()
          .join(", ")
      )
    }
    ArgType::Any(types) => format!(
      "[{}]",
      types
        .iter()
        .map(|typ| format!("...{}", complete_type(typ)))
        .collect::<Vec<_>>()
        .join(" ")
    ),
    _ => "[]".to_owned(), // todo implement
  }
}
