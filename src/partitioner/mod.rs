pub mod match_pair;
pub mod match_group;

use std::collections::BTreeMap;
use thiserror::Error;

use self::match_pair::MatchPair;
use self::match_group::{MatchGroup,SingleFileMatchGroup};
use crate::log_entry::LogEntry;
use crate::some_vec::SomeVec;

/// Represents the contents of two hashdeep logs partitioned
/// by match type, allowing inference of intervening file changes.
#[derive(PartialEq, Debug, Default)]
pub struct MatchPartition<'a> {

    pub full_match_pairs: Vec<MatchPair<'a>>,
    pub full_match_groups: Vec<MatchGroup<'a>>,
    pub full_match_groups_file1: Vec<SingleFileMatchGroup<'a>>,
    pub full_match_groups_file2: Vec<SingleFileMatchGroup<'a>>,

    pub name_match_pairs: Vec<MatchPair<'a>>,
    pub name_match_groups: Vec<MatchGroup<'a>>,
    pub name_match_groups_file1: Vec<SingleFileMatchGroup<'a>>,
    pub name_match_groups_file2: Vec<SingleFileMatchGroup<'a>>,

    pub hashes_match_pairs: Vec<MatchPair<'a>>,
    pub hashes_match_groups: Vec<MatchGroup<'a>>,
    pub hashes_match_groups_file1: Vec<SingleFileMatchGroup<'a>>,
    pub hashes_match_groups_file2: Vec<SingleFileMatchGroup<'a>>,

    pub no_match_file1: Vec<&'a LogEntry>,
    pub no_match_file2: Vec<&'a LogEntry>,
}

impl<'a> MatchPartition<'a> {

    fn total_log_entries(&self) -> Option<usize> {

        fn pairs_sum(pairs: &[MatchPair]) -> Option<usize> {
            pairs.len().checked_mul(2)
        }
        fn groups_sum(groups: &[MatchGroup]) -> Option<usize> {
            groups.iter().try_fold(0usize, |acc, x| {
                x.from_file1.len().checked_add(x.from_file2.len())
                    .and_then(|x| acc.checked_add(x))
            })
        }
        fn single_file_groups_sum(groups: &[SingleFileMatchGroup]) -> Option<usize> {
            groups.iter().try_fold(0usize, |acc, x| {
                acc.checked_add(x.log_entries.len())
            })
        }

        (vec!{
            pairs_sum(&self.full_match_pairs),
            single_file_groups_sum(&self.full_match_groups_file1),
            single_file_groups_sum(&self.full_match_groups_file2),
            groups_sum(&self.full_match_groups),
            pairs_sum(&self.name_match_pairs),
            single_file_groups_sum(&self.name_match_groups_file1),
            single_file_groups_sum(&self.name_match_groups_file2),
            groups_sum(&self.name_match_groups),
            pairs_sum(&self.hashes_match_pairs),
            single_file_groups_sum(&self.hashes_match_groups_file1),
            single_file_groups_sum(&self.hashes_match_groups_file2),
            groups_sum(&self.hashes_match_groups),
            Some(self.no_match_file1.len()),
            Some(self.no_match_file2.len()),
        }).iter()
            .try_fold(0usize, |acc: usize, x: &Option<usize>| {
                x.and_then(|y| acc.checked_add(y))
            })
    }
}

#[derive(Error, Debug)]
pub enum MatchPartitionError {
    #[error("Serious error: Match partition checksum failed (this should never happen)")]
    ChecksumFailure,
    #[error("arithmetic overflow in match partition checksum calculation")]
    ChecksumArithmeticOverflow,
}

/// The main implementation function for the `part` command:
/// Partitions entries into a structure of match pairs and groupings, allowing
/// comparison of the two source hashdeep log files in terms of inferred
/// intervening file changes.
///
/// # Errors
///
/// An integrity check is run on the results of the partitioning operation.
/// An error will be issued if this check fails (this is extremely unlikely).
pub fn match_partition<'b>(from_file1: &[&'b LogEntry], from_file2: &[&'b LogEntry]) -> Result<MatchPartition<'b>, MatchPartitionError> {

    struct SortedMatches<'a> {
        match_pairs: Vec<MatchPair<'a>>,
        match_groups: Vec<MatchGroup<'a>>,
        match_groups_file1: Vec<SingleFileMatchGroup<'a>>,
        match_groups_file2: Vec<SingleFileMatchGroup<'a>>,
        no_match_file1: Vec<&'a LogEntry>,
        no_match_file2: Vec<&'a LogEntry>,
    }

    fn sort_matches<'c, F>(from_file1: &[&'c LogEntry], from_file2: &[&'c LogEntry], f: F) -> SortedMatches<'c>
        where F: Fn(&LogEntry) -> String
    {
        enum LogEntryFrom<'a> {
            File1(&'a LogEntry),
            File2(&'a LogEntry),
        }

        let mut matches = BTreeMap::<String, SomeVec<LogEntryFrom>>::new();

        for &i in from_file1 {
            matches.entry(f(i))
                .and_modify(|x| x.push(LogEntryFrom::File1(i)))
                .or_insert_with(|| SomeVec::<LogEntryFrom>::from_first_value(LogEntryFrom::File1(i)));
        }

        for &i in from_file2 {
            matches.entry(f(i))
                .and_modify(|x| x.push(LogEntryFrom::File2(i)))
                .or_insert_with(|| SomeVec::<LogEntryFrom>::from_first_value(LogEntryFrom::File2(i)));
        }


        let mut match_pairs = Vec::<MatchPair>::new();
        let mut match_groups = Vec::<MatchGroup>::new();
        let mut match_groups_file1 = Vec::<SingleFileMatchGroup>::new();
        let mut match_groups_file2 = Vec::<SingleFileMatchGroup>::new();
        let mut no_match_file1 = Vec::<&LogEntry>::new();
        let mut no_match_file2 = Vec::<&LogEntry>::new();

        for (_, v) in matches {
            match v.len() {
                0 => unreachable!(), //SomeVec.len() is always positive
                1 => match v.at(0) {
                    LogEntryFrom::File1(x) => no_match_file1.push(x),
                    LogEntryFrom::File2(x) => no_match_file2.push(x),
                },
                2 => match (&v.at(0), &v.at(1)) {
                    (LogEntryFrom::File1(x),LogEntryFrom::File1(y)) => match_groups_file1.push(SingleFileMatchGroup{log_entries: SomeVec::from_values(*x,*y)}),
                    (LogEntryFrom::File1(x),LogEntryFrom::File2(y)) => match_pairs.push(MatchPair{from_file1: x, from_file2: y}),
                    (LogEntryFrom::File2(x),LogEntryFrom::File1(y)) => match_pairs.push(MatchPair{from_file1: y, from_file2: x}),
                    (LogEntryFrom::File2(x),LogEntryFrom::File2(y)) => match_groups_file2.push(SingleFileMatchGroup{log_entries: SomeVec::from_values(*x,*y)}),
                },
                _ => {
                    let mut from_file1 = Vec::<&LogEntry>::new();
                    let mut from_file2 = Vec::<&LogEntry>::new();

                    for i in v.inner_ref() {
                        match i {
                            LogEntryFrom::File1(x) => from_file1.push(x),
                            LogEntryFrom::File2(x) => from_file2.push(x),
                        }
                    }

                    match (SomeVec::from_vec(from_file1), SomeVec::from_vec(from_file2)) {
                        (Some(log_entries), None) => match_groups_file1.push(SingleFileMatchGroup{log_entries}),
                        (None, Some(log_entries)) => match_groups_file2.push(SingleFileMatchGroup{log_entries}),
                        (Some(from_file1), Some(from_file2)) => match_groups.push(MatchGroup{from_file1, from_file2}),
                        (None, None) => unreachable!("empty SomeVec in sort_matches"),
                    }
                }
            }
        }

        SortedMatches { match_pairs, match_groups, match_groups_file1, match_groups_file2, no_match_file1, no_match_file2 }
    }

    let full_matches = sort_matches(from_file1, from_file2, |x| x.to_string());

    let name_matches = sort_matches(&full_matches.no_match_file1, &full_matches.no_match_file2, |x| x.filename.clone());

    let hashes_matches = sort_matches(&name_matches.no_match_file1, &name_matches.no_match_file2, |x| x.hashes.clone());

    let mut mp = MatchPartition {

        full_match_pairs: full_matches.match_pairs,
        full_match_groups: full_matches.match_groups,
        full_match_groups_file1: full_matches.match_groups_file1,
        full_match_groups_file2: full_matches.match_groups_file2,

        name_match_pairs: name_matches.match_pairs,
        name_match_groups: name_matches.match_groups,
        name_match_groups_file1: name_matches.match_groups_file1,
        name_match_groups_file2: name_matches.match_groups_file2,

        hashes_match_pairs: hashes_matches.match_pairs,
        hashes_match_groups: hashes_matches.match_groups,
        hashes_match_groups_file1: hashes_matches.match_groups_file1,
        hashes_match_groups_file2: hashes_matches.match_groups_file2,

        no_match_file1: hashes_matches.no_match_file1,
        no_match_file2: hashes_matches.no_match_file2,
    };

    fn sort_match_pairs_by_filename(x: &mut [MatchPair]) {
        x.sort_by(|a, b| a.from_file1.filename.cmp(&b.from_file1.filename));
    }

    fn sort_match_groups_by_filename(x: &mut [MatchGroup]) {

        x.iter_mut().for_each(|x| {
            sort_log_entries_somevec_by_filename(&mut x.from_file1);
            sort_log_entries_somevec_by_filename(&mut x.from_file2);
        });

        x.sort_by(|a, b| {
            a.from_file1.first().filename.cmp(&b.from_file1.first().filename)
        });
    }

    fn sort_single_file_match_groups_by_filename(x: &mut [SingleFileMatchGroup]) {

        x.iter_mut().for_each(|x| {
            sort_log_entries_somevec_by_filename(&mut x.log_entries);
        });

        x.sort_by(|a, b| {
            a.log_entries.first().filename.cmp(&b.log_entries.first().filename)
        });
    }

    fn sort_log_entries_by_filename(x: &mut [&LogEntry]) {
        x.sort_by(|a, b| a.filename.cmp(&b.filename));
    }
    fn sort_log_entries_somevec_by_filename(x: &mut SomeVec<&LogEntry>) {
        x.sort_by(|a, b| a.filename.cmp(&b.filename));
    }

    sort_match_pairs_by_filename(&mut mp.full_match_pairs);
    sort_single_file_match_groups_by_filename(&mut mp.full_match_groups_file1);
    sort_single_file_match_groups_by_filename(&mut mp.full_match_groups_file2);
    sort_match_groups_by_filename(&mut mp.full_match_groups);
    sort_match_pairs_by_filename(&mut mp.name_match_pairs);
    sort_single_file_match_groups_by_filename(&mut mp.name_match_groups_file1);
    sort_single_file_match_groups_by_filename(&mut mp.name_match_groups_file2);
    sort_match_groups_by_filename(&mut mp.name_match_groups);
    sort_match_pairs_by_filename(&mut mp.hashes_match_pairs);
    sort_single_file_match_groups_by_filename(&mut mp.hashes_match_groups_file1);
    sort_single_file_match_groups_by_filename(&mut mp.hashes_match_groups_file2);
    sort_match_groups_by_filename(&mut mp.hashes_match_groups);
    sort_log_entries_by_filename(&mut mp.no_match_file1);
    sort_log_entries_by_filename(&mut mp.no_match_file2);

    let total_from_both_files = from_file1.len().checked_add(from_file2.len());

    match (mp.total_log_entries(), total_from_both_files) {
        (Some(x), Some(y)) if x == y => Ok(mp),
        (Some(_), Some(_)) => Err(MatchPartitionError::ChecksumFailure),
        _ => Err(MatchPartitionError::ChecksumArithmeticOverflow),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::read_log_entries_from_file;

    #[test]
    fn match_partition_test() {

        let file1 = read_log_entries_from_file::<Vec<LogEntry>>("tests/partition_test1.txt").unwrap();
        let file2 = read_log_entries_from_file::<Vec<LogEntry>>("tests/partition_test2.txt").unwrap();
        assert_eq!(0, file1.invalid_lines.len());
        assert_eq!(0, file2.invalid_lines.len());

        let from_file1 = file1.entries.iter().collect::<Vec<&LogEntry>>();
        let from_file2 = file2.entries.iter().collect::<Vec<&LogEntry>>();

        let mp = match_partition(&from_file1, &from_file2).unwrap();

        assert_eq!(2, mp.full_match_pairs.len());
        assert_eq!(1, mp.full_match_groups_file1.len());
        assert_eq!(1, mp.full_match_groups_file2.len());
        assert_eq!(1, mp.full_match_groups.len());
        assert_eq!(1, mp.name_match_pairs.len());
        assert_eq!(1, mp.name_match_groups_file1.len());
        assert_eq!(1, mp.name_match_groups_file2.len());
        assert_eq!(1, mp.name_match_groups.len());
        assert_eq!(1, mp.hashes_match_pairs.len());
        assert_eq!(1, mp.hashes_match_groups_file1.len());
        assert_eq!(1, mp.hashes_match_groups_file2.len());
        assert_eq!(1, mp.hashes_match_groups.len());
        assert_eq!(1, mp.no_match_file1.len());
        assert_eq!(1, mp.no_match_file2.len());
    }
}