use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

pub fn sort_log(filename: &str, out_filename: &str) -> ::std::io::Result<()>{

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

    let mut file = File::create(out_filename)?;

    //println!();
    for s in lines {
        //println!("{}", s);

        file.write(s.as_bytes());
        file.write("\n".as_bytes());
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use common::files_are_equal;

    #[test]
    fn sort_log_test() {
        {
            let test_out = "tests/temp/sort_log_test.txt";
            sort_log("tests/test1.txt", test_out);
            assert!(files_are_equal("tests/test1 sorted.txt", test_out));
        }
    }
}