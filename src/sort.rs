use crate::common::LogFile;
use crate::log_entry::LogEntry;
use crate::log_ops;

/// Reads a hashdeep log file and writes its entries to a new file, sorted by name.
///
/// On success, returns a Vec of warning strings, if any warnings were emitted while reading the file.
///
/// # Errors
///
/// Any error emitted while reading or writing the files will be returned.
pub fn sort_log(filename: &str, out_filename: &str) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>>{

    fn sort_entries(log_file: &mut LogFile<Vec<LogEntry>>) {
        log_file.entries.sort_by(|v1, v2| {
            v1.filename.cmp(&v2.filename)
        });
    }

    log_ops::process_log(filename, out_filename, sort_entries)
}

#[cfg(test)]
mod test {
    use super::*;

    use predicates::prelude::*;

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