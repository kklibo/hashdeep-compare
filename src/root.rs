use crate::common::LogFile;
use crate::log_entry::LogEntry;
use crate::log_ops;

/// Reads a hashdeep log file and writes its entries to a new file, with
/// its root directory adjusted:
/// 1. file paths will have `root_prefix` removed
/// 2. entries with file paths that don't start with `root_prefix` will be omitted
///
/// On success, returns a Vec of warning strings, if any warnings were emitted while reading the file.
///
/// # Errors
///
/// Any error emitted while reading or writing the files will be returned.
pub fn change_root(filename: &str, out_filename: &str, root_prefix: &str)
    -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {

    let f = |log_file: &mut LogFile<Vec<LogEntry>>| {
        log_file.entries =
            log_file.entries.iter().filter_map(|log_entry| {
                log_entry.filename.strip_prefix(root_prefix).map(|new_path| {
                    LogEntry{
                        filename: new_path.to_string(),
                        hashes: log_entry.hashes.clone(),
                    }
                })
            }).collect();
    };

    log_ops::process_log(filename, out_filename, f)
}

#[cfg(test)]
mod test {
    use super::*;

    use predicates::prelude::*;

    #[test]
    fn change_root_test() {
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let temp_file = temp_dir.path().join("test.txt");
            let temp_file_path_str = temp_file.to_str().unwrap();

            change_root("tests/test1.txt", temp_file_path_str, "hashdeepComp/").unwrap();

            let p = predicates::path::eq_file("tests/test1_root_changed.txt");
            assert!(p.eval(temp_file.as_path()));
        }
    }
}