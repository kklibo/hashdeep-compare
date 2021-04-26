extern crate hashdeep_compare;
use hashdeep_compare::*;

use std::error::Error;
use std::io::{stdout,stderr,Write};

/*!
main.rs has a chain of three main-like functions instead of just main():
- main
- main_io_wrapper
- main_impl

This structure exists to allow test coverage for integration tests.

Integration tests are defined in tests/integration.rs. These tests invoke the compiled program
binary and test its functions with command-line arguments. Because they call a
separate binary instead of code in hashdeep-compare's codebase, code coverage tools can't observe
what code they use.

To work around this, hashdeep-compare has an **integration_test_coverage** feature. When enabled,
integration.rs uses a modified test function that runs the same tests as before, but through direct
calls in the codebase, instead of through a precompiled binary. The **main_io_wrapper** function is
its interface in this mode.

Note: when the **integration_test_coverage** feature is enabled, tests/integration.rs accesses this
file directly through the include! macro. Some special handling may be needed to avoid importing
modules more than once.
**/

/**
Runs the program normally: directs program arguments, stdout, and stderr to main_io_wrapper,
and exits with the resulting exit code or error
**/
fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let exit_code=
    main_io_wrapper(
        &args,
        Box::new(stdout()),
        Box::new(stderr()),
    )?;

    std::process::exit(exit_code);
}

/**
Specifies program arguments and (re)direction of stdout/stderr, then runs the program

Returns the program's exit code

This is called by
- main() in normal execution
- integration.rs when the **integration_test_coverage** feature is enabled
**/
fn main_io_wrapper(
    args: &[&str],
    stdout: Box<dyn Write>,
    mut stderr: Box<dyn Write>,
) -> Result<i32, Box<dyn Error>> {

    let exit_code =
    match main_impl(args, stdout)
    {
        Ok(()) => 0,
        Err(err) => {
            writeln! (stderr, "Error: {:?}", err)?;
            1
        }
    };

    Ok(exit_code)
}

/**
Called by main_io_wrapper: Accepts program arguments and runs the program

(This was the main() function before the **integration_test_coverage** feature was added)
**/
fn main_impl(args: &[&str], mut stdout: Box<dyn Write>) -> Result<(), Box<dyn Error>> {

    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let mut show_help = || -> Result<(), Box<dyn Error>> {
        writeln!(stdout, "hashdeep-compare version {}", VERSION)?;
        writeln!(stdout, " arguments")?;
        writeln!(stdout, "  version")?;
        writeln!(stdout, "  hash target_directory output_path_base")?;
        writeln!(stdout, "  sort input_file output_file")?;
        writeln!(stdout, "  part input_file1 input_file2 output_file_base")?;
        Ok(())
    };

    if args.len() < 2 {
        show_help()?;
        return Ok(());
    }


    match args[1] {
        "hash" => {
            if args.len() < 4 {return Err("hash: not enough arguments".into());}

            command::run_hashdeep_command(
                args[2],
                args[3])?;
        },
        "sort" => {
            if args.len() < 4 {return Err("sort: not enough arguments".into());}

            sort::sort_log(args[2], args[3])?;
        },
        "part" => {
            if args.len() < 5 {return Err("part: not enough arguments".into());}

            let partition_stats =
            partition::partition_log(args[2], args[3], args[4])?;

            writeln!(stdout, "{}", partition_stats)?;
        },
        "version" => {
            writeln!(stdout, "hashdeep-compare version {}", VERSION)?;
        },

        x => return Err(format!("invalid command: {}", x).into())
    }

    Ok(())
}