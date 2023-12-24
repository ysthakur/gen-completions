//! For deserializing from KDL, because the serde support is not great

use std::collections::HashMap;

use kdl::{KdlDocument, KdlNode};
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{CommandInfo, Flag};

/// An error encountered when deserializing KDL specifically
#[derive(Debug, Diagnostic, Error)]
pub enum KdlDeserError {
  #[error(transparent)]
  SyntaxError(#[from] kdl::KdlError),

  #[error("file was empty, expected one node")]
  EmptyFile,

  #[error("expected exactly one node, got {0}")]
  TooManyNodes(usize),

  /// The text was valid KDL but could not be read as a [`CommandInfo`]
  #[error("errors encountered while reading command information")]
  ParseError(#[source_code] String, #[related] Vec<ParseError>),
}

#[derive(Debug, Diagnostic, Error)]
pub enum ParseError {
  #[error("unexpected child {child_name}")]
  UnexpectedChild {
    child_name: String,
    #[label("only flags and subcommands are allowed, not {child_name}")]
    span: SourceSpan,
  },

  #[error("duplicate child node")]
  #[diagnostic(help("merge the {child_name} nodes together"))]
  DuplicateChild {
    child_name: String,
    #[label("duplicate node named {child_name}")]
    span: SourceSpan,
    #[label("already given here")]
    prev_span: SourceSpan,
  },

  #[error("duplicate flag")]
  DuplicateFlag {
    flag: String,
    #[label("duplicate flag")]
    span: SourceSpan,
    #[label("already given here")]
    prev_span: SourceSpan,
  },

  #[error("flags should be strings, got {msg}")]
  #[diagnostic(help("wrap your flags in quotes"))]
  InvalidFlag {
    msg: String,
    #[label("should be a single string")]
    span: SourceSpan,
  },

  #[error("invalid description, expected a single string")]
  InvalidDescription(#[label("should be a single string")] SourceSpan),
}

type Result<T> = std::result::Result<T, KdlDeserError>;

/// Parse a string as KDL and convert it to a [`CommandInfo`]
///
/// # Errors
///
/// Possible reasons for failure:
/// - The document isn't valid KDL
/// - The document doesn't have exactly one node
/// - The format of the document doesn't match the shape of a [`CommandInfo`]
pub fn parse_from_str(text: &str) -> Result<CommandInfo> {
  let doc: KdlDocument = text.parse()?;
  let nodes = doc.nodes();
  if nodes.is_empty() {
    Err(KdlDeserError::EmptyFile)
  } else if nodes.len() > 1 {
    Err(KdlDeserError::TooManyNodes(nodes.len()))
  } else {
    kdl_to_cmd_info(&nodes[0])
      .map_err(|errors| KdlDeserError::ParseError(text.to_string(), errors))
  }
}

/// Convert a KDL node representing a command to a [`CommandInfo`]
///
/// Returns a list of all errors encountered along the way, if it failed
fn kdl_to_cmd_info(
  node: &KdlNode,
) -> std::result::Result<CommandInfo, Vec<ParseError>> {
  let name = node.name().to_string();
  let mut flags = vec![];
  let mut subcommands = vec![];

  let mut errors = vec![];

  if let Some(doc) = node.children() {
    let mut first_flags_node = None;
    let mut first_subcmds_node = None;

    for node in doc.nodes() {
      match node.name().to_string().as_str() {
        "flags" => {
          if let Some(prev_span) = first_flags_node {
            errors.push(ParseError::DuplicateChild {
              child_name: "flags".to_string(),
              span: *node.name().span(),
              prev_span,
            });
          } else {
            first_flags_node = Some(*node.name().span());

            let mut flag_spans = HashMap::new();

            if let Some(children) = node.children() {
              for flag_node in children.nodes() {
                match kdl_to_flag(flag_node, &mut flag_spans) {
                  Ok(flag) => flags.push(flag),
                  Err(mut errs) => errors.append(&mut errs),
                }
              }
            }
          }
        }
        "subcommands" => {
          if let Some(prev_span) = first_subcmds_node {
            errors.push(ParseError::DuplicateChild {
              child_name: "flags".to_string(),
              span: *node.name().span(),
              prev_span,
            });
          } else {
            first_subcmds_node = Some(*node.name().span());

            if let Some(children) = node.children() {
              for subcmd_node in children.nodes() {
                match kdl_to_cmd_info(subcmd_node) {
                  Ok(subcmd_info) => subcommands.push(subcmd_info),
                  Err(mut errs) => errors.append(&mut errs),
                }
              }
            }
          }
        }
        name => {
          errors.push(ParseError::UnexpectedChild {
            child_name: name.to_string(),
            span: *node.name().span(),
          });
        }
      }
    }
  }

  if errors.is_empty() {
    Ok(CommandInfo {
      name,
      flags,
      subcommands,
    })
  } else {
    Err(errors)
  }
}

/// `flag_spans` records the spans of all flags for the current command to find
/// duplicates
fn kdl_to_flag(
  node: &KdlNode,
  flag_spans: &mut HashMap<String, SourceSpan>,
) -> std::result::Result<Flag, Vec<ParseError>> {
  let mut forms = vec![];
  let mut desc = None;

  let mut errors = vec![];

  // The name of the node itself will be the first flag
  let first_flag = strip_quotes(&node.name().to_string());
  if let Some(prev_span) = flag_spans.get(&first_flag) {
    errors.push(ParseError::DuplicateFlag {
      flag: first_flag,
      span: *node.name().span(),
      prev_span: *prev_span,
    });
  } else {
    forms.push(first_flag.clone());
    flag_spans.insert(first_flag, *node.name().span());
  }

  // The other flags will be parsed as entries
  for flag_entry in node.entries() {
    if let Some(name) = flag_entry.name() {
      errors.push(ParseError::InvalidFlag {
        msg: format!("entry with name {name}"),
        span: *flag_entry.span(),
      });
    } else if !flag_entry.value().is_string_value() {
      errors.push(ParseError::InvalidFlag {
        msg: flag_entry.to_string(),
        span: *flag_entry.span(),
      });
    } else {
      let flag = strip_quotes(&flag_entry.value().to_string());
      if let Some(prev_span) = flag_spans.get(&flag) {
        errors.push(ParseError::DuplicateFlag {
          flag,
          span: *node.name().span(),
          prev_span: *prev_span,
        });
      } else {
        forms.push(flag.clone());
        flag_spans.insert(flag, *node.name().span());
      }
    }
  }

  if let Some(doc) = node.children() {
    if doc.nodes().len() == 1 {
      let desc_node = &doc.nodes()[0];
      if desc_node.children().is_some() || !desc_node.entries().is_empty() {
        errors.push(ParseError::InvalidDescription(*desc_node.span()));
      } else {
        desc = Some(desc_node.name().value().to_string());
      }
    } else if doc.nodes().len() > 1 {
      errors.push(ParseError::InvalidDescription(*doc.span()));
    }
  }

  if errors.is_empty() {
    Ok(Flag { forms, desc })
  } else {
    Err(errors)
  }
}

/// KDL returns values with quotes around them, so remove those
fn strip_quotes(flag: &str) -> String {
  // todo check if strip_prefix/suffix is the right way to remove the quotes
  // might need to unescape characters within string
  flag
    .strip_prefix('"')
    .and_then(|s| s.strip_suffix('"'))
    .unwrap_or(flag)
    .to_string()
}

#[cfg(test)]
mod tests {
  use super::{parse_from_str, Result};
  use crate::{CommandInfo, Flag};

  #[test]
  fn test1() -> Result<()> {
    assert_eq!(
      CommandInfo {
        name: "foo".to_string(),
        flags: vec![Flag {
          forms: vec!["--help".to_string(), "-h".to_string()],
          desc: Some("Show help output".to_string())
        }],
        subcommands: vec![]
      },
      parse_from_str(
        r#"
        foo {
          flags {
            "--help" "-h" {
              "Show help output"
            }
          }
        }
      "#
      )?
    );
    Ok(())
  }
}
