use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("could not parse manpage")]
  ParseError(String),

  #[error(transparent)]
  IoError(#[from] std::io::Error),

  #[error("no manpages found. Please set the MANPATH environment variable.")]
  NoManPages,

  #[error("encountered errors: {errors:?}")]
  Multiple { errors: Vec<Error> },

  #[error("{msg:?}")]
  Other { msg: String },
}

impl FromIterator<Error> for Error {
  fn from_iter<T: IntoIterator<Item = Error>>(iter: T) -> Self {
    Error::Multiple {
      errors: iter.into_iter().collect(),
    }
  }
}
