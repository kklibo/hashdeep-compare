use crate::log_entry::LogEntry;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MatchPair<'a> {
    pub from_file1: &'a LogEntry,
    pub from_file2: &'a LogEntry,
}