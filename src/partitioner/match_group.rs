use log_entry::LogEntry;
use common::WhichFile::{File1,File2};

pub struct MatchGroup<'a> {
    pub entries: Vec<&'a LogEntry>,
}

pub struct MatchGroupsByOrigin<'d> {
    pub file1_only: Vec<MatchGroup<'d>>,
    pub file2_only: Vec<MatchGroup<'d>>,
    pub file1_and_file2: Vec<MatchGroup<'d>>,
}

//todo: make this private/necessary constructor
pub fn match_groups_by_origin(match_groups: Vec<MatchGroup>) -> MatchGroupsByOrigin {

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
