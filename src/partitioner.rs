pub mod match_pair {
    use log_entry::LogEntry;
    use common::WhichFile::*;

    pub struct MatchPair<'a> {
        pub from_file1: &'a LogEntry,
        pub from_file2: &'a LogEntry,
        _prevent_struct_literal: (),
    }
/*
    pub fn new<'a>(a: &'a LogEntry, b: &'a LogEntry) -> Result<MatchPair<'a>, (&'a LogEntry, &'a LogEntry)> {

        match a.origin {
            File1 => match b.origin {
                File1 => Err((a,b)),
                File2 => Ok(MatchPair{from_file1: a, from_file2: b, _prevent_struct_literal: ()}),
                SingleFile => Err((a,b)),
            },
            File2 => match b.origin {
                File1 => Ok(MatchPair{from_file1: b, from_file2: a, _prevent_struct_literal: ()}),
                File2 => Err((a,b)),
                SingleFile => Err((a,b)),
            },
            SingleFile => Err((a,b)),
        }
    }*/
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

    /*
    full match
		1 in each file: no change
		anomalies (invalid file)
	only name match
		1 in each file: changed file
		anomalies (invalid file)
	only content match
		1 in each file: move/rename
		match groups (unknown cause)
	no match (list by origin)
*/

}
/*
fn one_from_each_file(a: &LogEntry, b: &LogEntry) -> bool {
    (a.origin == File1 && b.origin == File2) ||
    (a.origin == File2 && b.origin == File1)
}*/

pub fn match_partition<'b>(log_entries: &Vec<&'b LogEntry>) -> MatchPartition<'b> {
/*
    // let mut full_matches = HashMap::<String, Vec<&LogEntry>>::new();
    let mut full_match_pairs: Vec<MatchPair>;
    let mut full_match_groups: Vec<MatchGroup>;

    // let mut name_matches = HashMap::<String, Vec<&LogEntry>>::new();
    let mut name_match_pairs: Vec<MatchPair>;
    let mut name_match_groups: Vec<MatchGroup>;

    // let mut hashes_matches = HashMap::<String, Vec<&LogEntry>>::new();
    let mut hashes_match_pairs: Vec<MatchPair>;
    let mut hashes_match_groups: Vec<MatchGroup>;

    let mut no_match: Vec<&LogEntry>;*/


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


    MatchPartition {
        full_match_pairs: full_matches.match_pairs,
        full_match_groups: full_matches.match_groups,

        name_match_pairs: name_matches.match_pairs,
        name_match_groups: name_matches.match_groups,

        hashes_match_pairs: hashes_matches.match_pairs,
        hashes_match_groups: hashes_matches.match_groups,

        no_match: hashes_matches.no_match,
    }

    //todo: check totals at end
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

        let mp = match_partition(&log_entries);

        assert_eq!(1, mp.full_match_pairs.len());
        assert_eq!(1, mp.full_match_groups.len());
        assert_eq!(1, mp.name_match_pairs.len());
        assert_eq!(1, mp.name_match_groups.len());
        assert_eq!(1, mp.hashes_match_pairs.len());
        assert_eq!(2, mp.hashes_match_groups.len());
        assert_eq!(2, mp.no_match.len());


    }
}