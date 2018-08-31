use std::fs::{File,read_to_string};
use std::io::{Write,Error};

use log_entry::LogEntry;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum WhichFile {
    File1,
    File2,
    SingleFile,
}

pub struct LogFile<T>
    where T: Extend<LogEntry> + Default + IntoIterator
{
    pub entries: T,
    pub invalid_lines: Vec<String>,
    pub which_file: WhichFile,
}

pub fn read_log_entries_from_file<T>(filename: &str, which_file: WhichFile) -> Result<LogFile<T>, Error>
    where T: Extend<LogEntry> + Default + IntoIterator
{

    let contents = read_to_string(filename)?;

    let mut entries = T::default();
    let mut invalid_lines = Vec::<String>::new();

    entries.extend(contents.lines().skip(5).filter_map(|line| {

        LogEntry::from_str(line, which_file).or_else( || {
            invalid_lines.push(line.to_owned());
            None
        })
    }));


    Ok(LogFile{entries, invalid_lines, which_file})
}

pub fn write_log_entries_to_file<T>(log_entries: T, filename: &str) -> Result<(), Error>
    where T: IntoIterator, <T as ::std::iter::IntoIterator>::Item : ::std::string::ToString
{

    let mut file = File::create(filename)?;

    for log_entry in log_entries {
        file.write(log_entry.to_string().as_bytes())?;
        file.write("\n".as_bytes())?;
    };

    Ok(())
}

#[allow(dead_code)]
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