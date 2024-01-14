//! Test generating completions from JSON files

use std::{
  env,
  path::{Path, PathBuf},
  process::{Command, Stdio},
};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use insta::Settings;

const BIN_NAME: &str = "gen-completions";

fn run_test(shell: &str, conf: &str, args: &[&str]) {
  // The project's root directory
  let root = env::var("CARGO_MANIFEST_DIR").unwrap();

  let conf_file = PathBuf::from(root).join("tests/resources/gen").join(conf);

  // The gen-completions binary to test
  let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
  let cmd = cmd.arg("for").arg(shell).arg(&conf_file).args(args);
  // So we can explicitly ask for logging
  if let Ok(log_level) = env::var("RUST_LOG") {
    cmd.env("RUST_LOG", log_level).stderr(Stdio::inherit());
  }
  let got = cmd.output().unwrap().stdout;
  let got = std::str::from_utf8(&got).unwrap().trim();
  cmd.assert().success();

  let mut settings = Settings::clone_current();
  settings.set_snapshot_path(Path::new("snapshots/gen/"));
  settings.set_snapshot_suffix(format!("{}.{}", conf, shell));
  settings.set_description(format!(
    "Generated for shell {} using config file {}",
    shell, conf
  ));
  settings.set_input_file(conf_file);
  settings.bind(|| {
    insta::assert_snapshot!(got);
  });
}

#[test]
fn test1_zsh() {
  run_test("zsh", "test1.json", &[]);
}

#[test]
fn test1_bash() {
  run_test("bash", "test1.json", &[]);
}

#[test]
fn test1_nu() {
  run_test("nu", "test1.json", &[]);
}

#[test]
fn test1_kdl() {
  run_test("kdl", "test1.json", &[]);
}

#[test]
fn test1_json() {
  run_test("json", "test1.json", &[]);
}

// #[test]
// fn types_bash() {
//   run_test("bash", "test-types.kdl", &[]);
// }

// #[test]
// fn types_zsh() {
//   run_test("zsh", "test-types.kdl", &[]);
// }

#[test]
fn types_nu() {
  run_test("nu", "test-types.kdl", &[]);
}
