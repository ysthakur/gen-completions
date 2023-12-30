/// Wrap in single quotes (and escape single quotes inside) so that it's safe
/// for Bash and Zsh to read
pub fn quote_bash<S: AsRef<str>>(s: S) -> String {
  format!("'{}'", s.as_ref().replace('\'', r#"'"'"'"#))
}

/// Helper to write indented text to a string
pub struct Output {
  text: String,
  indent_str: String,
  indent_level: usize,
  /// If true, need to indent when the next string is written
  line_ended: bool,
}

impl Output {
  /// Make a new [Output]. `indent_str` is the string to indent with (e.g.
  /// `"\t"`).
  pub fn new(indent_str: String) -> Output {
    Output {
      text: String::new(),
      indent_str,
      indent_level: 0,
      line_ended: false,
    }
  }

  /// Increase the indentation level by 1
  pub fn indent(&mut self) {
    self.indent_level += 1;
  }

  /// Decrease the indentation level by 1
  pub fn dedent(&mut self) {
    self.indent_level -= 1;
  }

  fn write_indent(&mut self) {
    for _ in 0..self.indent_level {
      self.text.push_str(&self.indent_str);
    }
  }

  /// Write some text (without a newline)
  pub fn write<S: AsRef<str>>(&mut self, s: S) {
    if self.line_ended {
      self.write_indent();
      self.line_ended = false;
    }

    let mut lines = s.as_ref().split('\n');
    if let Some(mut line) = lines.next() {
      loop {
        self.text.push_str(line);
        if let Some(next) = lines.next() {
          self.text.push('\n');
          self.write_indent();
          line = next;
        } else {
          break;
        }
      }
    }
  }

  /// Write some text (with a newline)
  pub fn writeln<S: AsRef<str>>(&mut self, s: S) {
    self.write(s.as_ref());
    self.text.push('\n');
    self.line_ended = true;
  }

  /// Consume this [Output], returning the text written to it
  pub fn text(self) -> String {
    self.text
  }
}
