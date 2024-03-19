use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),

  #[error("Unsupported manpage format for {path}")]
  UnsupportedFormat { path: PathBuf },

  #[error("Could not find manpage for {cmd_name}")]
  ManpageNotFound { cmd_name: String },
}
