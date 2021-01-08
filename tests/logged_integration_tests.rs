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

    //part subcommand
    {
        let valid_hashdeep_logfile1 = "tests/partition_test1.txt";
        let valid_hashdeep_logfile2 = "tests/partition_test2.txt";

        //success
    }



    //clean up temp files
    std::fs::remove_dir_all(temp_dir).unwrap();

}