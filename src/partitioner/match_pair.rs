use log_entry::LogEntry;

#[derive(Eq, PartialEq, Debug)]
pub struct MatchPair<'a> {
    pub from_file1: &'a LogEntry,
    pub from_file2: &'a LogEntry,
}