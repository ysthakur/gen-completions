use log::debug;
use regex::Regex;

use super::util::{self, ParseResult};

/// Ported from Fish's `Type2ManParser`
pub fn try_parse(cmd_name: &str, page_text: &str) -> Option<ParseResult> {
  util::get_section("OPTIONS", page_text).map(|content| parse(cmd_name, &content))
}
fn parse(cmd_name: &str, content: &str) -> ParseResult {
  let mut flags = Vec::new();

      // todo this diverges from the Fish impl for splitting, check if it's okay
      // need to see more samples of manpages of this kind
      let para_re =
        Regex::new(&format!(r"\.[IT]P( {}i?)?", util::NUM_RE)).unwrap();
      let para_end = Regex::new(r"\.(IP|TP|UNINDENT|UN|SH)").unwrap();

      let mut paras = para_re.split(content);
      paras.next(); // Discard the part before the first option
      for para in paras {
        let data = if let Some(mat) = para_end.find(para) {
          &para[0..mat.start()]
        } else {
          // todo should this case be an error?
          para
        };
        let data = util::remove_groff_formatting(data);
        let data = data.trim();
        let flag = if let Some((options, desc)) = data.split_once('\n') {
          util::make_flag(options, Some(desc))
        } else {
          // todo should this be an error instead?
          debug!(
            "In command {cmd_name}, no description, data: {}",
            util::truncate(data, 40)
          );
          util::make_flag(data, None)
        };
        if let Some(flag) = flag {
          flags.push(flag);
        }
      }

      Ok(flags)
}
