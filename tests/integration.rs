extern crate assert_cmd;
extern crate predicates;
extern crate tempfile;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::NamedTempFile;

use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::io::Write;


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


#[test]
//todo: rename this function?
fn structured_integration_tests() -> Result<(), Box<dyn std::error::Error>> {

    //remove existing test results
    std::fs::remove_dir_all("tests/expected")?;


    //hash subcommand tests
    run_test("hash/0_arguments",    &["hash"])?;
    run_test("hash/1_argument",     &["hash", "arg1"])?;

    run_test("hash/target_dir/empty",       &["hash", "",               "./hashlog"])?;
    run_test("hash/target_dir/invalid",     &["hash", "/dev/null",      "./hashlog"])?;
    run_test("hash/target_dir/nonexistent", &["hash", "does_not_exist/","./hashlog"])?;

    run_test("hash/output_path_base/invalid", &["hash", ".", "/dev/null"])?;


    //sort subcommand tests
    run_test("sort/0_arguments",    &["sort"])?;
    run_test("sort/1_argument",     &["sort", "arg1"])?;

    run_test("sort/input_file/empty",       &["sort", "",               "sorted"])?;
    run_test("sort/input_file/invalid",     &["sort", "/dev/null",      "sorted"])?;
    run_test("sort/input_file/nonexistent", &["sort", "does_not_exist", "sorted"])?;

    run_test("sort/success", &["sort", "../../../../test1.txt", "test1_sorted.txt"])?;


    //part subcommand tests
    run_test("part/1_full_match_pair", &["part",
        "../../../../part_files/1_full_match_pair_file1",
        "../../../../part_files/1_full_match_pair_file2",
        "part"
    ])?;

    fn run_test (subdir: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let expected_files =
            Path::new("tests/expected")
                .join(subdir);

        let outfiles = expected_files.join("outfiles");
        let stdout_path = expected_files.join("stdout");
        let stderr_path = expected_files.join("stderr");
        let exitcode_path = expected_files.join("exitcode");


        std::fs::create_dir_all(&outfiles)?;

        let output =
            Command::cargo_bin(BIN_NAME)?
                .current_dir(outfiles.as_path())
                .args(args)
                .output()?;

        let mut stdout_file = File::create(stdout_path.as_path())?;
        stdout_file.write_all(&output.stdout)?;

        let mut stderr_file = File::create(stderr_path.as_path())?;
        stderr_file.write_all(&output.stderr)?;


        //remove empty outputs
        let _ = std::fs::remove_dir(outfiles.as_path()); //will fail if not empty

        if std::fs::metadata(&stdout_path)?.len() == 0 {
            std::fs::remove_file(&stdout_path)?;
        }
        if std::fs::metadata(&stderr_path)?.len() == 0 {
            std::fs::remove_file(&stderr_path)?;
        }


        let mut exitcode_file = File::create(exitcode_path.as_path())?;
        write!(exitcode_file, "{:?}", output.status.code())?;

        Ok(())
    }

    Ok(())
}