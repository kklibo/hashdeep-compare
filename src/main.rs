mod sort;

fn show_help() {
    println!("hashdeep tool lite");
    println!(" arguments");
    println!("  sort input_file output_file");
    println!("  nand input_file1 input_file2 output_file");
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
        "nand" => println!("not yet implemented"),
        _ => show_help(),
    }


}