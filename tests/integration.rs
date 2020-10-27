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
        .assert().stdout(predicates::str::contains("hashdeep tool lite version"));

    Ok(())
}

#[test]
fn no_parameters() -> Result<(), Box<dyn std::error::Error>> {

    prints_help_message_fn(&[])

}

#[test]
fn invalid_parameters() -> Result<(), Box<dyn std::error::Error>> {

    prints_help_message_fn(&["nonexistent_command"])?;

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
