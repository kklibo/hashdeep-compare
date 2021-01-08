extern crate assert_cmd;

//The results of these integration tests are written to a log that is checked into the repo
// (tests/logged_integration_test_out.txt).
//
// After running these tests, changes in results will be detected when attempting to commit:
// if they are unintended or wrong, the code should be fixed before the commit is done.

use assert_cmd::prelude::*;
use std::process::{Command, Stdio};
use std::fs::File;
use std::io::Write;


#[test]
fn logged_integration_tests() {

    const LOGFILENAME: &str = "tests/logged_integration_test_out.txt";

    let mut logfile = File::create(LOGFILENAME)
        .expect(format!("{} could not be created", LOGFILENAME).as_str());


    let mut run_command = |args: &[&str]| {

        //for convenience, catch errors in a local closure
        || -> Result<(), Box<dyn std::error::Error>> {

            let output =
                Command::cargo_bin(env!("CARGO_PKG_NAME"))?
                    .args(args)
                    .stdin(Stdio::null())
                    .output()?;

            logfile.write_all("[ arguments ]:".as_ref())?;
            for &x in args {
                logfile.write_all(format!(" {}", x).as_ref())?;
            }

            logfile.write_all("\n".as_ref())?;
            logfile.write_all("[ stdout ]\n".as_ref())?;
            logfile.write_all(&output.stdout)?;
            logfile.write_all("[ stderr ]\n".as_ref())?;
            logfile.write_all(&output.stderr)?;
            logfile.write_all(format!("[ return ]: {}", &output.status).as_ref())?;
            logfile.write_all("\n\n".as_ref())?;

            Ok(())
        }
        ().unwrap();
    };



    //use a temp folder with a consistent relative path
    // (inconsistent temp file and folder names would change the test output log)
    let temp_dir  = "tests/temp/";
    let temp_file = "tests/temp/temp_file";
    std::fs::create_dir_all(temp_dir).unwrap();

    //sort subcommand
    {
        let valid_hashdeep_logfile = "tests/test1.txt";

        run_command(&["sort"]);
        run_command(&["sort", "arg2"]);
        run_command(&["sort", "", temp_file]);
        run_command(&["sort", "/dev/null", temp_file]);
        run_command(&["sort", "non-existent_file", temp_file]);
        run_command(&["sort", "non-existent_dir/", temp_file]);
        run_command(&["sort", valid_hashdeep_logfile, ""]);
        run_command(&["sort", valid_hashdeep_logfile, "/dev/null/invalid"]);
        run_command(&["sort", valid_hashdeep_logfile, temp_dir]);
        run_command(&["sort", valid_hashdeep_logfile, temp_file]);
    }

    //part subcommand
    {
        let valid_hashdeep_logfile1 = "tests/partition_test1.txt";
        let valid_hashdeep_logfile2 = "tests/partition_test2.txt";

        //not enough parameters
        run_command(&["part"]);
        run_command(&["part", "arg2", "arg3"]);

        //invalid first logfile
        run_command(&["part", "",                  valid_hashdeep_logfile2, temp_file]);
        run_command(&["part", "/dev/null",         valid_hashdeep_logfile2, temp_file]);
        run_command(&["part", "non-existent_file", valid_hashdeep_logfile2, temp_file]);
        run_command(&["part", "non-existent_dir/", valid_hashdeep_logfile2, temp_file]);
        run_command(&["part", temp_dir           , valid_hashdeep_logfile2, temp_file]);

        //invalid second logfile
        run_command(&["part", valid_hashdeep_logfile1, "",                  temp_file]);
        run_command(&["part", valid_hashdeep_logfile1, "/dev/null",         temp_file]);
        run_command(&["part", valid_hashdeep_logfile1, "non-existent_file", temp_file]);
        run_command(&["part", valid_hashdeep_logfile1, "non-existent_dir/", temp_file]);
        run_command(&["part", valid_hashdeep_logfile1, temp_dir,            temp_file]);

        //invalid output file base
        run_command(&["part", valid_hashdeep_logfile1, valid_hashdeep_logfile2, "/dev/null"  ]);

        //success
        run_command(&["part", valid_hashdeep_logfile1, valid_hashdeep_logfile2, temp_file]);
    }



    //clean up temp files
    std::fs::remove_dir_all(temp_dir).unwrap();

}