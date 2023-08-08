use log::debug;
use regex::{Regex, RegexBuilder};

use super::{Arg, ManParser};

/// Maximum length of a description
///
/// After this, `...` will be added
const MAX_DESC_LEN: usize = 80;

const ELLIPSIS: &str = "...";

pub struct Type1Parser;

impl ManParser for Type1Parser {
  fn parse(self, cmd_name: &str, page_text: &str) -> Option<Vec<Arg>> {
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
              if let Some(arg) = make_arg(options, desc) {
                args.push(arg);
              }
            } else {
              debug!("No indent in description, cmd: {}", cmd_name);
            }
          }
        }

        Some(args)
      }
      None => None,
    }
  }
}

/// Regex to get the contents of a section with the given title
fn regex_for_section(title: &str) -> Regex {
  RegexBuilder::new(&format!(r#"\.SH {title}(.*?)(\.SH|\z)"#))
    .multi_line(true)
    .dot_matches_new_line(true)
    .build()
    .unwrap()
}

// Copied more or less directly from Fish's `built_command`
fn make_arg(options: &str, desc: &str) -> Option<Arg> {
  let mut forms = Vec::new();

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
    let option =
      option.trim_matches(" \t\r\n[](){}.,:!".chars().collect::<Vec<_>>().as_slice());
    if !option.starts_with('-') || option == "-" || option == "--" {
      continue;
    }
    // todo use str.matches instead
    if Regex::new(r"\{\}\(\)").unwrap().is_match(option) {
      continue;
    }
    forms.push(option.to_owned());
  }

  if forms.is_empty() {
    debug!("No options found in {}", options);
    return None;
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

  Some(Arg { forms, desc })
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
