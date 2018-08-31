use std::collections::HashSet;
use std::cmp::Ordering;
use common;
use common::WhichFile::SingleFile;
use log_entry::LogEntry;

pub fn nand_log(filename1: &str, filename2: &str, only_in_1_filename: &str, only_in_2_filename: &str) -> ::std::io::Result<()>{

    let log_file1 = common::read_log_entries_from_file::<HashSet<LogEntry>>(filename1, SingleFile)?;
    assert_eq!(0, log_file1.invalid_lines.len());//todo: remove this

    let log_file2 = common::read_log_entries_from_file::<HashSet<LogEntry>>(filename2, SingleFile)?;
    assert_eq!(0, log_file2.invalid_lines.len());//todo: remove this

    let only_in_1 = log_file1.entries.difference(&log_file2.entries);
    let only_in_2 = log_file2.entries.difference(&log_file1.entries);

    fn cmp_by_filename(l1: &&LogEntry, l2: &&LogEntry) -> Ordering {
        l1.filename.cmp(&l2.filename)
    }

    let mut only_in_1_lines: Vec<&LogEntry> = only_in_1.collect();
    only_in_1_lines.sort_by(cmp_by_filename);

    let mut only_in_2_lines: Vec<&LogEntry> = only_in_2.collect();
    only_in_2_lines.sort_by(cmp_by_filename);

    common::write_log_entries_to_file(only_in_1_lines, only_in_1_filename)?;
    common::write_log_entries_to_file(only_in_2_lines, only_in_2_filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::files_are_equal;

    #[test]
    fn nand_log_test() {

        {
            let same_file = "tests/test1.txt";
            let empty_file = "tests/empty file.txt";
            let only_in_1 = "tests/temp/only in 1";
            let only_in_2 = "tests/temp/only in 2";
            nand_log(same_file, same_file, only_in_1, only_in_2).unwrap();
            assert!(files_are_equal(empty_file, only_in_1));
            assert!(files_are_equal(empty_file, only_in_2));
        }
        {
            let only_in_test1 = "tests/temp/only in test1";
            let only_in_test2 = "tests/temp/only in test2";
            nand_log("tests/test1.txt", "tests/test2.txt", only_in_test1, only_in_test2).unwrap();
            assert!(files_are_equal("tests/only in test1.txt", only_in_test1));
            assert!(files_are_equal("tests/only in test2.txt", only_in_test2));
        }

    }
}