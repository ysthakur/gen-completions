use std::io;

use miette::Diagnostic;
use thiserror::Error;

use super::kdl::KdlDeserError;

#[derive(Debug, Diagnostic, Error)]
pub enum Error {
  #[error("{file_path} has no extension")]
  #[diagnostic(code(gen_completions::deser::no_ext), url(docsrs))]
  NoExtension { file_path: String },

  #[error("{file_path} has an unrecognizable extension")]
  #[diagnostic(code(gen_completions::deser::unrecognizable_ext), url(docsrs))]
  UnrecognizableExtension { file_path: String },

  #[error("Error encountered while reading {file_path}")]
  #[diagnostic(code(gen_completions::deser::io_error), url(docsrs))]
  Io {
    file_path: String,
    #[source]
    source: io::Error,
  },

  #[error("Error encountered while deserializing {file_path}")]
  Deser {
    file_path: String,
    #[source_code]
    text: String,
    #[diagnostic_source]
    source: DeserError,
  },
}

/// An error encountered while deserializing
#[derive(Debug, Diagnostic, Error)]
pub enum DeserError {
  #[error(transparent)]
  Json(#[from] serde_json::Error),

  #[error(transparent)]
  #[diagnostic()]
  Yaml(#[from] serde_yaml::Error),

  #[error(transparent)]
  #[diagnostic(transparent)]
  Kdl(#[from] KdlDeserError),
}
