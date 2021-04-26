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