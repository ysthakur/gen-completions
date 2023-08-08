//! Common utilities for parsers

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
