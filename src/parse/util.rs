//! Common utilities for parsers

use super::Arg;
use log::debug;
use regex::{Regex, RegexBuilder};

/// Maximum length of a description
///
/// After this, `...` will be added
static MAX_DESC_LEN: usize = 80;

static ELLIPSIS: &str = "...";

/// Note to future self: Don't bother making this return a Cow since the
/// description will usually be trimmed anyway
pub fn trim_desc(desc: String) -> String {
  // TODO port the sentence-splitting part too
  // https://github.com/fish-shell/fish-shell/blob/master/share/tools/create_manpage_completions.py#L211
  if desc.len() > MAX_DESC_LEN {
    format!("{}{}", &desc[0..MAX_DESC_LEN - ELLIPSIS.len()], ELLIPSIS)
  } else {
    desc.to_string()
  }
}

/// Regex to get the contents of a section with the given title
pub fn regex_for_section(title: &str) -> Regex {
  RegexBuilder::new(&format!(r#"\.SH {}(.*?)(\.SH|\z)"#, title))
    .multi_line(true)
    .dot_matches_new_line(true)
    .build()
    .unwrap()
}

/// Copied more or less directly from Fish's `remove_groff_formatting`
pub fn remove_groff_formatting(data: &str) -> String {
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
    .replace(r"\(aq", "'") // Added by me, not from Fish. May need to remove all \(xx

  // TODO .sp is being left behind, see how Fish handles it
}

/// Truncates to at most `len` characters, as well as trims and removes newlines
pub fn truncate(s: &str, len: usize) -> String {
  let s = s.trim().replace("\n", " ");
  if s.len() > len {
    s[0..len].to_string()
  } else {
    s
  }
}

/// Parse the line of options after .PP and the description after it
///
/// Ported from Fish's `built_command`
pub fn make_arg(options: &str, desc: Option<&str>) -> Option<Arg> {
  // Unquote the options string
  let options = options.trim();
  let options = if options.len() < 2 {
    options
  } else if options.starts_with('"') && options.ends_with('"') {
    &options[1..options.len() - 1]
  } else if options.starts_with('\'') && options.ends_with('\'') {
    &options[1..options.len() - 1]
  } else {
    options
  };

  let mut forms = Vec::new();
  let delim = Regex::new(r#"[ ,="|]"#).unwrap();
  for option in delim.split(options) {
    let option = Regex::new(r"\[.*\]").unwrap().replace(option, "");
    // todo Fish doesn't replace <.*> so maybe this is wrong
    let option = Regex::new(r"<.*>").unwrap().replace(&option, "");
    // todo this is ridiculously verbose
    let option =
      option.trim_matches(" \t\r\n[](){}.:!".chars().collect::<Vec<_>>().as_slice());
    if !option.starts_with('-') || option == "-" || option == "--" {
      continue;
    }
    if Regex::new(r"\{\}\(\)").unwrap().is_match(option) {
      continue;
    }
    forms.push(option.to_owned());
  }

  if forms.is_empty() {
    let desc = if let Some(desc) = desc {
      truncate(desc, 40)
    } else {
      String::from("")
    };
    debug!("No options found in '{}', desc: '{}'", options.trim(), desc);
    return None;
  }

  match desc {
    Some(desc) => {
      let desc = desc.trim().replace("\n", " ");
      let desc = desc.trim_end_matches('.');
      // Remove bogus escapes
      let desc = desc.replace(r"\'", "").replace(r"\.", "");

      let desc = trim_desc(desc);
      let desc = if desc.is_empty() { None } else { Some(desc) };
      Some(Arg { forms, desc })
    }
    None => Some(Arg { forms, desc: None }),
  }
}