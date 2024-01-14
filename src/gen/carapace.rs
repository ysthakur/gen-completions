use std::collections::BTreeMap;

use serde::Serialize;

use super::util::{pair_forms, trim_dashes};
use crate::{ArgType, CommandInfo};

const HEADER: &str =
  "# yaml-language-server: $schema=https://carapace.sh/schemas/command.json";

/// To let `serde_yaml` serialize the command info
#[derive(Serialize)]
struct CarapaceCmd {
  name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  flags: BTreeMap<String, String>,
  #[serde(skip_serializing_if = "Completion::is_empty")]
  completion: Completion,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  commands: Vec<CarapaceCmd>,
}

#[derive(Serialize)]
struct Completion {
  #[serde(skip_serializing_if = "Vec::is_empty")]
  positional: Vec<Vec<String>>,
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  flag: BTreeMap<String, Vec<String>>,
}

impl Completion {
  fn is_empty(&self) -> bool {
    self.positional.is_empty() && self.flag.is_empty()
  }
}

/// Generate a Carapace spec from a [`CommandInfo`]
pub fn generate(cmd: &CommandInfo) -> String {
  let yaml = serde_yaml::to_string(&to_carapace(cmd))
    .expect("Carapace spec should've been serialized to YAML");
  format!("{}\n{}", HEADER, yaml)
}

/// Generate a [`CommandInfo`] to a carapace spec so it can be serialized
fn to_carapace(cmd: &CommandInfo) -> CarapaceCmd {
  // BTreeMap used rather than HashMap so that output always has predictable
  // order, otherwise tests can fail sometimes
  let mut flags = BTreeMap::new();
  let mut flag_completions = BTreeMap::new();

  for flag in &cmd.flags {
    let desc = flag.desc.clone().unwrap_or_default();
    let typ = flag.typ.as_ref().map(carapace_type);

    for (short, long) in pair_forms(&flag.forms) {
      let (main_form, combined) = match (short, long) {
        (Some(short), Some(long)) => (long, format!("{},{}", short, long)),
        (Some(short), None) => (short, short.to_owned()),
        (None, Some(long)) => (long, long.to_owned()),
        (None, None) => unreachable!(),
      };
      if let Some(typ) = typ.clone() {
        // If there's an argument, the flag name needs a `=` after it
        flags.insert(format!("{}=", combined), desc.clone());
        flag_completions.insert(trim_dashes(main_form), typ);
      } else {
        flags.insert(combined, desc.clone());
      }
    }
  }

  CarapaceCmd {
    name: cmd.name.clone(),
    description: cmd.desc.clone(),
    flags,
    completion: Completion {
      positional: cmd.args.iter().map(carapace_type).collect(),
      flag: flag_completions,
    },
    commands: cmd.subcommands.iter().map(to_carapace).collect(),
  }
}

/// Turn a type into something Carapace understands
fn carapace_type(typ: &ArgType) -> Vec<String> {
  match typ {
    ArgType::Strings(strs) => strs
      .iter()
      .map(|(val, desc)| {
        if let Some(desc) = desc {
          format!("{}\t{}", val, desc)
        } else {
          val.to_owned()
        }
      })
      .collect::<Vec<_>>(),
    ArgType::Path => vec!["$files".to_owned()],
    ArgType::Dir => vec!["$directories".to_owned()],
    ArgType::Any(types) => types.iter().flat_map(carapace_type).collect(),
    _ => todo!(),
  }
}

#[cfg(test)]
mod tests {
  use indoc::indoc;
  use pretty_assertions::assert_eq;

  use super::generate;
  use crate::{ArgType, CommandInfo, Flag};

  /// Removes the header from the generated YAML and trims both strings
  macro_rules! assert_fmt {
    ($left:expr, $right:expr) => {
      assert_eq!(
        indoc! { $left }.trim(),
        generate(&$right)
          .trim()
          .lines()
          .skip(1)
          .collect::<Vec<_>>()
          .join("\n")
      )
    };
  }

  #[test]
  fn test_empty() {
    assert_fmt!(
      r#"
        name: foo
      "#,
      CommandInfo {
        name: "foo".to_owned(),
        desc: None,
        args: vec![],
        flags: vec![],
        subcommands: vec![],
      }
    )
  }

  #[test]
  fn test_multiple_forms() {
    assert_fmt!(
      r#"
        name: foo
        description: |-
          blah blah
          Newline
        flags:
          --why=: This flag does nothing
          -b,--bar=: This flag does nothing
        completion:
          positional:
          - - $files
          flag:
            bar:
            - "baz1\tDescription for baz1"
            - "baz2\tAnother description"
            why:
            - "baz1\tDescription for baz1"
            - "baz2\tAnother description"
      "#,
      CommandInfo {
        name: "foo".to_owned(),
        desc: Some("blah blah\nNewline".to_owned()),
        args: vec![ArgType::Any(vec![ArgType::Path])],
        flags: vec![Flag {
          forms: vec!["-b".to_owned(), "--bar".to_owned(), "--why".to_owned()],
          desc: Some("This flag does nothing".to_owned()),
          typ: Some(ArgType::Strings(vec![
            ("baz1".to_owned(), Some("Description for baz1".to_owned())),
            ("baz2".to_owned(), Some("Another description".to_owned()))
          ]))
        }],
        subcommands: vec![],
      }
    )
  }
}
