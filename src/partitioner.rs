pub mod match_pair {
    use log_entry::LogEntry;
    use common::WhichFile::*;

    pub struct MatchPair<'a> {
        pub from_file1: &'a LogEntry,
        pub from_file2: &'a LogEntry,
        _prevent_struct_literal: (),
    }

    impl<'b> MatchPair<'b> {

        pub fn maybe_match_pair<'a>(a: &'a LogEntry, b: &'a LogEntry) -> Option<MatchPair<'a>> {
            if a.origin == File1 && b.origin == File2 {
                Some(MatchPair { from_file1: a, from_file2: b, _prevent_struct_literal: () })
            } else if a.origin == File2 && b.origin == File1 {
                Some(MatchPair { from_file1: b, from_file2: a, _prevent_struct_literal: () })
            } else {
                None
            }
        }
    }
}
pub use partitioner::match_pair::MatchPair;

use std::collections::HashMap;
use log_entry::LogEntry;

pub struct MatchGroup<'a> {
    pub entries: Vec<&'a LogEntry>, //todo: can this be replaced with a set?
}

pub struct MatchPartition<'a> {

    pub full_match_pairs: Vec<MatchPair<'a>>,
    pub full_match_groups: Vec<MatchGroup<'a>>,

    pub name_match_pairs: Vec<MatchPair<'a>>,
    pub name_match_groups: Vec<MatchGroup<'a>>,

    pub hashes_match_pairs: Vec<MatchPair<'a>>,
    pub hashes_match_groups: Vec<MatchGroup<'a>>,

    pub no_match: Vec<&'a LogEntry>,
}

impl<'a> MatchPartition<'a> {

    fn total_log_entries(&self) -> Option<usize> {
        use std::iter::once;

        fn pairs_sum(pairs: &Vec<MatchPair>) -> Option<usize> {
            pairs.iter().try_fold(0usize, |acc, ref x| acc.checked_add(2))
        }
        fn groups_sum(groups: &Vec<MatchGroup>) -> Option<usize> {
            groups.iter().try_fold(0usize, |acc, ref x| acc.checked_add(x.entries.len()))
        }
        fn vec_sum(v: &Vec<&LogEntry>) -> Option<usize> {
            Some(v.len())
        }

        (vec!{
            pairs_sum(&self.full_match_pairs),
            groups_sum(&self.full_match_groups),
            pairs_sum(&self.name_match_pairs),
            groups_sum(&self.name_match_groups),
            pairs_sum(&self.hashes_match_pairs),
            groups_sum(&self.hashes_match_groups),
            vec_sum(&self.no_match),
        }).iter()
        .try_fold(0usize, |acc: usize, x: &Option<usize>| {
            x.and_then(|y| acc.checked_add(y))
        })
    }
}

#[derive(Debug)]
pub enum MatchPartitionError {
    ChecksumFailure,
    ChecksumAdditionOverflow,
}

pub fn match_partition<'b>(log_entries: &Vec<&'b LogEntry>) -> Result<MatchPartition<'b>, MatchPartitionError> {

    struct SortedMatches<'a> {
        match_pairs: Vec<MatchPair<'a>>,
        match_groups: Vec<MatchGroup<'a>>,
        no_match: Vec<&'a LogEntry>,
    }

    fn sort_matches<'c, F>(log_entries: &Vec<&'c LogEntry>, f: F) -> SortedMatches<'c>
        where F: Fn(&LogEntry) -> String
    {
        let mut matches = HashMap::<String, Vec<&LogEntry>>::new();
        let mut match_pairs = Vec::<MatchPair>::new();
        let mut match_groups = Vec::<MatchGroup>::new();
        let mut no_match = Vec::<&LogEntry>::new();

        for &i in log_entries {
            matches.entry(f(i)).or_insert(Vec::<&LogEntry>::new()).push(i);
        }

        for (_, v) in matches {
            match v.len() {
                1 => no_match.push(v[0]),
                2 => match MatchPair::maybe_match_pair(v[0], v[1]) {
                    Some(mp) => match_pairs.push(mp),
                    None => match_groups.push(MatchGroup { entries: v })
                }
                _ => match_groups.push(MatchGroup { entries: v })
            }
        }

        SortedMatches { match_pairs, match_groups, no_match }
    }

    let full_matches = sort_matches(log_entries, |x| x.source_text());

    let name_matches = sort_matches(&full_matches.no_match, |ref x| x.filename.clone());

    let hashes_matches = sort_matches(&name_matches.no_match, |ref x| x.hashes.clone());

    let mp = MatchPartition {

        full_match_pairs: full_matches.match_pairs,
        full_match_groups: full_matches.match_groups,

        name_match_pairs: name_matches.match_pairs,
        name_match_groups: name_matches.match_groups,

        hashes_match_pairs: hashes_matches.match_pairs,
        hashes_match_groups: hashes_matches.match_groups,

        no_match: hashes_matches.no_match,
    };

    match mp.total_log_entries() {
        Some(t) if t == log_entries.len() => Ok(mp),
        Some(t) => Err(MatchPartitionError::ChecksumFailure),
        None => Err(MatchPartitionError::ChecksumAdditionOverflow),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{read_log_entries_from_file,WhichFile};

    #[test]
    fn match_partition_test() {

        let file1 = read_log_entries_from_file::<Vec<LogEntry>>("tests/partition_test1.txt", WhichFile::File1).unwrap();
        let file2 = read_log_entries_from_file::<Vec<LogEntry>>("tests/partition_test2.txt", WhichFile::File2).unwrap();
        assert_eq!(0, file1.invalid_lines.len());
        assert_eq!(0, file2.invalid_lines.len());

        let log_entries = file1.entries.iter().chain(file2.entries.iter()).collect::<Vec<&LogEntry>>();

        let mp = match_partition(&log_entries).unwrap();

        assert_eq!(1, mp.full_match_pairs.len());
        assert_eq!(1, mp.full_match_groups.len());
        assert_eq!(1, mp.name_match_pairs.len());
        assert_eq!(1, mp.name_match_groups.len());
        assert_eq!(1, mp.hashes_match_pairs.len());
        assert_eq!(2, mp.hashes_match_groups.len());
        assert_eq!(2, mp.no_match.len());
    }
}