use std::collections::{HashMap, HashSet};

use regex::{Regex, RegexBuilder};

use anyhow::Result;

use super::{Arg, CommandInfo};

/// Maximum length of a description
///
/// After this, `...` will be added
const MAX_DESC_LEN: usize = 80;

const ELLIPSIS: &str = "...";

/// Regex to get the contents of a section with the given title
fn regex_for_section(title: &str) -> Regex {
  RegexBuilder::new(&format!(r#"\.SH {title}(.*?)(\.SH|\z)"#))
    .multi_line(true)
    .dot_matches_new_line(true)
    .build()
    .unwrap()
}

pub fn parse(_cmd_name: &str, page_text: &str) -> Result<Option<CommandInfo>> {
  let re = regex_for_section(r#""OPTIONS""#);
  match re.captures(page_text) {
    Some(captures) => {
      let content = captures.get(1).unwrap().as_str();
      let mut args = Vec::new();

      for para in content.split(".PP") {
        if let Some(end) = para.find(".RE") {
          let data = &para[0..end];
          let data = remove_groff_formatting(data);
          let mut data = data.split(".RS 4");
          let options = data.next().unwrap();
          if let Some(desc) = data.next() {
            args.push(make_arg(options, desc));
          } else {
            println!("No indent in description");
          }
        }
      }

      Ok(Some(CommandInfo {
        args,
        subcommands: HashMap::new(),
      }))
    }
    None => Ok(None),
  }
}

// Copied more or less directly from Fish's `built_command`
fn make_arg(options: &str, desc: &str) -> Arg {
  let mut forms = HashSet::new();

  // Unquote the options
  let options = if options.len() == 1 {
    options
  } else if options.starts_with('"') && options.ends_with('"') {
    &options[1..options.len() - 1]
  } else if options.starts_with('\'') && options.ends_with('\'') {
    &options[1..options.len() - 1]
  } else {
    options
  };
  let delim = Regex::new(r#"[ ,="|]"#).unwrap();
  for option in delim.split(options) {
    let option = Regex::new(r"\[.*\]").unwrap().replace(option, "");
    // todo this is ridiculously verbose
    let option = option.trim_matches(" \t\r\n[](){}.,:!".chars().collect::<Vec<_>>().as_slice());
    if !option.starts_with('-') || option == "-" || option == "--" {
      continue;
    }
    // todo use str.matches instead
    if Regex::new(r"\{\}\(\)").unwrap().is_match(option) {
      continue;
    }
    forms.insert(option.to_owned());
  }

  let desc = desc.trim().replace("\n", " ");
  let desc = desc.trim_end_matches('.');
  // Remove bogus escapes
  let desc = desc.replace(r"\'", "").replace(r"\.", "");

  // TODO port the sentence-splitting part too

  let desc = if desc.len() > MAX_DESC_LEN {
    format!("{}{}", &desc[0..MAX_DESC_LEN - ELLIPSIS.len()], ELLIPSIS)
  } else {
    desc
  };

  Arg { forms, desc }
}

// Copied more or less directly from Fish
fn remove_groff_formatting(data: &str) -> String {
  let data = data
    .replace(r"\fI", "")
    .replace(r"\fP", "")
    .replace(r"\f1", "")
    .replace(r"\fB", "")
    .replace(r"\fR", "")
    .replace(r"\e", "");
  // TODO check if this one is necessary
  // also, fish uses a slightly different regex: `.PD( \d+)`, check if that's fine
  let re = Regex::new(r"\.PD \d+").unwrap();
  let data = re.replace_all(&data, "");
  data
    .replace(".BI", "")
    .replace(".BR", "")
    .replace("0.5i", "")
    .replace(".rb", "")
    .replace(r"\^", "")
    .replace("{ ", "")
    .replace(" }", "")
    .replace(r"\ ", "")
    .replace(r"\-", "-")
    .replace(r"\&", "")
    .replace(".B", "")
    .replace(r"\-", "-")
    .replace(".I", "")
    .replace("\u{C}", "")
    .replace(r"\(cq", "'")

  // TODO .sp is being left behind, see how Fish handles it
}
