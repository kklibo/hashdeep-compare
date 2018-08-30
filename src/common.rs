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