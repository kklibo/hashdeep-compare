use log_entry::LogEntry;
use some_vec::SomeVec;

//todo: define as newtype?
pub struct SingleFileMatchGroup<'a> {
    pub log_entries: SomeVec<&'a LogEntry>,
}

pub struct MatchGroup<'a> {
    pub from_file1: Vec<&'a LogEntry>,
    pub from_file2: Vec<&'a LogEntry>,
}