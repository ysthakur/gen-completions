use log::debug;

use super::util::{self, ParseResult};

/// Ported from Fish's `Type1ManParser`
///
/// todo implement fallback and fallback2 like the Fish script
pub fn try_parse(cmd_name: &str, page_text: &str) -> Option<ParseResult> {
  util::get_section(r#""OPTIONS""#, page_text)
    .map(|content| parse(cmd_name, &content))
}

fn parse(cmd_name: &str, content: &str) -> ParseResult {
  let mut flags = Vec::new();

  let mut paras = content.split(".PP");
  paras.next(); // Discard the part before the first option
  for para in paras {
    if let Some(end) = para.find(".RE") {
      let data = &para[0..end];
      let data = util::remove_groff_formatting(data);
      let mut data = data.split(".RS 4");
      let options = data.next().unwrap();
      let desc = data.next();
      if let Some(flag) = util::make_flag(options, desc) {
        flags.push(flag);
      }
    } else {
      debug!(
        "In command {cmd_name}, no .RE found to end description, para: {}",
        util::truncate(para, 40)
      );
    }
  }

  Ok(flags)
}
