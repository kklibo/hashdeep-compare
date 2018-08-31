use common;
use common::WhichFile::SingleFile;
use log_entry::LogEntry;

pub fn sort_log(filename: &str, out_filename: &str) -> ::std::io::Result<()>{

    let mut log_file = common::read_log_entries_from_file::<Vec<LogEntry>>(filename, SingleFile)?;
    assert_eq!(0, log_file.invalid_lines.len());

    log_file.entries.sort_by(|ref v1, ref v2| {

        v1.filename.cmp(&v2.filename)
    });

    common::write_log_entries_to_file(log_file.entries, out_filename)
}

#[cfg(test)]
mod test {
    use super::*;
    use common::files_are_equal;

    #[test]
    fn sort_log_test() {
        {
            let test_out = "tests/temp/sort_log_test.txt";
            sort_log("tests/test1.txt", test_out).unwrap();
            assert!(files_are_equal("tests/test1 sorted.txt", test_out));
        }
    }
}