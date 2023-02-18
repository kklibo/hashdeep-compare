use crate::common::LogFile;
use crate::log_entry::LogEntry;
use crate::log_ops;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ChangeRootSuccess
{
    /// Printable warning lines about the hashdeep log file, if any were emitted
    pub file_warning_lines: Option<Vec<String>>,
    /// The number of entries that matched the prefix
    pub entries_matched: usize,
    /// The number of entries that were omitted (did not match the prefix)
    pub entries_omitted: usize,
}

impl ChangeRootSuccess {
    /// Printable info lines about the `change_root` operation
    pub fn info_lines(&self) -> Vec<String> {
        let mut v = vec![];

        let total_entries = self.entries_matched.checked_add(self.entries_omitted)
            .expect("entry stats should not cause arithmetic overflow");
        v.push(format!("Input file contains {total_entries} entries:"));

        match self.entries_matched {
            0 => {},
            x if x == total_entries => v.push(format!("  All {x} entries matched the prefix")),
            x => {
                v.push(format!("  {x} entries matched the prefix"));
                v.push(format!("  {} entries did not match the prefix and were omitted", self.entries_omitted));
            },
        }
        v
    }

    /// Printable warning lines about the `change_root` operation
    pub fn warning_lines(&self) -> Vec<String> {
        if self.entries_matched == 0 {
            vec![format!("Warning: No entries matched the prefix (All entries were omitted)")]
        }
        else { vec![] }
    }
}

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
    -> Result<ChangeRootSuccess, Box<dyn std::error::Error>> {

    let mut entry_count_before= 0;
    let mut entry_count_after= 0;

    let f = |log_file: &mut LogFile<Vec<LogEntry>>| {
        entry_count_before = log_file.entries.len();

        log_file.entries =
            log_file.entries.iter().filter_map(|log_entry| {
                log_entry.filename.strip_prefix(root_prefix).map(|new_path| {
                    LogEntry{
                        filename: new_path.to_string(),
                        hashes: log_entry.hashes.clone(),
                    }
                })
            }).collect();

        entry_count_after = log_file.entries.len();
    };

    let file_warning_lines = log_ops::process_log(filename, out_filename, f)?;
    let entries_matched = entry_count_after;
    // Safety: this will not overflow, because filtering can only remove entries.
    let entries_omitted = entry_count_before.checked_sub(entry_count_after)
        .expect("filter should not increase entry count");

    Ok(ChangeRootSuccess { file_warning_lines, entries_matched, entries_omitted })
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