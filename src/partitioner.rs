use match_pair::MatchPair;
use std::collections::HashMap;
use log_entry::LogEntry;

pub struct MatchGroup<'a> {
    pub entries: Vec<&'a LogEntry>,
}

use common::WhichFile::{File1, File2};
pub struct MatchGroupsByOrigin<'d> {
    pub file1_only: Vec<MatchGroup<'d>>,
    pub file2_only: Vec<MatchGroup<'d>>,
    pub file1_and_file2: Vec<MatchGroup<'d>>,
}

fn match_groups_by_origin(match_groups: Vec<MatchGroup>) -> MatchGroupsByOrigin {

    let mut file1_only: Vec<MatchGroup> = Vec::new();
    let mut file2_only: Vec<MatchGroup> = Vec::new();
    let mut file1_and_file2: Vec<MatchGroup> = Vec::new();

    for match_group in match_groups {

        let f1 = match_group.entries.iter().any(|&x| x.origin == File1);
        let f2 = match_group.entries.iter().any(|&x| x.origin == File2);

        match f1 {
            true => match f2 {
                true => &mut file1_and_file2,
                false => &mut file1_only,
            },
            false => match f2 {
                true => &mut file2_only,
                false => panic!("match_groups_by_origin invalid file origin"),    //todo: remove this
            },
        }.push(match_group);
    }

    MatchGroupsByOrigin{file1_only, file2_only, file1_and_file2}
}

pub struct LogEntriesByOrigin<'a> {
    pub file1: Vec<&'a LogEntry>,
    pub file2: Vec<&'a LogEntry>,
}

fn log_entries_by_origin(log_entries: Vec<&LogEntry>) -> LogEntriesByOrigin {

    let mut file1: Vec<&LogEntry> = Vec::new();
    let mut file2: Vec<&LogEntry> = Vec::new();

    for log_entry in log_entries {

        match log_entry.origin {
            File1 => file1.push(log_entry),
            File2 => file2.push(log_entry),
            _ => panic!("log_entries_by_origin invalid file origin"),
        }
    }

    LogEntriesByOrigin{file1, file2}
}

pub struct MatchPartition<'a> {

    pub full_match_pairs: Vec<MatchPair<'a>>,
    pub full_match_groups: MatchGroupsByOrigin<'a>,

    pub name_match_pairs: Vec<MatchPair<'a>>,
    pub name_match_groups: MatchGroupsByOrigin<'a>,

    pub hashes_match_pairs: Vec<MatchPair<'a>>,
    pub hashes_match_groups: MatchGroupsByOrigin<'a>,

    pub no_match: LogEntriesByOrigin<'a>
}

impl<'a> MatchPartition<'a> {

    fn total_log_entries(&self) -> Option<usize> {

        fn pairs_sum(pairs: &Vec<MatchPair>) -> Option<usize> {
            pairs.iter().try_fold(0usize, |acc, _| acc.checked_add(2))
        }
        fn groups_sum(groups: &Vec<MatchGroup>) -> Option<usize> {
            groups.iter().try_fold(0usize, |acc, ref x| acc.checked_add(x.entries.len()))
        }
        fn vec_sum(v: &Vec<&LogEntry>) -> Option<usize> {
            Some(v.len())
        }

        (vec!{
            pairs_sum(&self.full_match_pairs),
            groups_sum(&self.full_match_groups.file1_only),
            groups_sum(&self.full_match_groups.file2_only),
            groups_sum(&self.full_match_groups.file1_and_file2),
            pairs_sum(&self.name_match_pairs),
            groups_sum(&self.name_match_groups.file1_only),
            groups_sum(&self.name_match_groups.file2_only),
            groups_sum(&self.name_match_groups.file1_and_file2),
            pairs_sum(&self.hashes_match_pairs),
            groups_sum(&self.hashes_match_groups.file1_only),
            groups_sum(&self.hashes_match_groups.file2_only),
            groups_sum(&self.hashes_match_groups.file1_and_file2),
            vec_sum(&self.no_match.file1),
            vec_sum(&self.no_match.file2),
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

    let mut mp = MatchPartition {

        full_match_pairs: full_matches.match_pairs,
        full_match_groups: match_groups_by_origin(full_matches.match_groups),

        name_match_pairs: name_matches.match_pairs,
        name_match_groups: match_groups_by_origin(name_matches.match_groups),

        hashes_match_pairs: hashes_matches.match_pairs,
        hashes_match_groups: match_groups_by_origin(hashes_matches.match_groups),

        no_match: log_entries_by_origin(hashes_matches.no_match),
    };

    fn sort_match_pairs_by_filename(x: &mut Vec<MatchPair>) {
        x.sort_by(|a, b| a.from_file1.filename.cmp(&b.from_file1.filename));
    }

    fn sort_match_groups_by_filename(x: &mut Vec<MatchGroup>) {

        x.into_iter().for_each(|x| sort_log_entries_by_filename(&mut x.entries));

        x.sort_by(|a, b| {
            if a.entries.len() > 0 && b.entries.len() > 0 {
                a.entries[0].filename.cmp(&b.entries[0].filename)
            }
            else {
                panic!("sort_match_groups_by_filename: empty MatchGroup") //todo: replace this
            }
        });
    }

    fn sort_log_entries_by_filename(x: &mut Vec<&LogEntry>) {
        x.sort_by(|a, b| a.filename.cmp(&b.filename));
    }

    sort_match_pairs_by_filename(&mut mp.full_match_pairs);
    sort_match_groups_by_filename(&mut mp.full_match_groups.file1_only);
    sort_match_groups_by_filename(&mut mp.full_match_groups.file2_only);
    sort_match_groups_by_filename(&mut mp.full_match_groups.file1_and_file2);
    sort_match_pairs_by_filename(&mut mp.name_match_pairs);
    sort_match_groups_by_filename(&mut mp.name_match_groups.file1_only);
    sort_match_groups_by_filename(&mut mp.name_match_groups.file2_only);
    sort_match_groups_by_filename(&mut mp.name_match_groups.file1_and_file2);
    sort_match_pairs_by_filename(&mut mp.hashes_match_pairs);
    sort_match_groups_by_filename(&mut mp.hashes_match_groups.file1_only);
    sort_match_groups_by_filename(&mut mp.hashes_match_groups.file2_only);
    sort_match_groups_by_filename(&mut mp.hashes_match_groups.file1_and_file2);
    sort_log_entries_by_filename(&mut mp.no_match.file1);
    sort_log_entries_by_filename(&mut mp.no_match.file2);

    match mp.total_log_entries() {
        Some(t) if t == log_entries.len() => Ok(mp),
        Some(_) => Err(MatchPartitionError::ChecksumFailure),
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
        assert_eq!(0, mp.full_match_groups.file1_only.len());
        assert_eq!(0, mp.full_match_groups.file2_only.len());
        assert_eq!(1, mp.full_match_groups.file1_and_file2.len());
        assert_eq!(1, mp.name_match_pairs.len());
        assert_eq!(0, mp.name_match_groups.file1_only.len());
        assert_eq!(0, mp.name_match_groups.file2_only.len());
        assert_eq!(1, mp.name_match_groups.file1_and_file2.len());
        assert_eq!(1, mp.hashes_match_pairs.len());
        assert_eq!(1, mp.hashes_match_groups.file1_only.len());
        assert_eq!(0, mp.hashes_match_groups.file2_only.len());
        assert_eq!(1, mp.hashes_match_groups.file1_and_file2.len());
        assert_eq!(1, mp.no_match.file1.len());
        assert_eq!(1, mp.no_match.file2.len());
    }
}