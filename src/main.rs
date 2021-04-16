extern crate hashdeep_compare;
use hashdeep_compare::*;

use std::error::Error;
use std::io::{stdout,stderr,Write};

fn main() -> Result<(), Box<dyn Error>> {
    main_impl(Box::new(stdout()))
}

fn main_impl(mut stdout: Box<dyn Write>) -> Result<(), Box<dyn Error>> {

    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let mut show_help = ||{
        writeln!(stdout, "hashdeep-compare version {}", VERSION);
        writeln!(stdout, " arguments");
        writeln!(stdout, "  version");
        writeln!(stdout, "  hash target_directory output_path_base");
        writeln!(stdout, "  sort input_file output_file");
        writeln!(stdout, "  part input_file1 input_file2 output_file_base");
    };

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        show_help();
        return Ok(());
    }


    match args[1].as_str() {
        "hash" => {
            if args.len() < 4 {return Err("hash: not enough arguments".into());}

            command::run_hashdeep_command(
                args[2].as_str(),
                args[3].as_str())?;
        },
        "sort" => {
            if args.len() < 4 {return Err("sort: not enough arguments".into());}

            sort::sort_log(args[2].as_str(), args[3].as_str())?;
        },
        "part" => {
            if args.len() < 5 {return Err("part: not enough arguments".into());}

            let partition_stats =
            partition::partition_log(args[2].as_str(), args[3].as_str(), args[4].as_str())?;

            writeln!(stdout, "{}", partition_stats);
        },
        "version" => {
            writeln!(stdout, "hashdeep-compare version {}", VERSION);
        },

        x => return Err(format!("invalid command: {}", x).into())
    }

    Ok(())
}