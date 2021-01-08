use common;
use log_entry::LogEntry;

pub fn sort_log(filename: &str, out_filename: &str) -> Result<(), Box<dyn std::error::Error>>{

    if std::path::Path::exists(out_filename.as_ref()) {
        return Err(format!("{} exists (will not overwrite existing files)", out_filename).into());
    }

    let mut log_file = common::read_log_entries_from_file::<Vec<LogEntry>>(filename)?;
    assert_eq!(0, log_file.invalid_lines.len());//todo: remove this

    log_file.entries.sort_by(|ref v1, ref v2| {

        v1.filename.cmp(&v2.filename)
    });

    common::write_log_entries_to_file(log_file.entries, out_filename)?;
    Ok(())
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
            let temp_dir = tempfile::tempdir().unwrap();
            let temp_file = temp_dir.path().join("test1 sorted.txt");
            let temp_file_path_str = temp_file.to_str().unwrap();

            sort_log("tests/test1.txt", temp_file_path_str).unwrap();

            let p = predicates::path::eq_file("tests/test1 sorted.txt");
            assert!(p.eval(temp_file.as_path()));
        }
    }
}