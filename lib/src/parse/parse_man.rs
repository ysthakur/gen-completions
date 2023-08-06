use regex::{Regex, RegexBuilder};

use crate::result::Result;

use super::CommandInfo;

/// Regex to get the contents of a section with the given title
fn regex_for_section(title: &str) -> Regex {
  RegexBuilder::new(&format!(r#"\.SH {title}(.*?)(\.SH|\z)"#))
    .multi_line(true)
    .dot_matches_new_line(true)
    .build()
    .unwrap()
}

pub fn parse(cmd_name: &str, page_text: &str) -> Result<Option<CommandInfo>> {
  let re = regex_for_section(r#""OPTIONS""#);
  match re.captures(page_text) {
    Some(captures) => {
      let content = captures.get(1).unwrap().as_str();
      println!("{}", content);
      todo!()
    }
    None => Ok(None),
  }
}
