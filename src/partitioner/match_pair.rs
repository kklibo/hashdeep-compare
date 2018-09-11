use log_entry::LogEntry;
use common::WhichFile::*;

#[derive(Eq, PartialEq, Debug)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn maybe_match_pair_test() {
        let from_file1_1 = LogEntry::from_str("1,a,b,filename1_1", File1).expect("");
        let from_file1_2 = LogEntry::from_str("2,c,d,filename1_2", File1).expect("");
        let from_file2 = LogEntry::from_str("3,e,f,filename2", File2).expect("");
        let from_single_file1 = LogEntry::from_str("4,g,h,filename_s1", SingleFile).expect("");
        let from_single_file2 = LogEntry::from_str("5,i,j,filename_s2", SingleFile).expect("");

        assert_eq!(MatchPair::maybe_match_pair(&from_file1_1, &from_file1_1), None);
        assert_eq!(MatchPair::maybe_match_pair(&from_file1_1, &from_file1_2), None);
        assert_eq!(MatchPair::maybe_match_pair(&from_file1_2, &from_file1_1), None);
        assert_eq!(MatchPair::maybe_match_pair(&from_file1_1, &from_single_file1), None);
        assert_eq!(MatchPair::maybe_match_pair(&from_single_file2, &from_file1_2), None);
        assert_eq!(MatchPair::maybe_match_pair(&from_single_file1, &from_single_file2), None);

        assert_eq!(MatchPair::maybe_match_pair(&from_file1_1, &from_file2),
                   Some(MatchPair{from_file1: &from_file1_1, from_file2: &from_file2, _prevent_struct_literal: ()}));

        assert_eq!(MatchPair::maybe_match_pair(&from_file2, &from_file1_2),
                   Some(MatchPair{from_file1: &from_file1_2, from_file2: &from_file2, _prevent_struct_literal: ()}));
    }
}
