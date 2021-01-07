extern crate assert_cmd;
extern crate predicates;
extern crate tempfile;
extern crate pathdiff;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::NamedTempFile;

use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use pathdiff::diff_paths;


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


    /*
    each test writes its results to its own named folder under tests/expected/:
        [name]/stdout :     stdout (exists IFF non-empty)
        [name]/stderr :     stderr (exists IFF non-empty)
        [name]/outfiles/ :  contains output files (exists IFF non-empty)
        [name]/exitcode :   contains exit code (as text)
    */


    run_test("version", &["version"])?;


    //invalid subcommand tests
    run_test("invalid/0_arguments",             &[""])?;
    run_test("invalid/nonexistent_subcommand",  &["nonexistent_subcommand"])?;


    //hash subcommand tests
    run_test("hash/0_arguments",    &["hash"])?;
    run_test("hash/1_argument",     &["hash", "arg1"])?;

    run_test("hash/target_dir/empty",       &["hash", "",               "./hashlog"])?;
    run_test("hash/target_dir/invalid",     &["hash", "/dev/null",      "./hashlog"])?;
    run_test("hash/target_dir/nonexistent", &["hash", "does_not_exist/","./hashlog"])?;

    {
        let rel_path = relative_path(
            &path_in_tests("test1.txt"),
            &path_in_tests("expected/hash/target_dir/is_file/outfiles")
        );
        run_test("hash/target_dir/is_file", &["hash", &rel_path, "hashlog"])?;

        remove_hashdeep_log_header_invocation_path("tests/expected/hash/target_dir/is_file/outfiles/hashlog");
    }

    run_test("hash/output_path_base/invalid",         &["hash", ".", "/dev/null"])?;
    run_test("hash/output_path_base/nonexistent_dir", &["hash", ".", "does_not_exist/hash"])?;

    create_path_and_file("tests/expected/hash/output_path_base/log_file_exists/outfiles/hashlog", "");
    run_test("hash/output_path_base/log_file_exists", &["hash", ".", "hashlog"])?;

    create_path_and_file("tests/expected/hash/output_path_base/log_error_file_exists/outfiles/hashlog.errors", "");
    run_test("hash/output_path_base/log_error_file_exists", &["hash", ".", "hashlog"])?;

    create_path_and_file("tests/expected/hash/output_path_base/log_file_and_error_file_exist/outfiles/hashlog", "");
    create_path_and_file("tests/expected/hash/output_path_base/log_file_and_error_file_exist/outfiles/hashlog.errors", "");
    run_test("hash/output_path_base/log_file_and_error_file_exist", &["hash", ".", "hashlog"])?;


    //sort subcommand tests
    run_test("sort/0_arguments",    &["sort"])?;
    run_test("sort/1_argument",     &["sort", "arg1"])?;

    run_test("sort/input_file/empty",       &["sort", "",               "sorted"])?;
    run_test("sort/input_file/invalid",     &["sort", "/dev/null",      "sorted"])?;
    run_test("sort/input_file/nonexistent", &["sort", "does_not_exist", "sorted"])?;

    run_test("sort/output_file/empty",       &["sort", &path_in_tests("test1.txt"), ""                     ])?;
    run_test("sort/output_file/invalid",     &["sort", &path_in_tests("test1.txt"), "/dev/null/invalid"    ])?;
    run_test("sort/output_file/nonexistent", &["sort", &path_in_tests("test1.txt"), "does_not_exist/sorted"])?;

    run_test("sort/success", &["sort", &path_in_tests("test1.txt"), "test1_sorted.txt"])?;


    //part subcommand tests
    run_test("part/0_arguments",    &["part"])?;
    run_test("part/1_argument",     &["part", "arg1"])?;
    run_test("part/2_arguments",    &["part", "arg1", "arg2"])?;

    run_test("part/input_file1/empty",       &["part", "",                  &path_in_tests("partition_test2.txt"), "part"])?;
    run_test("part/input_file1/invalid",     &["part", "/dev/null/invalid", &path_in_tests("partition_test2.txt"), "part"])?;
    run_test("part/input_file1/nonexistent", &["part", "does_not_exist",    &path_in_tests("partition_test2.txt"), "part"])?;

    run_test("part/input_file2/empty",       &["part", &path_in_tests("partition_test1.txt"), "",                  "part"])?;
    run_test("part/input_file2/invalid",     &["part", &path_in_tests("partition_test1.txt"), "/dev/null/invalid", "part"])?;
    run_test("part/input_file2/nonexistent", &["part", &path_in_tests("partition_test1.txt"), "does_not_exist",    "part"])?;

    run_test("part/output_file_base/empty",       &["part", &path_in_tests("partition_test1.txt"), &path_in_tests("partition_test2.txt"), ""])?;
    run_test("part/output_file_base/invalid",     &["part", &path_in_tests("partition_test1.txt"), &path_in_tests("partition_test2.txt"), "/dev/null"])?;
    run_test("part/output_file_base/nonexistent", &["part", &path_in_tests("partition_test1.txt"), &path_in_tests("partition_test2.txt"), "does_not_exist/part"])?;

    fn part_test(testname: &str) -> Result<(), Box<dyn std::error::Error>> {
        run_test(format!("part/{}", testname).as_str(), &["part",
            &path_in_tests(&format!("part_files/{}_file1", testname)),
            &path_in_tests(&format!("part_files/{}_file2", testname)),
            "part"
        ])
    }

    part_test("1_full_match_pair")?;
    part_test("1_full_match_group_in_file1_only")?;
    part_test("1_full_match_group_in_file2_only")?;
    part_test("1_full_match_group_in_both_files")?;

    part_test("1_name_match_pair")?;
    part_test("1_name_match_group_in_file1_only")?;
    part_test("1_name_match_group_in_file2_only")?;
    part_test("1_name_match_group_in_both_files")?;

    part_test("1_hashes_match_pair")?;
    part_test("1_hashes_match_group_in_file1_only")?;
    part_test("1_hashes_match_group_in_file2_only")?;
    part_test("1_hashes_match_group_in_both_files")?;

    part_test("1_entry_in_file1_with_no_match")?;
    part_test("1_entry_in_file2_with_no_match")?;

    part_test("no_entries")?;
    part_test("no_matches")?;
    part_test("file_move")?;
    part_test("file_rename")?;
    part_test("dir_move")?;
    part_test("dir_rename")?;
    part_test("file_edit")?;
    part_test("file_create")?;
    part_test("file_delete")?;


    //multiple-command tests
    //hash then sort (guarantees ordering stability for nontrivial hash target)
    run_test("multi/hash_then_sort/success",       &["hash", "../../../../../hashdeep_target_nontrivial", "hashlog"])?;
    run_test("multi/hash_then_sort/success",       &["sort", "hashlog", "hashlog_sorted"])?;
    //remove nondeterministic intermediate result
    std::fs::remove_file("tests/expected/multi/hash_then_sort/success/outfiles/hashlog")?;



    //utility functions

    fn path_in_tests(relative: &str) -> String{
        let tests_path = std::fs::canonicalize("tests/").unwrap();
        tests_path.join(relative).into_os_string().into_string().unwrap()
    }

    //relative paths can be used to avoid writing environment-specific paths in hashdeep logs in repo
    fn relative_path(target: &str, base: &str) -> String{
        let path = diff_paths(target, base).unwrap();
        path.into_os_string().into_string().unwrap()
    }

    //removes environment-specific invocation path information from a hashdeep log header
    // this allows consistent file contents regardless of where tests are run from
    fn remove_hashdeep_log_header_invocation_path(target_path: &str) {

        let file_string= std::fs::read_to_string(target_path).unwrap();
        let mut lines: Vec<_> = file_string.split("\n").collect();

        let invocation_path_line = lines.get_mut(2).unwrap();

        assert!( invocation_path_line.starts_with("## Invoked from: ") );

        *invocation_path_line = "## Invoked from: [path removed by hashdeep-compare test]";

        std::fs::write(target_path, lines.join("\n")).unwrap();
    }

    fn create_path_and_file(target_path: &str, contents: &str) {
        std::fs::create_dir_all( Path::new(target_path).parent().unwrap() ).unwrap();
        std::fs::write(target_path, contents).unwrap();
    }

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