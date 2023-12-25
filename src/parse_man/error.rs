use miette::{Diagnostic, NamedSource};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),

  #[error("Unsupported manpage format")]
  #[diagnostic(code(gen_completions::man::unsupported_format), url(docsrs))]
  UnsupportedFormat(),

  #[error("Could not find manpage for {cmd_name}")]
  #[diagnostic(code(gen_completions::man::manpage_not_found), url(docsrs))]
  ManpageNotFound { cmd_name: String },

  #[error("Error when parsing command")]
  #[diagnostic(
    code(gen_completions::man::parse_error),
    url(docsrs),
    forward(source)
  )]
  ParseError {
    #[source_code]
    source_code: NamedSource,
    source: ParseError,
  },
}

#[derive(Debug, Diagnostic, Error)]
pub enum ParseError {}
