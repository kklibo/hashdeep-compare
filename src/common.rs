use std::fs::{File,read_to_string};
use std::io::{Write,Error};

use log_entry::LogEntry;
use partitioner::match_pair::MatchPair;
use partitioner::match_group::MatchGroup;

pub struct LogFile<T>
    where T: Extend<LogEntry> + Default + IntoIterator
{
    pub entries: T,
    pub invalid_lines: Vec<String>,
}

pub fn read_log_entries_from_file<T>(filename: &str) -> Result<LogFile<T>, Error>
    where T: Extend<LogEntry> + Default + IntoIterator
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

fn write_log_entry_to_file(label: &str, log_entry_str: &str, file: &mut File) -> Result<(), Error>
{
    let line = format!("{}{}\n", label, log_entry_str);

    file.write(line.as_bytes())?;
    Ok(())
}

pub fn write_log_entries_to_file<T>(log_entries: T, filename: &str) -> Result<(), Error>
    where T: IntoIterator, <T as ::std::iter::IntoIterator>::Item : ::std::string::ToString
{
    let mut file = File::create(filename)?;

    for log_entry in log_entries {
        write_log_entry_to_file("", &log_entry.to_string(), &mut file)?;
    };

    Ok(())
}

pub fn write_match_pairs_to_file(match_pairs: &Vec<MatchPair>, filename: &str) -> Result<(), Error>
{
    let mut file = File::create(filename)?;

    for match_pair in match_pairs {
        write_log_entry_to_file("file1: ", &match_pair.from_file1.to_string(), &mut file)?;
        write_log_entry_to_file("file2: ", &match_pair.from_file2.to_string(), &mut file)?;
        file.write("\n".as_bytes())?;
    };

    Ok(())
}

pub fn write_match_groups_to_file(match_groups: &Vec<MatchGroup>, filename: &str) -> Result<(), Error>
{
    let mut file = File::create(filename)?;

    for match_group in match_groups {

        fn write_entries(entries: &Vec<&LogEntry>, label: &str, file: &mut File) -> Result<(), Error>
        {
            for &log_entry in entries {
                write_log_entry_to_file(label, &log_entry.to_string(), file)?;
            };
            Ok(())
        };
        write_entries(&match_group.from_file1, "file1: ", &mut file)?;
        write_entries(&match_group.from_file2, "file2: ", &mut file)?;

        file.write("\n".as_bytes())?;
    };

    Ok(())
}

#[cfg(test)]
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