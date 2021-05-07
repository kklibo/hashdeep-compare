use std::fs::{File,OpenOptions,read_to_string};
use std::io::{Write, ErrorKind};

use thiserror::Error;

use crate::log_entry::LogEntry;
use crate::partitioner::match_pair::MatchPair;
use crate::partitioner::match_group::{SingleFileMatchGroup,MatchGroup};
use crate::some_vec::SomeVec;


#[derive(Error, Debug)]
pub enum WriteToFileError {

    #[error("{0} exists (will not overwrite existing files)")]
    OutputFileExists(String),

    #[error("\"{0}\" cannot be opened for writing (does the directory exist?)")]
    OutputFileNotFound(String),

    #[error("\"{0}\" cannot be opened for writing (invalid path or unknown error)")]
    OutputFileOtherError(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl WriteToFileError {

    fn new(e: std::io::Error, path: &str) -> Self {

        match e.kind() {
            ErrorKind::AlreadyExists => WriteToFileError::OutputFileExists(path.to_string()),
            ErrorKind::NotFound      => WriteToFileError::OutputFileNotFound(path.to_string()),
            ErrorKind::Other         => WriteToFileError::OutputFileOtherError(path.to_string()),
            _ => e.into(),
        }
    }
}


#[derive(Error, Debug)]
pub enum ReadLogEntriesFromFileError {

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct LogFile<T>
    where T: Extend<LogEntry> + Default + IntoIterator
{
    pub entries: T,
    pub invalid_lines: Vec<String>,
}

pub fn read_log_entries_from_file<T>(filename: &str) -> Result<LogFile<T>, ReadLogEntriesFromFileError>
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

fn open_writable_file(filename: &str) -> Result<File, WriteToFileError>
{
    OpenOptions::new().write(true).create_new(true).open(filename)
        .map_err(|e| WriteToFileError::new(e, filename).into())
}

fn write_log_entry_to_file(label: &str, log_entry_str: &str, file: &mut File) -> Result<(), WriteToFileError>
{
    let line = format!("{}{}\n", label, log_entry_str);

    file.write_all(line.as_bytes())?;
    Ok(())
}

pub fn write_log_entries_to_file<T>(log_entries: T, filename: &str) -> Result<(), WriteToFileError>
    where T: IntoIterator, <T as ::std::iter::IntoIterator>::Item : ::std::string::ToString
{
    let mut file = open_writable_file(filename)?;

    for log_entry in log_entries {
        write_log_entry_to_file("", &log_entry.to_string(), &mut file)?;
    };

    Ok(())
}

pub fn write_match_pairs_to_file(match_pairs: &[MatchPair], filename: &str) -> Result<(), WriteToFileError>
{
    let mut file = open_writable_file(filename)?;

    for match_pair in match_pairs {
        write_log_entry_to_file("file1: ", &match_pair.from_file1.to_string(), &mut file)?;
        write_log_entry_to_file("file2: ", &match_pair.from_file2.to_string(), &mut file)?;
        file.write_all(b"\n")?;
    };

    Ok(())
}

pub fn write_match_groups_to_file(match_groups: &[MatchGroup], filename: &str) -> Result<(), WriteToFileError>
{
    let mut file = open_writable_file(filename)?;

    for match_group in match_groups {

        fn write_entries(entries: &SomeVec<&LogEntry>, label: &str, file: &mut File) -> Result<(), WriteToFileError>
        {
            for &log_entry in entries.inner_ref() {
                write_log_entry_to_file(label, &log_entry.to_string(), file)?;
            };
            Ok(())
        };
        write_entries(&match_group.from_file1, "file1: ", &mut file)?;
        write_entries(&match_group.from_file2, "file2: ", &mut file)?;

        file.write_all(b"\n")?;
    };

    Ok(())
}

pub fn write_single_file_match_groups_to_file(single_file_match_groups: &[SingleFileMatchGroup], filename: &str) -> Result<(), WriteToFileError>
{
    let mut file = open_writable_file(filename)?;

    for single_file_match_group in single_file_match_groups {

        for log_entry in single_file_match_group.log_entries.inner_ref() {
            write_log_entry_to_file("", &log_entry.to_string(), &mut file)?;
        };
        file.write_all(b"\n")?;
    };

    Ok(())
}
