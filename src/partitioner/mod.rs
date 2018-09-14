pub mod match_pair;
pub mod match_group;

use self::match_pair::MatchPair;
use self::match_group::{MatchGroup,MatchGroupsByOrigin, match_groups_by_origin};
use std::collections::HashMap;
use log_entry::LogEntry;

pub struct MatchPartition<'a> {

    pub full_match_pairs: Vec<MatchPair<'a>>,
    pub full_match_groups: MatchGroupsByOrigin<'a>,

    pub name_match_pairs: Vec<MatchPair<'a>>,
    pub name_match_groups: MatchGroupsByOrigin<'a>,

    pub hashes_match_pairs: Vec<MatchPair<'a>>,
    pub hashes_match_groups: MatchGroupsByOrigin<'a>,

    pub no_match_file1: Vec<&'a LogEntry>,
    pub no_match_file2: Vec<&'a LogEntry>,
}

impl<'a> MatchPartition<'a> {

    fn total_log_entries(&self) -> Option<usize> {

        fn pairs_sum(pairs: &Vec<MatchPair>) -> Option<usize> {
            pairs.iter().try_fold(0usize, |acc, _| acc.checked_add(2))
            //todo: replace this with checked add of checked multiplication of len()?
        }
        fn groups_sum(groups: &Vec<MatchGroup>) -> Option<usize> {
            groups.iter().try_fold(0usize, |acc, ref x| {
                x.from_file1.len().checked_add(x.from_file2.len())
                    .and_then(|x| acc.checked_add(x))
            })
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
            vec_sum(&self.no_match_file1),
            vec_sum(&self.no_match_file2),
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

pub fn match_partition<'b>(from_file1: &Vec<&'b LogEntry>, from_file2: &Vec<&'b LogEntry>) -> Result<MatchPartition<'b>, MatchPartitionError> {

    struct SortedMatches<'a> {
        match_pairs: Vec<MatchPair<'a>>,
        match_groups: Vec<MatchGroup<'a>>,
        no_match_file1: Vec<&'a LogEntry>,
        no_match_file2: Vec<&'a LogEntry>,
    }

    fn sort_matches<'c, F>(from_file1: &Vec<&'c LogEntry>, from_file2: &Vec<&'c LogEntry>, f: F) -> SortedMatches<'c>
        where F: Fn(&LogEntry) -> String
    {
        enum LogEntryFrom<'a> {
            File1(&'a LogEntry),
            File2(&'a LogEntry),
        }

        let mut matches = HashMap::<String, Vec<LogEntryFrom>>::new();
        let mut match_pairs = Vec::<MatchPair>::new();
        let mut match_groups = Vec::<MatchGroup>::new();
        let mut no_match_file1 = Vec::<&LogEntry>::new();
        let mut no_match_file2 = Vec::<&LogEntry>::new();

        for &i in from_file1 {
            matches.entry(f(i)).or_insert(Vec::<LogEntryFrom>::new()).push(LogEntryFrom::File1(i));
        }

        for &i in from_file2 {
            matches.entry(f(i)).or_insert(Vec::<LogEntryFrom>::new()).push(LogEntryFrom::File2(i));
        }

        for (_, v) in matches {
            match v.len() {
                //todo b: check for 0 here?
                1 => match v[0] {
                    LogEntryFrom::File1(x) => no_match_file1.push(x),
                    LogEntryFrom::File2(x) => no_match_file2.push(x),
                },
                2 => {  //todo b: do better match patterns here?
                    match v[0] {
                        LogEntryFrom::File1(x) => {
                            match v[1] {
                                LogEntryFrom::File1(y) => match_groups.push(MatchGroup{from_file1: vec![x,y], from_file2: vec![]}),
                                LogEntryFrom::File2(y) => match_pairs.push(MatchPair{from_file1: x, from_file2: y}),
                            }
                        },
                        LogEntryFrom::File2(x) => {
                            match v[1] {
                                LogEntryFrom::File1(y) => match_pairs.push(MatchPair{from_file1: y, from_file2: x}),
                                LogEntryFrom::File2(y) => match_groups.push(MatchGroup{from_file1: vec![], from_file2: vec![x,y]}),
                            }
                        },
                    }
                },
                _ => {
                    let mut from_file1 = Vec::<&LogEntry>::new();
                    let mut from_file2 = Vec::<&LogEntry>::new();

                    for i in v {
                        match i {
                            LogEntryFrom::File1(x) => from_file1.push(x),
                            LogEntryFrom::File2(x) => from_file2.push(x),
                        }
                    }
                    match_groups.push(MatchGroup{from_file1, from_file2});
                }
            }
        }

        SortedMatches { match_pairs, match_groups, no_match_file1, no_match_file2 }
    }

    let full_matches = sort_matches(from_file1, from_file2, |x| x.source_text());

    let name_matches = sort_matches(&full_matches.no_match_file1, &full_matches.no_match_file2, |ref x| x.filename.clone());

    let hashes_matches = sort_matches(&name_matches.no_match_file1, &name_matches.no_match_file2, |ref x| x.hashes.clone());

    let mut mp = MatchPartition {

        full_match_pairs: full_matches.match_pairs,
        full_match_groups: match_groups_by_origin(full_matches.match_groups),

        name_match_pairs: name_matches.match_pairs,
        name_match_groups: match_groups_by_origin(name_matches.match_groups),

        hashes_match_pairs: hashes_matches.match_pairs,
        hashes_match_groups: match_groups_by_origin(hashes_matches.match_groups),

        no_match_file1: hashes_matches.no_match_file1,
        no_match_file2: hashes_matches.no_match_file2,
    };

    fn sort_match_pairs_by_filename(x: &mut Vec<MatchPair>) {
        x.sort_by(|a, b| a.from_file1.filename.cmp(&b.from_file1.filename));
    }

    fn sort_match_groups_by_filename(x: &mut Vec<MatchGroup>) {

        x.into_iter().for_each(|x| {
            sort_log_entries_by_filename(&mut x.from_file1);
            sort_log_entries_by_filename(&mut x.from_file2);
        });

        x.sort_by(|a, b| {

            fn get_first<'a>(mg: &'a MatchGroup) -> Option<&'a&'a LogEntry> {
                mg.from_file1.first().or(mg.from_file2.first())
            }

            match (get_first(a), get_first(b)) {
                (Some(x), Some(y)) => x.filename.cmp(&y.filename),
                _ => panic!("sort_match_groups_by_filename: empty MatchGroup") //todo b: replace this
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
    sort_log_entries_by_filename(&mut mp.no_match_file1);
    sort_log_entries_by_filename(&mut mp.no_match_file2);

    match mp.total_log_entries() {
        Some(t) if t == from_file1.len() + from_file2.len() => Ok(mp), //todo b: check for addition overflow
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

        let from_file1 = file1.entries.iter().collect::<Vec<&LogEntry>>();
        let from_file2 = file2.entries.iter().collect::<Vec<&LogEntry>>();

        let mp = match_partition(&from_file1, &from_file2).unwrap();

        assert_eq!(2, mp.full_match_pairs.len());
        assert_eq!(1, mp.full_match_groups.file1_only.len());
        assert_eq!(1, mp.full_match_groups.file2_only.len());
        assert_eq!(1, mp.full_match_groups.file1_and_file2.len());
        assert_eq!(1, mp.name_match_pairs.len());
        assert_eq!(1, mp.name_match_groups.file1_only.len());
        assert_eq!(1, mp.name_match_groups.file2_only.len());
        assert_eq!(1, mp.name_match_groups.file1_and_file2.len());
        assert_eq!(1, mp.hashes_match_pairs.len());
        assert_eq!(1, mp.hashes_match_groups.file1_only.len());
        assert_eq!(1, mp.hashes_match_groups.file2_only.len());
        assert_eq!(1, mp.hashes_match_groups.file1_and_file2.len());
        assert_eq!(1, mp.no_match_file1.len());
        assert_eq!(1, mp.no_match_file2.len());
    }
}