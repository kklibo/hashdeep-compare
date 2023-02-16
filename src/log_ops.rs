//! Hashdeep log operations

use crate::common;
use crate::common::{HashdeepLogHeaderWarning, LogFile};
use crate::log_entry::LogEntry;

/// Processes a hashdeep log:
/// 1. Reads a hashdeep log file's contents into a `LogFile`.
/// 2. Runs `f` on that `LogFile` to process it.
/// 3. Writes the resulting `LogFile` contents to a new file.
///
/// On success, returns a Vec of warning strings, if any warnings were emitted while reading the file.
///
/// # Errors
///
/// Any error emitted while reading or writing the files will be returned.
pub fn process_log<T>(filename: &str, out_filename: &str, mut f: T)
    -> Result<Option<Vec<String>>, Box<dyn std::error::Error>>
    where T: FnMut(&mut LogFile<Vec<LogEntry>>)
{
    if std::path::Path::exists(out_filename.as_ref()) {
        return Err(common::WriteToFileError::OutputFileExists(out_filename.to_string()).into());
    }

    let mut log_file = common::read_log_entries_from_file::<Vec<LogEntry>>(filename)?;

    f(&mut log_file);

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
            *line = format!("## Modified by hashdeep-compare v{VERSION}");
        }
    }

    let warning_report = log_file.warning_report();

    common::write_log_file_to_file(log_file, out_filename)?;
    Ok(warning_report)
}