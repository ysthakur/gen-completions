use log::debug;
use regex::RegexBuilder;

use super::{util, Flag};

/// Ported from Fish's `Type3ManParser`
/// Fish's `Type3ManParser` doesn't handle HP...IP...HP, but the man page for
/// sed, at least, uses that, so this parser handles that too.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
pub fn parse(page_text: &str) -> Option<Vec<Flag>> {
  match util::get_section("DESCRIPTION", page_text) {
    Some(content) => {
      let mut flags = Vec::new();

      let start_re = RegexBuilder::new(r"\.[HT]P(.*?)(\.[HPT]P|\z)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

      // Where the last match ended
      let mut last_end = 0;

      while let Some(mat) = start_re.find_at(&content, last_end) {
        let data = &content[mat.start() + 3..mat.end()];
        // Remove the .HP/.TP/.PP at the end
        // todo this is kinda verbose
        let end_offset = if data.ends_with(".HP")
          || data.ends_with(".TP")
          || data.ends_with(".PP")
        {
          3
        } else {
          0
        };
        let data = &data[..data.len() - end_offset];
        last_end = mat.end() - end_offset;

        if let Some((options, desc)) = data.split_once(".IP") {
          // This means there is a .HP before the options
          let options = util::remove_groff_formatting(options);
          let desc = util::remove_groff_formatting(desc);
          if let Some(flag) = util::make_flag(&options, Some(&desc)) {
            flags.push(flag);
          }
        } else {
          // This means there is a .TP before the options
          let data = util::remove_groff_formatting(data);
          let data = data.trim();
          if let Some((options, desc)) = data.split_once('\n') {
            if let Some(flag) = util::make_flag(options, Some(desc)) {
              flags.push(flag);
            }
          } else {
            // todo should this be an error instead?
            debug!("No description, data: {}", util::truncate(data, 40));
          }
        }
      }

      if flags.is_empty() {
        None
      } else {
        Some(flags)
      }
    }
    None => None,
  }
}
