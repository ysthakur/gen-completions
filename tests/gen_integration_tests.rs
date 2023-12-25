//! Test generating completions from JSON files

use std::{
  env, fs,
  path::PathBuf,
  process::{Command, Stdio},
};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

const BIN_NAME: &str = "gen-completions";

fn run_test(shell: &str, conf: &str, args: &[&str]) {
  // The project's root directory
  let root = env::var("CARGO_MANIFEST_DIR").unwrap();

  let test_resources = PathBuf::from(root).join("tests/resources/gen");
  let in_dir = test_resources.join("in");
  let expected_dir = test_resources.join("expected");

  let out_dir = tempfile::tempdir().unwrap();

  // The gen-completions binary to test
  let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
  let cmd = cmd
    .arg("for")
    .arg(shell)
    .arg(in_dir.join(conf))
    .args(args);
  // So we can explicitly ask for logging
  if let Ok(log_level) = env::var("RUST_LOG") {
    cmd.env("RUST_LOG", log_level).stderr(Stdio::inherit());
  }
  let got = cmd.output().unwrap().stdout;
  let got = std::str::from_utf8(&got).unwrap().trim();
  cmd.assert().success();

  let cmd_name = conf.split(".").next().unwrap();
  let file_name = match shell {
    "zsh" => format!("_{cmd_name}.zsh"),
    "bash" => format!("_{cmd_name}.bash"),
    "nu" => format!("{cmd_name}-completions.nu"),
    "fish" => format!("_{cmd_name}.fish"), // someday
    "json" => format!("{cmd_name}.json"),
    "kdl" => format!("{cmd_name}.kdl"),
    "yaml" => format!("{cmd_name}.yaml"),
    _ => todo!(),
  };

  let expected_path = expected_dir.join(&file_name);
  let expected_out = fs::read(&expected_path).unwrap();
  let expected_out = std::str::from_utf8(&expected_out).unwrap().trim();

  if got != expected_out {
    // Make a tmp folder to copy the incorrect outputs to, to view later
    let failed_dir = test_resources.join("tmp");
    if !failed_dir.exists() {
      fs::create_dir(&failed_dir).unwrap();
    }

    // Copy the incorrect output out of the temp directory
    let saved = failed_dir.join(file_name);
    fs::write(&saved, got).unwrap();

    let saved = saved.display().to_string();
    println!("Test for {} failed.", cmd_name);
    println!("To see the diff, run `diff {} {}`", &expected_path.display(), saved);
    println!("To overwrite the expected file, run `cp {} {}`", saved, expected_path.display());

    assert!(false);
  }

  out_dir.close().unwrap();
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
