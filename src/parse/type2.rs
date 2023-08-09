use log::{debug, trace};
use regex::Regex;

use super::{util, Arg};

/// Ported from Fish's `Type2ManParser`
///
/// TODO actually test this
pub fn parse(cmd_name: &str, page_text: &str) -> Option<Vec<Arg>> {
  let re = util::regex_for_section("OPTIONS");
  match re.captures(page_text) {
    Some(captures) => {
      let content = captures.get(1).unwrap().as_str();
      let mut args = Vec::new();

      // todo this diverges from the Fish impl for splitting, check if it's okay
      // need to see more samples of manpages of this kind
      let para_re = Regex::new(&format!(r"\.[IT]P( {}i?)?", util::NUM_RE)).unwrap();
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
        let arg = if let Some((options, desc)) = data.split_once("\n") {
          util::make_arg(options, Some(desc))
        } else {
          // todo should this be an error instead?
          debug!("No description, data: {}", util::truncate(&data, 40));
          util::make_arg(&data, None)
        };
        if let Some(arg) = arg {
          args.push(arg);
        }
      }

      Some(args)
    }
    None => None,
  }
}
