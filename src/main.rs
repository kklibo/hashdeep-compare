extern crate hashdeep_tool_lite;
use hashdeep_tool_lite::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    fn show_help() {
        println!("hashdeep tool lite version {}", VERSION);
        println!(" arguments");
        println!("  version");
        println!("  hash target_directory output_path_base");
        println!("  sort input_file output_file");
        println!("  part input_file1 input_file2 output_file_base");
    }

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        show_help();
        return Ok(());
    }


    match args[1].as_str() {
        "hash" => {
            if args.len() < 4 {Err("hash: not enough arguments")?}

            command::run_hashdeep_command(
                args[2].as_str(),
                args[3].as_str())?;
        },
        "sort" => {
            if args.len() < 4 {Err("sort: not enough arguments")?}

            sort::sort_log(args[2].as_str(), args[3].as_str())?;
        },
        "part" => {
            if args.len() < 5 {Err("part: not enough arguments")?}

            partition::partition_log(args[2].as_str(), args[3].as_str(), args[4].as_str())?;
        },
        "version" => {
            println!("hashdeep tool lite version {}", VERSION);
        },

        x => Err(format!("invalid command: {}", x))?
    }

    Ok(())
}