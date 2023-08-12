//! Common utilities for parsers

use log::debug;
use regex::{Regex, RegexBuilder};

use super::Flag;

/// Maximum length of a description
///
/// After this, `...` will be added
static MAX_DESC_LEN: usize = 80;

static ELLIPSIS: &str = "...";

/// Match roff numeric expressions
pub static NUM_RE: &str = r"(\d+(\.\d)?)";

pub fn trim_desc(desc: &str) -> String {
  // Remove extra spaces after sentence ends
  let re = Regex::new(r"\.\s+").unwrap();
  let desc = re.replace_all(desc, ". ");

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
  RegexBuilder::new(&format!(r#"\.SH {title}(.*?)(\.SH|\z)"#))
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
  // also, fish uses a slightly different regex: `.PD( \d+)`, check if that's
  // fine
  let re = Regex::new(r"\.PD \d+").unwrap();
  let data = re.replace_all(&data, "");
  let data = data
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
    .replace('\u{C}', "")
    .replace(r"\(cq", "'")
    .replace(r"\(aq", "'"); // Added by me, not from Fish. May need to remove all \(xx

  // todo Fish doesn't do this, see how it handles .sp
  let re = Regex::new(&format!(r"\.sp( {NUM_RE}v?)?")).unwrap();
  re.replace_all(&data, "").to_string()
}

/// Truncates to at most `len` characters, as well as trims and removes newlines
pub fn truncate(s: &str, len: usize) -> String {
  let s = s.trim().replace('\n', " ");
  if s.len() > len {
    s[0..len].to_string()
  } else {
    s
  }
}

/// Parse the line of options after .PP and the description after it
///
/// Ported from Fish's `built_command`
pub fn make_flag(options: &str, desc: Option<&str>) -> Option<Flag> {
  // Unquote the options string
  let options = options.trim();
  let options = if options.len() < 2 {
    options
  } else if (options.starts_with('"') && options.ends_with('"'))
    || (options.starts_with('\'') && options.ends_with('\''))
  {
    &options[1..options.len() - 1]
  } else {
    options
  };

  let mut forms = Vec::new();
  let delim = Regex::new(r#"[ ,="|]"#).unwrap();
  for option in delim.split(options) {
    let option = Regex::new(r"\[.*\]").unwrap().replace(option, "");
    // todo Fish doesn't replace <.*> so maybe this is wrong
    let option = Regex::new(r"<.*").unwrap().replace(&option, "");
    // todo this is ridiculously verbose
    let option = option
      .trim_matches(" \t\r\n[](){}.:!".chars().collect::<Vec<_>>().as_slice());
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
      String::new()
    };
    debug!("No options found in '{}', desc: '{}'", options.trim(), desc);
    return None;
  }

  match desc {
    Some(desc) => {
      let desc = desc.trim().replace('\n', " ");
      let desc = desc.trim_end_matches('.');
      // Remove bogus escapes
      let desc = desc.replace(r"\'", "").replace(r"\.", "");

      let desc = trim_desc(&desc);
      let desc = if desc.is_empty() { None } else { Some(desc) };
      Some(Flag { forms, desc })
    }
    None => Some(Flag { forms, desc: None }),
  }
}
