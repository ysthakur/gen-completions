//! For deserializing from KDL, because the serde support is not great

use std::collections::HashMap;

use kdl::{KdlDocument, KdlNode};
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::{ArgType, CommandInfo, Flag};

/// An error encountered when deserializing KDL specifically
#[derive(Debug, Diagnostic, Error)]
pub enum KdlDeserError {
  #[error(transparent)]
  #[diagnostic(transparent)]
  SyntaxError(#[from] kdl::KdlError),

  #[error("File was empty, expected one node")]
  #[diagnostic(code(gen_completions::deser::empty_file), url(docsrs))]
  EmptyFile,

  #[error("Expected exactly one node, got {0}")]
  #[diagnostic(code(gen_completions::deser::too_many_nodes), url(docsrs))]
  TooManyNodes(usize),

  /// The text was valid KDL but could not be read as a [`CommandInfo`]
  #[error("Errors encountered while reading command information")]
  #[diagnostic(
    code(gen_completions::deser::parse_error),
    url(docsrs),
    help("get good")
  )]
  ParseError {
    #[source_code]
    text: String,
    #[related]
    related: Vec<ParseError>,
  },
}

#[derive(Debug, Diagnostic, Error)]
pub enum ParseError {
  #[error("unexpected child {child_name}")]
  #[diagnostic(
    code(gen_completions::deser::kdl::unexpected_child),
    url(docsrs)
  )]
  UnexpectedChild {
    child_name: String,
    allowed: String,
    #[label("only {allowed} are allowed, not {child_name}")]
    span: SourceSpan,
  },

  #[error("duplicate child node")]
  #[diagnostic(
    code(gen_completions::deser::kdl::duplicate_child),
    url(docsrs),
    help("merge the {child_name} nodes together")
  )]
  DuplicateChild {
    child_name: String,
    #[label("duplicate node named {child_name}")]
    span: SourceSpan,
    #[label("already given here")]
    prev_span: SourceSpan,
  },

  #[error("duplicate flag")]
  #[diagnostic(code(gen_completions::deser::kdl::duplicate_flag), url(docsrs))]
  DuplicateFlag {
    flag: String,
    #[label("duplicate flag")]
    span: SourceSpan,
    #[label("already given here")]
    prev_span: SourceSpan,
  },

  #[error("flags should be strings, got {msg}")]
  #[diagnostic(
    code(gen_completions::deser::kdl::invalid_flag),
    url(docsrs),
    help("wrap your flags in quotes")
  )]
  InvalidFlag {
    msg: String,
    #[label("should be a single string")]
    span: SourceSpan,
  },

  #[error("invalid description, expected a single string")]
  #[diagnostic(code(gen_completions::deser::kdl::invalid_desc), url(docsrs))]
  InvalidDescription(#[label("should be a single string")] SourceSpan),

  #[error("type is empty")]
  #[diagnostic(
    code(gen_completions::deser::kdl::empty_type),
    url(docsrs),
    help("remove the type node entirely")
  )]
  EmptyType(#[label("node has no children")] SourceSpan),

  #[error("invalid type")]
  #[diagnostic(code(gen_completions::deser::kdl::invalid_type), url(docsrs))]
  InvalidType(String, #[label("unknown type {0}")] SourceSpan),
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
    kdl_to_cmd_info(&nodes[0]).map_err(|mut errors| KdlDeserError::ParseError {
      text: text.to_string(),
      related: vec![errors.pop().unwrap()],
    })
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
  let args = vec![]; // todo parse arg types at some point
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
                match parse_flag(flag_node, &mut flag_spans) {
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
              child_name: "subcommands".to_string(),
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
            allowed: "flags and subcommands".to_string(),
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
      args,
      subcommands,
    })
  } else {
    Err(errors)
  }
}

/// `flag_spans` records the spans of all flags for the current command to find
/// duplicates
fn parse_flag(
  node: &KdlNode,
  flag_spans: &mut HashMap<String, SourceSpan>,
) -> std::result::Result<Flag, Vec<ParseError>> {
  let mut forms = vec![];
  let mut desc = None;
  let mut typ = None;

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
    let mut first_desc_node = None;
    let mut first_type_node = None;
    for node in doc.nodes() {
      match node.name().to_string().as_str() {
        "desc" => {
          if let Some(prev_span) = first_desc_node {
            errors.push(ParseError::DuplicateChild {
              child_name: "desc".to_string(),
              span: *node.name().span(),
              prev_span,
            });
          } else {
            first_desc_node = Some(*node.name().span());
            if node.entries().len() == 1 {
              // todo account for invalid entry with name
              desc = Some(strip_quotes(&node.entries()[0].value().to_string()));
            } else {
              todo!()
            }
          }
        }
        "type" => {
          if let Some(prev_span) = first_type_node {
            errors.push(ParseError::DuplicateChild {
              child_name: "flags".to_string(),
              span: *node.name().span(),
              prev_span,
            });
          } else {
            first_type_node = Some(*node.name().span());

            if let Some(children) = node.children() {
              let mut types = Vec::new();
              for type_node in children.nodes() {
                let typ = match type_node.name().to_string().as_str() {
                  "path" => Some(ArgType::Path),
                  "dir" => Some(ArgType::Dir),
                  // todo handle other variants
                  typ => {
                    errors.push(ParseError::InvalidType(
                      typ.to_string(),
                      *type_node.name().span(),
                    ));
                    None
                  }
                };
                if let Some(typ) = typ {
                  types.push(typ);
                }
              }
              if types.len() == 1 {
                typ = Some(types.pop().unwrap());
              } else {
                typ = Some(ArgType::Any(types));
              }
            } else {
              errors.push(ParseError::EmptyType(*node.span()));
            }
          }
        }
        name => {
          errors.push(ParseError::UnexpectedChild {
            child_name: name.to_string(),
            allowed: "desc and type".to_string(),
            span: *node.name().span(),
          });
        }
      }
    }
  }

  if errors.is_empty() {
    Ok(Flag { forms, desc, typ })
  } else {
    Err(errors)
  }
}

// fn get_nodes(doc: &KdlDocument, names: &[String]) ->
// std::result::Result<HashMap<String, KdlNode>, Vec<ParseError>> {
//   let mut first_spans = HashMap::new();
//   let mut nodes = HashMap::new();

//   for node in doc.nodes() {
//     let name = node.name().to_string();
//     if names.contains(name) {
//       todo!()
//     }
//   }

//   nodes
// }

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
  use crate::{ArgType, CommandInfo, Flag};

  #[test]
  fn test1() -> Result<()> {
    assert_eq!(
      CommandInfo {
        name: "foo".to_string(),
        flags: vec![Flag {
          forms: vec!["--help".to_string(), "-h".to_string()],
          desc: Some("Show help output".to_string()),
          typ: Some(ArgType::Path),
        }],
        args: vec![],
        subcommands: vec![]
      },
      parse_from_str(
        r#"
        foo {
          flags {
            "--help" "-h" {
              desc "Show help output"
              type path
            }
          }
        }
      "#
      )?
    );
    Ok(())
  }
}
