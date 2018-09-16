use log_entry::LogEntry;

//todo: define as newtype?
pub struct SingleFileMatchGroup<'a> {
    pub log_entries: Vec<&'a LogEntry>,
}

pub struct MatchGroup<'a> {
    pub from_file1: Vec<&'a LogEntry>,
    pub from_file2: Vec<&'a LogEntry>,
}