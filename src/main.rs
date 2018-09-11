extern crate hashdeep_tool_lite;
use hashdeep_tool_lite::*;

fn show_help() {
    println!("hashdeep tool lite");
    println!(" arguments");
    println!("  sort input_file output_file");
    println!("  part input_file1 input_file2 output_file_base");
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        show_help();
        return;
    }


    match args[1].as_str() {
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

        _ => show_help(),
    }


}