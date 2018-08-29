use std::fs::read_to_string;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("hashdeep tool lite");
        println!(" arguments: input_file output_file");
        return Ok(());
    }

    let filename = &args[1];
    let out_filename = &args[2];

    //let filename = "tests/test1.txt";
    //let out_filename = &format!("{}.sorted", filename)[..];

    let contents = read_to_string(filename)?;
    let mut lines: Vec<&str> = contents.lines().skip(5).collect();

    lines.sort_by_key(|&s| {
        let a = s.split(",").skip(3).collect::<Vec<&str>>();
        let b = (&a[..].join(",")).to_owned();

        //println!("{}", b);
        b

    });


    use std::fs::File;
    use std::io::Write;
    let mut file = File::create(out_filename)?;

    //println!();
    for s in lines {
        //println!("{}", s);

        file.write(s.as_bytes());
        file.write("\n".as_bytes());
    };





    Ok(())
}
