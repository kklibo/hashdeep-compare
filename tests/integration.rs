/*!
## Integration Tests: Overview
This file implements an integration test system for hashdeep-compare.

*run_test* function calls define the tests: each one invokes a separate execution of
the program and records its results.

Normally, the program is tested as a separate binary. The **integration_test_coverage** feature
modifies how integration tests are invoked to allow code coverage analysis. Test results are
intended to be identical with and without this feature.

## Tests

A test is defined by
- a set of command line arguments with which to run the program
- a unique results subdirectory (in the project's tests/expected/ directory)

Each test is run with its specified command line arguments, and its outputs are saved in its
results subdirectory.


Test result subdirectories have the following structure:
- stdout: a file containing the program's stdout output
- stderr: a file containing the program's stderr output
- exitcode: a file containing the program's exit code, as a string representation of an
Option\<i32\>
- outfiles: a directory containing the files created by the program in its working directory

Any file or directory which would be empty (e.g.: stderr after a run with no errors)
is not generated.

Test command line arguments often include references to input files in the tests/ directory.

## Test results are version-controlled

The test result subdirectories are checked into the project repository along with the code. This
means that after the tests are run, any changes in results since the last commit will be visible
and must be approved (or fixed) as part of the commit/code review process. The first action after
the integration test is started is the deletion of all existing test results: this means that all
tests must be run to completion with the expected results to return the tests/expected/
subdirectory to the state that matches the repo.

### Disadvantage: determinism required

One problem with this approach is that all tests must generate the same output every time.
This is an issue for tests that use the **hash** option: multithreaded hashdeep does not
consistently order its output lines. The integration tests currently work around this by testing
the **hash** and **sort** options together: a hash log is generated, and then sorted, thus
rendering it deterministic.

## Special handling: the **integration_test_coverage** feature

When the **integration_test_coverage** feature is enabled, the *run_test* function runs tests
through function calls in the codebase, rather than by invoking a separate binary. This allows a
code coverage tool to observe the integration tests' use of the codebase directly. When this
feature is enabled, *run_test* uses the *run_coverage_test* function instead of the normal
*run_bin_test* function.

See main_impl.rs for more details.
**/


extern crate assert_cmd;
extern crate pathdiff;

use assert_cmd::prelude::*;

use std::process::Command;
use std::path::Path;
use std::fs::File;
use pathdiff::diff_paths;

#[cfg(feature = "integration_test_coverage")]
use std::path::PathBuf;


const BIN_NAME: &str = env!("CARGO_PKG_NAME");


#[cfg(feature = "integration_test_coverage")]
use hashdeep_compare::main_impl::main_io_wrapper;





#[test]
//todo: rename this function?
fn structured_integration_tests() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;


    #[cfg(feature = "integration_test_coverage")]
    let initial_working_dir = std::env::current_dir()
        .expect("Failed to start tests: could not read working directory");

    #[cfg(feature = "integration_test_coverage")]
    let run_test =  |subdir: &str, args: &[&str]| -> Result<(), Box<dyn std::error::Error>> {

        run_coverage_test(subdir, args, &initial_working_dir)
    };

    #[cfg(not(feature = "integration_test_coverage"))]
    let run_test =  |subdir: &str, args: &[&str]| -> Result<(), Box<dyn std::error::Error>> {

        run_bin_test(subdir, args)
    };


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
    run_test("invalid/0_arguments",             &[])?;
    run_test("invalid/empty_argument",          &[""])?;
    run_test("invalid/nonexistent_subcommand",  &["nonexistent_subcommand"])?;


    //hash subcommand tests
    run_test("hash/0_arguments",    &["hash"])?;
    run_test("hash/1_argument",     &["hash", "arg1"])?;

    run_test("hash/target_dir/empty",           &["hash", "",               "./hashlog"])?;
    run_test("hash/target_dir/invalid",         &["hash", "/dev/null",      "./hashlog"])?;
    run_test("hash/target_dir/nonexistent_file",&["hash", "does_not_exist ","./hashlog"])?;
    run_test("hash/target_dir/nonexistent_dir", &["hash", "does_not_exist/","./hashlog"])?;

    {
        let rel_path = relative_path(
            &path_in_tests("test1.txt"),
            &path_in_tests("expected/hash/target_dir/is_file/outfiles")
        );
        run_test("hash/target_dir/is_file", &["hash", &rel_path, "hashlog"])?;

        remove_hashdeep_log_header_invocation_path("tests/expected/hash/target_dir/is_file/outfiles/hashlog");
    }

    run_test("hash/output_path_base/empty",           &["hash", ".", ""])?;
    run_test("hash/output_path_base/invalid",         &["hash", ".", "/dev/null/invalid"])?;
    run_test("hash/output_path_base/nonexistent_dir", &["hash", ".", "does_not_exist/hash"])?;
    run_test("hash/output_path_base/in_target_dir",   &["hash", ".", "hashlog"])?;

    create_path_and_file("tests/expected/hash/output_path_base/log_file_exists/outfiles/hashlog", "");
    run_test("hash/output_path_base/log_file_exists", &["hash", ".", "hashlog"])?;

    create_path_and_file("tests/expected/hash/output_path_base/log_error_file_exists/outfiles/hashlog.errors", "");
    run_test("hash/output_path_base/log_error_file_exists", &["hash", ".", "hashlog"])?;

    create_path_and_file("tests/expected/hash/output_path_base/log_file_and_error_file_exist/outfiles/hashlog", "");
    create_path_and_file("tests/expected/hash/output_path_base/log_file_and_error_file_exist/outfiles/hashlog.errors", "");
    run_test("hash/output_path_base/log_file_and_error_file_exist", &["hash", ".", "hashlog"])?;

    {
        let rel_path = relative_path(
            &path_in_tests("hashdeep_target"),
            &path_in_tests("expected/hash/success/outfiles")
        );
        run_test("hash/success", &["hash", &rel_path, "hashlog"])?;

        remove_hashdeep_log_header_invocation_path("tests/expected/hash/success/outfiles/hashlog");
    }


    //sort subcommand tests
    run_test("sort/0_arguments",    &["sort"])?;
    run_test("sort/1_argument",     &["sort", "arg1"])?;

    run_test("sort/input_file/empty",           &["sort", "",                   "sorted"])?;
    run_test("sort/input_file/invalid",         &["sort", "/dev/null/invalid",  "sorted"])?;
    run_test("sort/input_file/nonexistent_file",&["sort", "does_not_exist",     "sorted"])?;
    run_test("sort/input_file/nonexistent_dir", &["sort", "does_not_exist/",    "sorted"])?;

    run_test("sort/output_file/empty",           &["sort", &path_in_tests("test1.txt"), ""                     ])?;
    run_test("sort/output_file/invalid",         &["sort", &path_in_tests("test1.txt"), "/dev/null/invalid"    ])?;
    run_test("sort/output_file/nonexistent_dir", &["sort", &path_in_tests("test1.txt"), "does_not_exist/sorted"])?;
    run_test("sort/output_file/is_dir",          &["sort", &path_in_tests("test1.txt"), "dir/"])?;

    create_path_and_file("tests/expected/sort/output_file/exists/outfiles/sorted", "");
    run_test("sort/output_file/exists",          &["sort", &path_in_tests("test1.txt"), "sorted"])?;

    create_path_and_file("tests/expected/sort/input_file_is_output_file/outfiles/same_file", "");
    run_test("sort/input_file_is_output_file", &["sort", "same_file", "same_file"])?;

    run_test("sort/success", &["sort", &path_in_tests("test1.txt"), "test1_sorted.txt"])?;


    //part subcommand tests
    run_test("part/0_arguments",    &["part"])?;
    run_test("part/1_argument",     &["part", "arg1"])?;
    run_test("part/2_arguments",    &["part", "arg1", "arg2"])?;

    run_test("part/input_file1/empty",              &["part", "",                  &path_in_tests("partition_test2.txt"), "part"])?;
    run_test("part/input_file1/invalid",            &["part", "/dev/null/invalid", &path_in_tests("partition_test2.txt"), "part"])?;
    run_test("part/input_file1/nonexistent_file",   &["part", "does_not_exist",    &path_in_tests("partition_test2.txt"), "part"])?;
    run_test("part/input_file1/nonexistent_dir",    &["part", "does_not_exist/",   &path_in_tests("partition_test2.txt"), "part"])?;
    run_test("part/input_file1/is_dir",             &["part", ".",                 &path_in_tests("partition_test2.txt"), "part"])?;

    run_test("part/input_file2/empty",              &["part", &path_in_tests("partition_test1.txt"), "",                  "part"])?;
    run_test("part/input_file2/invalid",            &["part", &path_in_tests("partition_test1.txt"), "/dev/null/invalid", "part"])?;
    run_test("part/input_file2/nonexistent_file",   &["part", &path_in_tests("partition_test1.txt"), "does_not_exist",    "part"])?;
    run_test("part/input_file2/nonexistent_dir",    &["part", &path_in_tests("partition_test1.txt"), "does_not_exist/",   "part"])?;
    run_test("part/input_file2/is_dir",             &["part", &path_in_tests("partition_test1.txt"), ".",                 "part"])?;

    run_test("part/output_file_base/empty",       &["part", &path_in_tests("partition_test1.txt"), &path_in_tests("partition_test2.txt"), ""])?;
    run_test("part/output_file_base/invalid",     &["part", &path_in_tests("partition_test1.txt"), &path_in_tests("partition_test2.txt"), "/dev/null/invalid"])?;
    run_test("part/output_file_base/nonexistent", &["part", &path_in_tests("partition_test1.txt"), &path_in_tests("partition_test2.txt"), "does_not_exist/part"])?;

    create_path_and_copy_file("tests/part_files/general_test_file1", "tests/expected/part/output_file_base/is_input_file1/outfiles/test");
    run_test("part/output_file_base/is_input_file1", &["part", "test", &path_in_tests("part_files/general_test_file2"), "test"])?;

    create_path_and_copy_file("tests/part_files/general_test_file2", "tests/expected/part/output_file_base/is_input_file2/outfiles/test");
    run_test("part/output_file_base/is_input_file2", &["part", &path_in_tests("part_files/general_test_file1"), "test", "test"])?;

    run_test("part/input_file1_is_input_file2", &["part", &path_in_tests("test1.txt"), &path_in_tests("test1.txt"), "part"])?;

    let part_test = |testname: &str| -> Result<(), Box<dyn std::error::Error>> {
        run_test(format!("part/{}", testname).as_str(), &["part",
            &path_in_tests(&format!("part_files/{}_file1", testname)),
            &path_in_tests(&format!("part_files/{}_file2", testname)),
            "part"
        ])
    };

    part_test("general_test")?;

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

    fn create_path_and_copy_file(source_path: &str, target_path: &str) {
        std::fs::create_dir_all( Path::new(target_path).parent().unwrap() ).unwrap();
        std::fs::copy(source_path, target_path).unwrap();
    }

    #[cfg(not(feature = "integration_test_coverage"))]
    fn run_bin_test (subdir: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
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

    #[cfg(feature = "integration_test_coverage")]
    fn run_coverage_test (subdir: &str, args: &[&str], initial_working_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let expected_files =
            Path::new("tests/expected")
                .join(subdir);

        let outfiles = expected_files.join("outfiles");
        let stdout_path = expected_files.join("stdout");
        let stderr_path = expected_files.join("stderr");
        let exitcode_path = expected_files.join("exitcode");


        std::fs::create_dir_all(&outfiles)?;

        //prepend an element to shift the indices to the right:
        // main_io_wrapper expects arguments to start at index 1
        let mut padded_args = args.to_vec();
            padded_args.insert(0, "");

        let args = &padded_args;


        assert!( initial_working_dir.is_absolute(),
            "test aborted: initial_working_dir path must be absolute (could reset to wrong directory)");

        assert!( ! outfiles.is_absolute(),
            "test aborted: outfiles path should not be absolute (could escape test directory)");

        std::env::set_current_dir(&initial_working_dir)?;

        let mut stdout_file = File::create(stdout_path.as_path())?;
        let mut stderr_file = File::create(stderr_path.as_path())?;


        let working_dir = initial_working_dir.join(&outfiles);
        std::env::set_current_dir(&working_dir)?;

        let exit_code = main_io_wrapper(
            args,
            Box::new(stdout_file),
            Box::new(stderr_file),
        )?;

        std::env::set_current_dir(&initial_working_dir)?;

        //remove empty outputs
        let _ = std::fs::remove_dir(outfiles.as_path()); //will fail if not empty

        if std::fs::metadata(&stdout_path)?.len() == 0 {
            std::fs::remove_file(&stdout_path)?;
        }
        if std::fs::metadata(&stderr_path)?.len() == 0 {
            std::fs::remove_file(&stderr_path)?;
        }


        let mut exitcode_file = File::create(exitcode_path.as_path())?;
        //put exit code in Some() to match expected std::process::ExitStatus::code output
        write!(exitcode_file, "{:?}", Some(exit_code))?;

        Ok(())
    }

    Ok(())
}