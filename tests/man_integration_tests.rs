//! Test parsing man pages

use std::{
  env, fs,
  path::PathBuf,
  process::{Command, Stdio},
};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

const BIN_NAME: &str = "gen-completions";

fn run_test(shell: &str, outputs: &[&str], args: &[&str]) {
  // The project's root directory
  let root = env::var("CARGO_MANIFEST_DIR").unwrap();

  let test_resources = PathBuf::from(root).join("tests/resources/man");
  let in_dir = test_resources.join("in");
  let expected_dir = test_resources.join("expected");

  let out_dir = tempfile::tempdir().unwrap();

  // The gen-completions binary to test
  let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
  let cmd = cmd
    .env("MANPATH", &in_dir)
    .arg("man")
    .args(args)
    .arg(shell)
    .arg(&out_dir.path().display().to_string());

  // So we can explicitly ask for logging
  if let Ok(log_level) = env::var("RUST_LOG") {
    cmd.env("RUST_LOG", log_level).stderr(Stdio::inherit());
  }
  cmd.assert().success();

  // Files that didn't get generated
  let mut not_generated = Vec::new();
  // Files that don't match the expected contents
  let mut not_match = Vec::new();

  for file_name in outputs {
    let file_name = match shell {
      "zsh" => format!("_{file_name}.zsh"),
      "bash" => format!("_{file_name}.bash"),
      "nu" => format!("{file_name}-completions.nu"),
      "json" => format!("{file_name}.json"),
      _ => todo!(),
    };

    let exp_file = expected_dir.join(&file_name);
    let got_file = out_dir.path().join(&file_name);
    if !got_file.exists() {
      not_generated.push(file_name);
      continue;
    }

    if exp_file.exists() {
      let expected = fs::read(exp_file).unwrap();
      let got = fs::read(&got_file).unwrap();
      if expected != got {
        not_match.push(file_name);
        continue;
      }
    } else {
      println!("No {file_name} found in expected folder");
      not_match.push(file_name);
      continue;
    }

    // Delete outputted file if it succeeded, since we don't need it anymore
    fs::remove_file(got_file).unwrap();
  }

  if !not_generated.is_empty() {
    println!("The following files weren't generated:");
    for file_name in &not_generated {
      println!("- {file_name}");
    }
  }

  if !not_match.is_empty() {
    // Make a tmp folder to copy the incorrect outputs to, to view later
    let failed_dir = test_resources.join("tmp");
    if !failed_dir.exists() {
      fs::create_dir(&failed_dir).unwrap();
    }

    println!("The following files didn't match what was expected:");
    for file_name in &not_match {
      let exp = expected_dir.join(file_name);
      let exp = exp.to_string_lossy();

      // Copy the incorrect output out of the temp directory
      let saved = failed_dir.join(file_name);
      let got = fs::read(&out_dir.path().join(file_name)).unwrap();
      fs::write(&saved, got).unwrap();

      let saved = saved.display().to_string();
      println!("Test for {file_name} failed: contents of {file_name} differed from expected");
      println!("To see the diff, run `diff {exp} {saved}`");
      println!("To overwrite the expected file, run `cp {saved} {exp}`");
    }
  }

  out_dir.close().unwrap();

  assert!(not_generated.is_empty() && not_match.is_empty());
}

#[test]
fn test1_json() {
  run_test("json", &["test1"], &["--cmds", "^test1"]);
}

#[test]
fn type1_git_json() {
  run_test("json", &["git"], &["--cmds", "^git"]);
}

#[test]
fn type2_rfcomm_json() {
  run_test("json", &["rfcomm"], &["--cmds", "^rfcomm"]);
}

#[test]
fn type3_sed_json() {
  run_test("json", &["sed"], &["--cmds", "^sed"]);
}

#[test]
fn scdoc_sway_json() {
  run_test("json", &["sway"], &["--cmds", "^sway"]);
}

#[test]
fn podman_ncdu_json() {
  run_test("json", &["ncdu"], &["--cmds", "^ncdu"]);
}

#[test]
fn darwin_bless_json() {
  run_test("json", &["bless"], &["--cmds", "^bless"]);
}
