use std::fs::{File,read_to_string};
use std::io::{Write,Error};

use log_entry::LogEntry;
use partitioner::match_pair::MatchPair;
use partitioner::match_group::{SingleFileMatchGroup,MatchGroup};
use some_vec::SomeVec;

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

        fn write_entries(entries: &SomeVec<&LogEntry>, label: &str, file: &mut File) -> Result<(), Error>
        {
            for &log_entry in entries.inner_ref() {
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

pub fn write_single_file_match_groups_to_file(single_file_match_groups: &Vec<SingleFileMatchGroup>, filename: &str) -> Result<(), Error>
{
    let mut file = File::create(filename)?;

    for single_file_match_group in single_file_match_groups {

        for log_entry in single_file_match_group.log_entries.inner_ref() {
            write_log_entry_to_file("", &log_entry.to_string(), &mut file)?;
        };
        file.write("\n".as_bytes())?;
    };

    Ok(())
}
