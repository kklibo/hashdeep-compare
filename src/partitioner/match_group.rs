use crate::log_entry::LogEntry;
use crate::some_vec::SomeVec;

pub struct SingleFileMatchGroup<'a> {
    pub log_entries: SomeVec<&'a LogEntry>,
}

pub struct MatchGroup<'a> {
    pub from_file1: SomeVec<&'a LogEntry>,
    pub from_file2: SomeVec<&'a LogEntry>,
}