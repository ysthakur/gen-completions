use regex::{Regex, RegexBuilder};

use super::util;
use crate::Flag;

/// For parsing Darwin man pages (ported from Fish)
#[allow(
  clippy::case_sensitive_file_extension_comparisons,
  clippy::doc_markdown
)]
pub fn parse(cmd_name: &str, page_text: &str) -> Vec<Flag> {
  let Some(start_ind) = page_text.find(".Sh DESCRIPTION") else {
    return Vec::new();
  };

  let mut flags = Vec::new();

  // Can't use util::get_section because we also want sections after DESCRIPTION
  let content = &page_text[start_ind..];

  // Replace '.It Fl' and the like with '.It -' (`Fl` is a dash)
  let fl_re = Regex::new(r"(\...) Fl ").expect("Regex should be valid");
  let content = fl_re.replace_all(content, "$1 -").replace(".Fl ", "-");

  let desc_end = RegexBuilder::new(r"\n\.El.*+")
    .dot_matches_new_line(true)
    .build()
    .expect("Regex should be valid");

  let mut paras = content.split(".It");
  paras.next(); // Discard the part before the first option
  for para in paras {
    let mut pieces = para.splitn(2, '\n');
    if let Some(options) = pieces.next() {
      let desc = pieces.next().map(|desc| {
        let desc = desc.replace(".Nm", cmd_name);
        desc_end.replace(&desc, "").to_string()
      });
      if let Some(flag) = util::make_flag(options, desc.as_deref()) {
        flags.push(flag);
      }
    }
  }

  flags
}
