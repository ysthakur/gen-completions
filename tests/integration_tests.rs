use std::{env, fs, path::PathBuf, process::Command};

#[test]
fn test() {
  // The project's root directory
  let root = env::var("CARGO_MANIFEST_DIR").unwrap();

  let test_resources = PathBuf::from(root).join("tests/resources");
  let in_dir = test_resources.join("in");
  let expected_dir = test_resources.join("expected");
  let out_dir = test_resources.join("tmp");

  // The man-completions binary to test
  let bin = env::var("CARGO_BIN_EXE_man-completions").unwrap();
  let status = Command::new(bin).env("MANPATH", in_dir).status().unwrap();
  assert!(status.success());

  // Files that didn't get generated
  let mut not_generated = Vec::new();
  // Files that don't match the expected contents
  let mut not_match = Vec::new();
  for exp_file in fs::read_dir(&expected_dir).unwrap() {
    let exp_file = exp_file.unwrap();
    let file_name = exp_file.file_name().to_string_lossy().to_string();
    let got_file = out_dir.join(&file_name);
    if !got_file.exists() {
      not_generated.push(file_name);
      continue;
    }

    let expected = fs::read(exp_file.path()).unwrap();
    let got = fs::read(got_file).unwrap();
    if expected != got {
      not_match.push(file_name);
    }
  }

  if !not_generated.is_empty() {
    println!("The following files weren't generated:");
    for file_name in &not_generated {
      println!("- {file_name}");
    }
  }

  if !not_match.is_empty() {
    println!("The following files didn't match what was expected:");
    for file_name in &not_match {
      let exp = expected_dir.join(&file_name);
      let exp = exp.to_string_lossy();
      let got = out_dir.join(&file_name);
      let got = got.to_string_lossy();
      println!("Test for {file_name} failed: contents of {got} did not match those of {exp}");
      println!("To see the diff, run `diff {exp} {got}`");
      println!("To overwrite the expected file, run `cp {got} {exp}`")
    }
  }

  if !not_generated.is_empty() || !not_match.is_empty() {
    assert!(false);
  }
}
