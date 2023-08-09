/// Wrap in single quotes (and escape single quotes inside) so that it's safe
/// for Bash and Zsh to read
pub fn quote(s: &str) -> String {
  format!("'{}'", s.replace('\'', r#"'"'"'"#))
}
