extern crate hashdeep_tool_lite;
use hashdeep_tool_lite::*;

fn main() {

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
        return;
    }


    match args[1].as_str() {
        "hash" => {
            if args.len() < 4 {return;}

            match command::run_hashdeep_command(
                args[2].as_str(),
                args[3].as_str())
            {
                Ok(_) => (),
                Err(e) => println!("{:?}", e),
            }
        },
        "sort" => {
            if args.len() < 4 {return;}
            match sort::sort_log(args[2].as_str(), args[3].as_str()) {
                Ok(()) => (),
                Err(e) => println!("{:?}", e),
            }
        },
        "part" => {
            if args.len() < 5 {return;}
            match partition::partition_log(args[2].as_str(), args[3].as_str(), args[4].as_str()) {
                Ok(()) => (),
                Err(e) => println!("{:?}", e),
            }
        },
        "version" => {
            println!("hashdeep tool lite version {}", VERSION);
        },

        _ => show_help(),
    }


}