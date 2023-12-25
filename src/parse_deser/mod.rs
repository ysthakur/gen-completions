//! For parsing completions from a serialization language (KDL, JSON, or YAML)

pub mod error;
mod kdl;

use std::{fs, path::Path};

use self::error::DeserError;
use crate::{parse_deser::error::Error, CommandInfo};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone)]
pub enum InputFormat {
  Kdl,
  Json,
  Yaml,
}

/// # Errors
///
/// Fails if the file's extension isn't recognized (only KDL, JSON, and YAML are
/// supported), or if [`parse_from_str`] fails.
pub fn parse(file: impl AsRef<Path>) -> Result<CommandInfo> {
  let file = file.as_ref();
  let file_path = file.to_string_lossy().to_string();
  if let Some(ext) = file.extension() {
    match fs::read_to_string(file) {
      Ok(text) => {
        if let Some(ext) = ext.to_str() {
          let format = match ext {
            "json" => InputFormat::Json,
            "yaml" | "yml" => InputFormat::Yaml,
            "kdl" => InputFormat::Kdl,
            _ => return Err(Error::UnrecognizableExtension { file_path }),
          };
          parse_from_str(&text, format).map_err(|e| Error::Deser {
            file_path,
            text,
            source: e,
          })
        } else {
          Err(Error::UnrecognizableExtension { file_path })
        }
      }
      Err(e) => Err(Error::Io {
        file_path,
        source: e,
      }),
    }
  } else {
    Err(Error::NoExtension { file_path })
  }
}

/// # Errors
///
/// Fails if the shape of the KDL/JSON/YAML didn't match a [`CommandInfo`]
pub fn parse_from_str(
  text: &str,
  format: InputFormat,
) -> std::result::Result<CommandInfo, DeserError> {
  let cmd_info = match format {
    InputFormat::Json => serde_json::from_str(text)?,
    InputFormat::Yaml => serde_yaml::from_str(text)?,
    InputFormat::Kdl => kdl::parse_from_str(text)?,
  };
  Ok(cmd_info)
}
