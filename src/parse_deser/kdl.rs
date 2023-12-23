//! For deserializing from KDL, because the serde support is not great

use kdl::KdlDocument;
use thiserror::Error;

use crate::CommandInfo;

/// An error encountered when deserializing KDL specifically
#[derive(Debug, Error)]
pub enum KdlDeserError {
  #[error(transparent)]
  ParseError(#[from] kdl::KdlError),
}

type Result<T> = std::result::Result<T, KdlDeserError>;

pub fn parse_from_str(text: &str) -> Result<CommandInfo> {
  kdl_to_cmd_info(text.parse()?)
}

fn kdl_to_cmd_info(doc: KdlDocument) -> Result<CommandInfo> {
  todo!()
}

#[cfg(test)]
mod tests {
  use super::parse_from_str;
  use crate::CommandInfo;

  #[test]
  fn test1() {
    assert_eq!(
      CommandInfo {
        name: "foo".to_string(),
        flags: vec![],
        subcommands: vec![]
      },
      parse_from_str(r#"
        foo {
          flags {
            - "--help" "-h" {
              "Show help output"
            }
          }
        }
      "#).unwrap()
    )
  }
}
