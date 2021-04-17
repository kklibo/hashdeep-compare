extern crate hashdeep_compare;
use hashdeep_compare::*;

use std::error::Error;
use std::io::{stdout,stderr,Write};

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let return_code=
    main_io_wrapper(
        &args,
        Box::new(stdout()),
        Box::new(stderr()),
    )?;

    std::process::exit(return_code);
}

fn main_io_wrapper(
    args: &[&str],
    stdout: Box<dyn Write>,
    mut stderr: Box<dyn Write>,
) -> Result<i32, Box<dyn Error>> {

    let return_code =
    match main_impl(args, stdout)
    {
        Ok(()) => 0,
        Err(err) => {
            writeln! (stderr, "Error: {:?}", err)?;
            1
        }
    };

    Ok(return_code)
}

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