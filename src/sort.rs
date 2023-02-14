use crate::common;
use crate::common::HashdeepLogHeaderWarning;
use crate::log_entry::LogEntry;

/// Reads a hashdeep log file and writes its entries to a new file, sorted by name.
///
/// On success, returns a Vec of warning strings, if any warnings were emitted while reading the file.
///
/// # Errors
///
/// Any error emitted while reading or writing the files will be returned.
pub fn sort_log(filename: &str, out_filename: &str) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>>{

    if std::path::Path::exists(out_filename.as_ref()) {
        return Err(common::WriteToFileError::OutputFileExists(out_filename.to_string()).into());
    }

    let mut log_file = common::read_log_entries_from_file::<Vec<LogEntry>>(filename)?;

    log_file.entries.sort_by(|v1, v2| {

        v1.filename.cmp(&v2.filename)
    });

    fn should_skip_header_note(warning: &HashdeepLogHeaderWarning) -> bool {
        matches!(warning,
            HashdeepLogHeaderWarning::HeaderNotFound |
            HashdeepLogHeaderWarning::UnexpectedHeaderLineCount(_) |
            HashdeepLogHeaderWarning::Unexpected5thLineContent(_)
        )
    }

    // Unless any disqualifying header warnings are found,
    // add a note to the 5th line of the header.
    if ! log_file.header_warnings.iter().any(should_skip_header_note) {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        if let Some(line) = log_file.header_lines.get_mut(4) {
            *line = format!("## Sorted by hashdeep-compare v{VERSION}");
        }
    }

    let warning_report = log_file.warning_report();

    common::write_log_file_to_file(log_file, out_filename)?;
    Ok(warning_report)
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