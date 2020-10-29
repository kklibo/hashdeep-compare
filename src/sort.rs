use common;
use log_entry::LogEntry;

pub fn sort_log(filename: &str, out_filename: &str) -> ::std::io::Result<()>{

    let mut log_file = common::read_log_entries_from_file::<Vec<LogEntry>>(filename)?;
    assert_eq!(0, log_file.invalid_lines.len());//todo: remove this

    log_file.entries.sort_by(|ref v1, ref v2| {

        v1.filename.cmp(&v2.filename)
    });

    common::write_log_entries_to_file(log_file.entries, out_filename)
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate tempfile;
    extern crate predicates;

    use self::tempfile::NamedTempFile;
    use self::predicates::prelude::*;

    #[test]
    fn sort_log_test() {
        {
            let temp_file = NamedTempFile::new().unwrap();
            let temp_file_path_str = temp_file.path().to_str().unwrap();

            sort_log("tests/test1.txt", temp_file_path_str).unwrap();

            let p = predicates::path::eq_file("tests/test1 sorted.txt");
            assert!(p.eval(temp_file.path()));
        }
    }
}