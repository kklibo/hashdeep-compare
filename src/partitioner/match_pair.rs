use crate::log_entry::LogEntry;

/// A match pair of file entries, one from each of two files.
///
/// The intended meaning of a match pair is a (probable) representation
/// of the same file in two different hashdeep logs, possibly before and after
/// some change to the file name, path, or contents.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MatchPair<'a> {
    pub from_file1: &'a LogEntry,
    pub from_file2: &'a LogEntry,
}