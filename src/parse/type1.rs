use log::debug;
use regex::Regex;

use super::{util, Arg};

/// Ported from Fish's `Type1ManParser`
///
/// todo implement fallback and fallback2 like the Fish script
pub fn parse(cmd_name: &str, page_text: &str) -> Option<Vec<Arg>> {
  let re = util::regex_for_section(r#""OPTIONS""#);
  match re.captures(page_text) {
    Some(captures) => {
      let content = captures.get(1).unwrap().as_str();
      let mut args = Vec::new();

      for para in content.split(".PP") {
        if let Some(end) = para.find(".RE") {
          let data = &para[0..end];
          let data = util::remove_groff_formatting(data);
          let mut data = data.split(".RS 4");
          let options = data.next().unwrap();
          let desc = data.next();
          if let Some(arg) = make_arg(options, desc) {
            args.push(arg);
          }
        }
      }

      Some(args)
    }
    None => None,
  }
}

/// Parse the line of options after .PP and the description after it
///
/// Ported from Fish's `built_command`
fn make_arg(options: &str, desc: Option<&str>) -> Option<Arg> {
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
      &desc.trim()[..40]
    } else {
      ""
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

      let desc = util::trim_desc(desc);
      let desc = if desc.is_empty() { None } else { Some(desc) };
      Some(Arg { forms, desc })
    }
    None => Some(Arg { forms, desc: None }),
  }
}
