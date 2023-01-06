use std::fs::{File,OpenOptions,read_to_string};
use std::io::{Write, ErrorKind};
use std::fmt::{Display, Formatter};

use thiserror::Error;
use peeking_take_while::PeekableExt;

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

    #[error("\"{0}\" cannot be opened for writing ({})", .1)]
    OutputFileOtherError(String, #[source] std::io::Error),

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
            _                        => WriteToFileError::OutputFileOtherError(path.to_string(), e),
        }
    }
}


#[derive(Error, Debug)]
pub enum ReadLogEntriesFromFileError {

    #[error("\"{0}\" cannot be opened for reading (not found)")]
    FileNotFound(String),

    #[error("\"{0}\" cannot be opened for reading ({})", .1)]
    OtherIoError(String, #[source] std::io::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ReadLogEntriesFromFileError {

    fn new(e: std::io::Error, path: &str) -> Self {

        match e.kind() {
            ErrorKind::NotFound => ReadLogEntriesFromFileError::FileNotFound(path.to_string()),
            _                   => ReadLogEntriesFromFileError::OtherIoError(path.to_string(), e),
        }
    }
}


//#[derive(Debug, Eq, PartialEq)]//
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HashdeepLogHeaderWarning {
    UnexpectedVersionString(String),
    HeaderNotFound,
    UntestedLogFormat(String),
    UnexpectedHeaderLineCount(usize),
}

impl Display for HashdeepLogHeaderWarning {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        use HashdeepLogHeaderWarning::*;

        match self {
            UnexpectedVersionString(s) => write!(f, "Unexpected version string: \"{}\"", s),
            HeaderNotFound => write!(f, "Header not found"),
            UntestedLogFormat(s) => write!(f, "Untested log format: \"{}\"", s),
            UnexpectedHeaderLineCount(n) => write!(f, "Unexpected header line count: {} (expected: 5)", n),
        }
    }
}

fn check_hashdeep_log_header(header_lines: &[String]) -> Vec<HashdeepLogHeaderWarning> {

    //  example header (should always be 5 lines):
    //
    //  %%%% HASHDEEP-1.0
    //  %%%% size,md5,sha256,filename
    //  ## Invoked from: /home/user
    //  ## $ hashdeep -lr hashdeepComp/
    //  ##

    let mut warnings: Vec<HashdeepLogHeaderWarning> = Vec::new();

    match header_lines.get(0) {
        Some(x) if x == "%%%% HASHDEEP-1.0" => {},
        Some(x) => warnings.push(HashdeepLogHeaderWarning::UnexpectedVersionString(x.clone())),
        None => return [HashdeepLogHeaderWarning::HeaderNotFound].into(),
    }

    match header_lines.get(1) {
        Some(x) if x == "%%%% size,md5,sha256,filename" => {},
        Some(x) => warnings.push(HashdeepLogHeaderWarning::UntestedLogFormat(x.clone())),
        None => {}
    }

    match header_lines.len() {
        5 => {},
        x => warnings.push(HashdeepLogHeaderWarning::UnexpectedHeaderLineCount(x))
    };

    warnings
}


/// The result of successfully reading a hashdeep log:
/// its entries, plus load-time header warnings and entry parse failures (if any)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LogFile<T>
    where T: Extend<LogEntry> + Default + IntoIterator
{
    pub entries: T,
    pub header_warnings: Vec<HashdeepLogHeaderWarning>,
    pub invalid_lines: Vec<String>,
}

impl<T> LogFile<T>
    where T: Extend<LogEntry> + Default + IntoIterator
{
    ///Returns a Vec of printable warning lines (or None, if no warnings or invalid lines exist)
    pub fn warning_report(&self) -> Option<Vec<String>> {

        let mut lines = self.header_warnings.iter().map(
            |w| w.to_string()
        ).collect::<Vec<String>>();

        match self.invalid_lines.len() {
            0 => {},
            1 => lines.push("1 invalid log entry detected".to_string()),
            x => lines.push(format!("{} invalid log entries detected", x))
        }

        match lines.is_empty() {
            true => None,
            false => Some(lines)
        }
    }
}

/// Reads a hashdeep log: checks the header, then collects entries + parse failures
///
/// # Errors
///
/// Any error encountered while reading the file will be returned.
pub fn read_log_entries_from_file<T>(filename: &str) -> Result<LogFile<T>, ReadLogEntriesFromFileError>
    where T: Extend<LogEntry> + Default + IntoIterator
{
    let contents = read_to_string(filename)
        .map_err(|e| ReadLogEntriesFromFileError::new(e, filename))?;

    let mut entries = T::default();
    let mut invalid_lines = Vec::<String>::new();

    let mut lines = contents.lines().peekable();

    //collect the header lines based on expected prefix symbols
    let header_lines: Vec<String> = lines.peeking_take_while(|x: &&str| {
        x.starts_with("%%%%") ||
        x.starts_with("##")
    }).map(|s| s.to_string() ).collect();

    let header_warnings = check_hashdeep_log_header(&header_lines);

    entries.extend(lines.filter_map(|line| {

        LogEntry::from_str(line).or_else( || {
            invalid_lines.push(line.to_owned());
            None
        })
    }));


    Ok(LogFile{entries, header_warnings, invalid_lines})
}

fn open_writable_file(filename: &str) -> Result<File, WriteToFileError>
{
    OpenOptions::new().write(true).create_new(true).open(filename)
        .map_err(|e| WriteToFileError::new(e, filename))
}

fn write_log_entry_to_file(label: &str, log_entry_str: &str, file: &mut File) -> Result<(), WriteToFileError>
{
    let line = format!("{}{}\n", label, log_entry_str);

    file.write_all(line.as_bytes())?;
    Ok(())
}

/// Writes log entries to a new file (will not overwrite an existing file).
///
/// # Errors
///
/// Will return an error if the file at `filename` already exists, or
/// if an error occurs while writing to the file.
pub fn write_log_entries_to_file<T>(log_entries: T, filename: &str) -> Result<(), WriteToFileError>
    where T: IntoIterator, <T as IntoIterator>::Item : ToString
{
    let mut file = open_writable_file(filename)?;

    for log_entry in log_entries {
        write_log_entry_to_file("", &log_entry.to_string(), &mut file)?;
    };

    Ok(())
}

/// Writes match pairs of log entries to a new file (will not overwrite an existing file).
///
/// # Errors
///
/// Will return an error if the file at `filename` already exists, or
/// if an error occurs while writing to the file.
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

/// Writes match groups of log entries to a new file (will not overwrite an existing file).
///
/// # Errors
///
/// Will return an error if the file at `filename` already exists, or
/// if an error occurs while writing to the file.
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
        }
        write_entries(&match_group.from_file1, "file1: ", &mut file)?;
        write_entries(&match_group.from_file2, "file2: ", &mut file)?;

        file.write_all(b"\n")?;
    };

    Ok(())
}

/// Writes match groups (from a single source file) of log entries to a new file
/// (will not overwrite an existing file).
///
/// # Errors
///
/// Will return an error if the file at `filename` already exists, or
/// if an error occurs while writing to the file.
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

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn check_hashdeep_log_header_test() {

        use HashdeepLogHeaderWarning::*;

        fn to_vec_string(x: &[&str]) -> Vec<String> {
            x.iter().map(|x|x.to_string()).collect::<Vec<String>>()
        }

        //success
        {
            let header_lines = [
                "%%%% HASHDEEP-1.0",
                "%%%% size,md5,sha256,filename",
                "## Invoked from: /home/user",
                "## $ hashdeep -lr hashdeepComp/",
                "## ",
            ];

            let expected = [];

            let warnings = check_hashdeep_log_header(&to_vec_string(&header_lines));

            assert_eq!(warnings, expected);
        }

        {
            let header_lines = [
                "%%%% HASHDEEP-2.0",
                "%%%% size,md5,sha256,filename",
                "## Invoked from: /home/user",
                "## $ hashdeep -lr hashdeepComp/",
                "## ",
            ];

            let expected = [UnexpectedVersionString("%%%% HASHDEEP-2.0".into())];

            let warnings = check_hashdeep_log_header(&to_vec_string(&header_lines));

            assert_eq!(warnings, expected);
        }

        {
            let header_lines = [];

            let expected = [HeaderNotFound];

            let warnings = check_hashdeep_log_header(&to_vec_string(&header_lines));

            assert_eq!(warnings, expected);
        }

        {
            let header_lines = [
                "%%%% HASHDEEP-1.0",
                "%%%% size,md5,filename",
                "## Invoked from: /home/user",
                "## $ hashdeep -lr hashdeepComp/",
                "## ",
            ];

            let expected = [UntestedLogFormat("%%%% size,md5,filename".into())];

            let warnings = check_hashdeep_log_header(&to_vec_string(&header_lines));

            assert_eq!(warnings, expected);
        }

        {
            let header_lines = [
                "%%%% HASHDEEP-1.0",
                "%%%% size,md5,sha256,filename",
                "## Invoked from: /home/user",
                "## $ hashdeep -lr hashdeepComp/",
            ];

            let expected = [UnexpectedHeaderLineCount(4)];

            let warnings = check_hashdeep_log_header(&to_vec_string(&header_lines));

            assert_eq!(warnings, expected);
        }

        {
            let header_lines = [
                "%%%% HASHDEEP-3.0",
                "%%%% size,sha256,filename",
            ];

            let expected = [
                UnexpectedVersionString("%%%% HASHDEEP-3.0".into()),
                UntestedLogFormat("%%%% size,sha256,filename".into()),
                UnexpectedHeaderLineCount(2),
            ];

            let warnings = check_hashdeep_log_header(&to_vec_string(&header_lines));

            assert_eq!(warnings, expected);
        }
    }
}