use std::error::Error;
use std::io::{stdout,stderr};
use hashdeep_compare::main_impl::main_io_wrapper;

/**
Runs the program: directs program arguments, stdout, and stderr to main_impl::main_io_wrapper,
and exits with the resulting exit code or error

See main_impl.rs for more details.
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