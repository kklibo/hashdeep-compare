extern crate assert_cmd;
extern crate predicates;
extern crate tempfile;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::NamedTempFile;

use std::process::Command;
use std::path::Path;


const BIN_NAME: &str = env!("CARGO_PKG_NAME");

fn prints_help_message_fn(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {

    Command::cargo_bin(BIN_NAME)?
        .args(args)
        .assert().stdout(predicates::str::contains("hashdeep-compare version"));

    Ok(())
}

#[test]
fn no_parameters() -> Result<(), Box<dyn std::error::Error>> {

    prints_help_message_fn(&[])

}

#[test]
fn invalid_parameters() -> Result<(), Box<dyn std::error::Error>> {

    Command::cargo_bin(BIN_NAME)?
        .arg("nonexistent_command")
        .assert().stderr(predicates::str::contains("invalid command"));

    Ok(())
}


fn sort_success_fn(in_file: &str, expected_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    let temp_file = NamedTempFile::new()?;

    Command::cargo_bin(BIN_NAME)?
        .arg("sort")
        .arg(in_file)
        .arg(temp_file.path())
        .assert().success();

    let p = predicates::path::eq_file(Path::new(expected_file));
    assert!(p.eval(temp_file.path()));

    Ok(())
}


#[test]
fn sort_success() -> Result<(), Box<dyn std::error::Error>> {

    sort_success_fn("tests/test1.txt", "tests/test1 sorted.txt")?;

    Ok(())
}


#[test]
fn hash_success() -> Result<(), Box<dyn std::error::Error>> {

    let expected_result_path = Path::new("tests/hashdeep_result.txt");
    let target_path = "tests/hashdeep_target";

    let temp_dir = tempfile::TempDir::new()?;
    let temp_file_path = temp_dir.path().join("test1");


    Command::cargo_bin(BIN_NAME)?
        .arg("hash")
        .arg(target_path)
        .arg(&temp_file_path)
        .assert().success();

    //special comparison:
    // skip the third line of the header: it contains the invocation directory
    // and will be (correctly) inconsistent between runs

    let     test_file_string = std::fs::read_to_string(&      temp_file_path)?;
    let expected_file_string = std::fs::read_to_string(&expected_result_path)?;

    let test_lines =
        test_file_string    .lines().take(2).chain(test_file_string    .lines().skip(3));

    let expected_lines =
        expected_file_string.lines().take(2).chain(expected_file_string.lines().skip(3));

    assert!(test_lines.zip(expected_lines).all(|(a,b)| a == b));

    temp_dir.close()?;

    Ok(())
}
