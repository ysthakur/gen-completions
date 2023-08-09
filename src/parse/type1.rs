use log::debug;

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

      let mut paras = content.split(".PP");
      paras.next(); // Discard the part before the first option
      for para in paras {
        if let Some(end) = para.find(".RE") {
          let data = &para[0..end];
          let data = util::remove_groff_formatting(data);
          let mut data = data.split(".RS 4");
          let options = data.next().unwrap();
          let desc = data.next();
          if let Some(arg) = util::make_arg(options, desc) {
            args.push(arg);
          }
        } else {
          debug!(
            "No .RE found to end description, para: {}",
            util::truncate(&para, 40)
          );
        }
      }

      Some(args)
    }
    None => None,
  }
}
