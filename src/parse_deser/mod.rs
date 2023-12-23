//! For parsing completions from a serialization language (KDL, JSON, or YAML)

pub mod error;
mod kdl;

use std::{fs, path::Path};

use self::error::DeserError;
use crate::{parse_deser::error::Error, CommandInfo};

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse(file: impl AsRef<Path>) -> Result<CommandInfo> {
  let file = file.as_ref();
  let file_path = file.to_string_lossy().to_string();
  if let Some(ext) = file.extension() {
    match fs::read_to_string(file) {
      Ok(text) => {
        if let Some(ext) = ext.to_str() {
          match parse_from_str(&text, ext) {
            Ok(Some(cmd_info)) => Ok(cmd_info),
            Ok(None) => Err(Error::UnrecognizableExtension { file_path }),
            Err(e) => Err(Error::Deser {
              file_path,
              source: e,
            }),
          }
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

pub fn parse_from_str(
  text: &str,
  ext: &str,
) -> std::result::Result<Option<CommandInfo>, DeserError> {
  let cmd_info = match ext {
    "json" => Some(serde_json::from_str(&text)?),
    "yaml" | "yml" => Some(serde_yaml::from_str(&text)?),
    "kdl" => Some(kdl::parse_from_str(&text)?),
    _ => None,
  };
  Ok(cmd_info)
}
