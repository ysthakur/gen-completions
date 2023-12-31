use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),

  #[error("Unsupported manpage format")]
  UnsupportedFormat(),

  #[error("Could not find manpage for {cmd_name}")]
  ManpageNotFound { cmd_name: String },
}
