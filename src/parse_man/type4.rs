use log::warn;

use super::{
  util::{self, ParseResult},
};

/// Ported from Fish's `Type4ManParser`
///
/// TODO This is completely untested
#[allow(clippy::case_sensitive_file_extension_comparisons)]
pub fn try_parse(cmd_name: &str, page_text: &str) -> Option<ParseResult> {
  util::get_section("FUNCTION LETTERS", page_text)
    .map(|content| parse(cmd_name, &content))
}

fn parse(cmd_name: &str, content: &str) -> ParseResult {
  let mut flags = Vec::new();

  let mut paras = content.split(".TP");
  paras.next(); // Discard the part before the first option
  for para in paras {
    let data = util::remove_groff_formatting(para);
    let data = data.trim();
    if let Some((options, desc)) = data.split_once('\n') {
      if let Some(flag) = util::make_flag(options, Some(desc)) {
        flags.push(flag);
      }
    } else {
      warn!(
        "In command {cmd_name}, no description, data: {}",
        util::truncate(data, 40)
      );
    }
  }

  Ok(flags)
}
