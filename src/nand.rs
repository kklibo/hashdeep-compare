use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::collections::HashSet;

pub fn nand_log(filename1: &str, filename2: &str, only_in_1_filename: &str, only_in_2_filename: &str) -> ::std::io::Result<()>{

    let contents1 = read_to_string(filename1)?;
    let lines1: HashSet<&str> = contents1.lines().skip(5).collect();

    let contents2 = read_to_string(filename2)?;
    let lines2: HashSet<&str> = contents2.lines().skip(5).collect();

    //let nand_lines_set = lines1.symmetric_difference(&lines2);
    let only_in_1 = lines1.difference(&lines2);
    let only_in_2 = lines2.difference(&lines1);

    //let mut nand_lines: Vec<&&str> = nand_lines_set.into_iter().collect();
    let mut only_in_1_lines: Vec<&&str> = only_in_1.collect();

    //nand_lines.sort_by_key(|&s| {
    only_in_1_lines.sort_by_key(|&s| {
        let a = s.split(",").skip(3).collect::<Vec<&str>>();
        let b = (&a[..].join(",")).to_owned();
        b
    });

    let mut only_in_2_lines: Vec<&&str> = only_in_2.collect();

    //nand_lines.sort_by_key(|&s| {
    only_in_2_lines.sort_by_key(|&s| {
        let a = s.split(",").skip(3).collect::<Vec<&str>>();
        let b = (&a[..].join(",")).to_owned();
        b
    });

    let mut outfile1 = File::create(only_in_1_filename)?;

    //println!();
    for s in only_in_1_lines {
        //println!("{}", s);

        outfile1.write(s.as_bytes());
        outfile1.write("\n".as_bytes());
    };

    let mut outfile2 = File::create(only_in_2_filename)?;

    //println!();
    for s in only_in_2_lines {
        //println!("{}", s);

        outfile2.write(s.as_bytes());
        outfile2.write("\n".as_bytes());
    };

    Ok(())



}