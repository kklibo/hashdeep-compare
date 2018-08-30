use std::fs::read_to_string;
use std::io::Error;

use log_entry::LogEntry;

struct LogFile<T>
    where T: Extend<LogEntry> + Default
{
    entries: T,
    invalid_lines: Vec<String>,
}

fn read_log_entries_from_file<T>(filename: &str) -> Result<LogFile<T>, Error>
    where T: Extend<LogEntry> + Default
{

    let contents = read_to_string(filename)?;

    let mut entries = T::default();
    let mut invalid_lines = Vec::<String>::new();

    entries.extend(contents.lines().skip(5).filter_map(|line| {

        LogEntry::from_str(line).or_else( || {
            invalid_lines.push(line.to_owned());
            None
        })
    }));


    Ok(LogFile{entries, invalid_lines})
}

pub fn files_are_equal(filename1: &str, filename2: &str) -> bool {
    use std::fs::read_to_string;

    let str1 = match read_to_string(filename1) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let str2 = match read_to_string(filename2) {
        Ok(s) => s,
        Err(_) => return false,
    };

    str1 == str2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn files_are_equal_test() {
        assert!(files_are_equal("tests/empty file.txt", "tests/empty file.txt"));
        assert!(!files_are_equal("tests/empty file.txt", "tests/one newline.txt"));
        assert!(files_are_equal("tests/one newline.txt", "tests/one newline.txt"));
        assert!(files_are_equal("tests/test1.txt", "tests/test1.txt"));
        assert!(files_are_equal("tests/test1.txt", "tests/test1 copy.txt"));
        assert!(!files_are_equal("tests/test1.txt", "tests/test2.txt"));
    }
}