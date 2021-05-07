//! The **main_impl** module exists to add I/O redirection to the main program function, to allow
//! test coverage analysis for integration tests.
//!
//! ## Normal Program Behavior
//!
//! *main()* in main.rs calls *main_io_wrapper* in this module, supplying arguments and the standard
//! output and error streams.
//! *main_io_wrapper*, in turn, calls *main_impl*, which is the equivalent of a typical *main()*
//! function.
//!
//! ## Special handling: the **integration_test_coverage** feature
//!
//! Integration tests are defined in tests/integration.rs. These tests invoke the compiled program
//! binary and test its functions with command-line arguments. Because they call a
//! separate binary instead of code in hashdeep-compare's codebase, code coverage tools can't observe
//! what code they use.
//!
//! To work around this, hashdeep-compare has an **integration_test_coverage** feature. When enabled,
//! integration.rs uses a modified test function that runs the same tests as before, but through direct
//! calls in the codebase, instead of through a precompiled binary. The *main_io_wrapper* function is
//! its interface in this mode. These direct calls are part of the test binary, and can be observed by
//! a code coverage tool.
//!
//! ## Justification
//!
//! Separating the program's main function code between main.rs and **main_impl** does add some
//! complexity to the codebase.
//! The option of doing integration testing through direct calls to *main_io_wrapper*, as the
//! **integration_test_coverage** mode does now, would test almost all of the relevant code: the only code bypassed
//! is the code in *main()* in main.rs, which is a minimal wrapper around *main_io_wrapper*. However,
//! actual invocation through a separate binary provides a higher level of certainty against potential
//! future anomalies that disrupt the creation of the binary itself, or the processing of its inputs.
//! Integration testing with binaries is meant to replicate the actual use of the tool as closely as
//! possible: for this reason, the **integration_test_coverage** feature-based mode switch is preferred.

use crate::*;
use std::error::Error;
use std::io::Write;

/// Specifies program arguments and (re)direction of stdout/stderr, then runs the program
///
/// Returns the program's exit code
///
/// This is called by
/// - main() in normal execution
/// - integration.rs when the **integration_test_coverage** feature is enabled
pub fn main_io_wrapper(
    args: &[&str],
    stdout: Box<dyn Write>,
    mut stderr: Box<dyn Write>,
) -> Result<i32, Box<dyn Error>> {

    let exit_code =
    match main_impl(args, stdout)
    {
        Ok(()) => 0,
        Err(err) => {

            //conditionally use Display output for thiserror-based error types
            if let Some(err) = err.downcast_ref::<command::RunHashdeepCommandError>() {
                writeln! (stderr, "Error: \"{}\"", err)?;
            }
            else if let Some(err) = err.downcast_ref::<common::ReadLogEntriesFromFileError>() {
                writeln! (stderr, "Error: \"{}\"", err)?;
            }
            else if let Some(err) = err.downcast_ref::<common::WriteToFileError>() {
                writeln! (stderr, "Error: \"{}\"", err)?;
            }
            else {
                writeln! (stderr, "Error: {:?}", err)?;
            }

            1
        }
    };

    Ok(exit_code)
}

/// Called by main_io_wrapper: Accepts program arguments and runs the program
///
/// (This was the main() function before the **integration_test_coverage** feature was added)
fn main_impl(args: &[&str], mut stdout: Box<dyn Write>) -> Result<(), Box<dyn Error>> {

    const VERSION: &str = env!("CARGO_PKG_VERSION");

    match args.get(1) {

        None | Some(&"help") => {

            let help_string =
            match args.get(2) {
                Some(&"hash") => help::help_hash_string(),
                Some(&"sort") => help::help_sort_string(),
                Some(&"part") => help::help_part_string(),
                _ => help::help_string(VERSION),
            };

            writeln!(stdout, "{}", help_string)?;

        },
        Some(&"hash") => {
            if args.len() < 4 {return Err("hash: not enough arguments".into());}
            if args.len() > 4 {return Err("hash: too many arguments".into());}

            command::run_hashdeep_command(
                args[2],
                args[3],
                "hashdeep")?;
        },
        Some(&"sort") => {
            if args.len() < 4 {return Err("sort: not enough arguments".into());}
            if args.len() > 4 {return Err("sort: too many arguments".into());}

            sort::sort_log(args[2], args[3])?;
        },
        Some(&"part") => {
            if args.len() < 5 {return Err("part: not enough arguments".into());}
            if args.len() > 5 {return Err("part: too many arguments".into());}

            let partition_stats =
            partition::partition_log(args[2], args[3], args[4])?;

            writeln!(stdout, "{}", partition_stats)?;
        },
        Some(&"version") => {
            if args.len() > 2 {return Err("version: does not accept arguments".into());}

            writeln!(stdout, "hashdeep-compare version {}", VERSION)?;
        },

        Some(x) => return Err(format!("invalid command: {}", x).into())
    }

    Ok(())
}