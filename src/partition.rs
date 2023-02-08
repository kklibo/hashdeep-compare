use crate::common;
use crate::log_entry::LogEntry;
use crate::partitioner;


#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PartitionLogSuccess
{
    /// Printable warning lines about the first hashdeep log file, if any were emitted
    pub file1_warning_lines: Option<Vec<String>>,
    /// Printable warning lines about the second hashdeep log file, if any were emitted
    pub file2_warning_lines: Option<Vec<String>>,
    /// Printable statistics about the partitioning results
    pub stats_string: String,
}

/// Partitions entries from two hashdeep logs by content and name matches.
///
/// hashdeep logs are loaded from filename1 and filename2, and output groups
/// are based on the output_filename_base path prefix.
///
/// Entries in the loaded logs will be grouped in this order:
///
/// 1. full match
///     1. 1 in each file: no change between logs
///     2. anomalies (invalid file)
/// 2. only name match
///     1. 1 in each file: file content changed between logs
///     2. anomalies (invalid file)
/// 3. only content match
///     1. 1 in each file: file moved/renamed between logs
///     2. match groups (unknown cause)
/// 4. no match (listed by origin)
///
/// Each log entry is guaranteed to be represented in exactly one group.
///
/// On success, returns a statistics string about the successful operation,
/// plus warning strings if any were emitted while loading the hashdeep logs.
///
/// # Errors
///
/// Any error emitted while reading or writing the files will be returned.
///
/// An integrity check is run on the partitioning results after calculation:
///  an error will be emitted if this fails (this is extremely unlikely).
///
pub fn partition_log(filename1: &str, filename2: &str, output_filename_base: &str) -> Result<PartitionLogSuccess, Box<dyn std::error::Error>> {

    let log_file1 = common::read_log_entries_from_file::<Vec<LogEntry>>(filename1)?;
    let log_file2 = common::read_log_entries_from_file::<Vec<LogEntry>>(filename2)?;

    let from_file1: Vec<&LogEntry> = log_file1.entries.iter().collect::<Vec<&LogEntry>>();
    let from_file2: Vec<&LogEntry> = log_file2.entries.iter().collect::<Vec<&LogEntry>>();

    let mp = partitioner::match_partition(&from_file1, &from_file2)?;

    common::write_match_pairs_to_file(&mp.full_match_pairs, format!("{output_filename_base}_full_match_pairs").as_str())?;
    common::write_single_file_match_groups_to_file(&mp.full_match_groups_file1, format!("{output_filename_base}_full_match_groups_file1_only").as_str())?;
    common::write_single_file_match_groups_to_file(&mp.full_match_groups_file2, format!("{output_filename_base}_full_match_groups_file2_only").as_str())?;
    common::write_match_groups_to_file(&mp.full_match_groups, format!("{output_filename_base}_full_match_groups_file1_and_file2").as_str())?;
    common::write_match_pairs_to_file(&mp.name_match_pairs, format!("{output_filename_base}_name_match_pairs").as_str())?;
    common::write_single_file_match_groups_to_file(&mp.name_match_groups_file1, format!("{output_filename_base}_name_match_groups_file1_only").as_str())?;
    common::write_single_file_match_groups_to_file(&mp.name_match_groups_file2, format!("{output_filename_base}_name_match_groups_file2_only").as_str())?;
    common::write_match_groups_to_file(&mp.name_match_groups, format!("{output_filename_base}_name_match_groups_file1_and_file2").as_str())?;
    common::write_match_pairs_to_file(&mp.hashes_match_pairs, format!("{output_filename_base}_hashes_match_pairs").as_str())?;
    common::write_single_file_match_groups_to_file(&mp.hashes_match_groups_file1, format!("{output_filename_base}_hashes_match_groups_file1_only").as_str())?;
    common::write_single_file_match_groups_to_file(&mp.hashes_match_groups_file2, format!("{output_filename_base}_hashes_match_groups_file2_only").as_str())?;
    common::write_match_groups_to_file(&mp.hashes_match_groups, format!("{output_filename_base}_hashes_match_groups_file1_and_file2").as_str())?;
    common::write_log_entries_to_file(&mp.no_match_file1, format!("{output_filename_base}_no_match_entries_file1").as_str())?;
    common::write_log_entries_to_file(&mp.no_match_file2, format!("{output_filename_base}_no_match_entries_file2").as_str())?;


    let mut stats_string = String::new();
    stats_string.push_str("log partition statistics:\n");
    stats_string.push_str("   (note: \"pairs\" have 1 entry in each file)\n");
    stats_string.push_str(format!(" {} full match pairs\n", mp.full_match_pairs.len()).as_str());
    stats_string.push_str(format!(" {} full match groups in file 1 only (should be 0)\n", mp.full_match_groups_file1.len()).as_str());
    stats_string.push_str(format!(" {} full match groups in file 2 only (should be 0)\n", mp.full_match_groups_file2.len()).as_str());
    stats_string.push_str(format!(" {} full match groups in both files (should be 0)\n", mp.full_match_groups.len()).as_str());
    stats_string.push_str(format!(" {} name match pairs\n", mp.name_match_pairs.len()).as_str());
    stats_string.push_str(format!(" {} name match groups in file 1 only (should be 0)\n", mp.name_match_groups_file1.len()).as_str());
    stats_string.push_str(format!(" {} name match groups in file 2 only (should be 0)\n", mp.name_match_groups_file2.len()).as_str());
    stats_string.push_str(format!(" {} name match groups in both files (should be 0)\n", mp.name_match_groups.len()).as_str());
    stats_string.push_str(format!(" {} hashes match pairs\n", mp.hashes_match_pairs.len()).as_str());
    stats_string.push_str(format!(" {} hashes match groups in file 1 only\n", mp.hashes_match_groups_file1.len()).as_str());
    stats_string.push_str(format!(" {} hashes match groups in file 2 only\n", mp.hashes_match_groups_file2.len()).as_str());
    stats_string.push_str(format!(" {} hashes match groups in both files\n", mp.hashes_match_groups.len()).as_str());
    stats_string.push_str(format!(" {} entries in file 1 with no match\n", mp.no_match_file1.len()).as_str());
    stats_string.push_str(format!(" {} entries in file 2 with no match\n", mp.no_match_file2.len()).as_str());

    Ok(PartitionLogSuccess
    {
        file1_warning_lines: log_file1.warning_report(),
        file2_warning_lines: log_file2.warning_report(),
        stats_string
    })
}