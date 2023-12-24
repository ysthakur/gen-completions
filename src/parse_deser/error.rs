use std::io;

use thiserror::Error;

use super::kdl::KdlDeserError;

#[derive(Debug, Error)]
pub enum Error {
  #[error("{file_path} has no extension")]
  NoExtension { file_path: String },
  #[error("{file_path} has an unrecognizable extension")]
  UnrecognizableExtension { file_path: String },
  #[error("error encountered while reading {file_path}")]
  Io {
    file_path: String,
    #[source]
    source: io::Error,
  },
  #[error("error encountered while deserializing {file_path}")]
  Deser {
    file_path: String,
    #[source]
    source: DeserError,
  },
}

/// An error encountered while deserializing
#[derive(Debug, Error)]
pub enum DeserError {
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  Yaml(#[from] serde_yaml::Error),
  #[error(transparent)]
  Kdl(#[from] KdlDeserError),
}
