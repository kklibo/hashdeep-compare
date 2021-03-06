use crate::log_entry::LogEntry;
use crate::some_vec::SomeVec;

/// A match group with entries from only one file.
#[derive(PartialEq, Debug)]
pub struct SingleFileMatchGroup<'a> {
    pub log_entries: SomeVec<&'a LogEntry>,
}

/// A match group with entries from each of two files.
#[derive(PartialEq, Debug)]
pub struct MatchGroup<'a> {
    pub from_file1: SomeVec<&'a LogEntry>,
    pub from_file2: SomeVec<&'a LogEntry>,
}