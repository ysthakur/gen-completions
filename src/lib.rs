//! This is a library for generating completions either by parsing manpages or
//! from KDL/JSON/YAML config files. If you're looking for the CLI tool, head to
//! <https://crates.io/crates/gen-completions>
//!
//! The [`parse_man`] module parses manpages, while the [`parse_deser`] module
//! deserializes a KDL/JSON/YAML file to get command information. Both produce
//! [`CommandInfo`]s that can then be used to generate shell completions using
//! the [`gen`] module.

pub mod gen;
pub mod parse_deser;
pub mod parse_man;

use serde::{Deserialize, Serialize};

/// Flags parsed from a command, as well as its parsed subcommands
#[derive(Debug, Deserialize, Eq, Serialize, PartialEq)]
pub struct CommandInfo {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub desc: Option<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub flags: Vec<Flag>,
  /// The types of the arguments to this command
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub args: Vec<ArgType>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub subcommands: Vec<CommandInfo>,
}

/// A parsed flag
#[derive(Debug, Deserialize, Eq, Serialize, PartialEq)]
pub struct Flag {
  /// The different short and long forms of a flag
  pub forms: Vec<String>,
  /// Optional description for the flag
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub desc: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub typ: Option<ArgType>,
}

/// How to complete an argument
#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
pub enum ArgType {
  /// Complete using either file or directory paths
  Path,

  /// Complete using directory paths
  Dir,

  /// Complete by running a command
  Run {
    /// The command to run
    cmd: String,
    /// The separator to split on to get the value (first) and description
    /// (second). If none, assumed to only return values
    sep: Option<String>,
  },

  /// Only these strings are allowed. The second part of each tuple is an
  /// optional description
  Strings(Vec<(String, Option<String>)>),

  /// Complete with the name of a command
  CommandName,

  /// Any of the given types work
  Any(Vec<ArgType>),

  Unknown,
}
