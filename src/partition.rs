use common;
use common::WhichFile::{File1,File2};
use log_entry::LogEntry;
use partitioner;


/// Partition entries from two hashdeep logs by content and name matches
///
/// Loads hashdeep logs from filename1 and filename2.
/// Entries in the loaded logs will be grouped in this order:
///
///	1. full match
///		1 in each file: no change
///		anomalies (invalid file)
///	2. only name match
///		1 in each file: changed file
///		anomalies (invalid file)
///	3. only content match
///		1 in each file: move/rename
///		match groups (unknown cause)
///	4. no match (list by origin)
///
/// Each log entry is guaranteed to be represented in exactly one group.
///
pub fn partition_log(filename1: &str, filename2: &str, output_filename_base: &str) -> ::std::io::Result<()> {

    let log_file1 = common::read_log_entries_from_file::<Vec<LogEntry>>(filename1, File1)?;
    assert_eq!(0, log_file1.invalid_lines.len());//todo: remove this

    let log_file2 = common::read_log_entries_from_file::<Vec<LogEntry>>(filename2, File2)?;
    assert_eq!(0, log_file2.invalid_lines.len());//todo: remove this

    let mut from_file1: Vec<&LogEntry> = log_file1.entries.iter().collect::<Vec<&LogEntry>>();
    let mut from_file2: Vec<&LogEntry> = log_file2.entries.iter().collect::<Vec<&LogEntry>>();

    let mp = partitioner::match_partition(&from_file1, &from_file2).unwrap(); //todo: remove unwrap

    common::write_match_pairs_to_file(&mp.full_match_pairs, format!("{}_full_match_pairs", output_filename_base).as_str())?;
    common::write_match_groups_to_file(&mp.full_match_groups.file1_only, format!("{}_full_match_groups_file1_only", output_filename_base).as_str())?;
    common::write_match_groups_to_file(&mp.full_match_groups.file2_only, format!("{}_full_match_groups_file2_only", output_filename_base).as_str())?;
    common::write_match_groups_to_file(&mp.full_match_groups.file1_and_file2, format!("{}_full_match_groups_file1_and_file2", output_filename_base).as_str())?;
    common::write_match_pairs_to_file(&mp.name_match_pairs, format!("{}_name_match_pairs", output_filename_base).as_str())?;
    common::write_match_groups_to_file(&mp.name_match_groups.file1_only, format!("{}_name_match_groups_file1_only", output_filename_base).as_str())?;
    common::write_match_groups_to_file(&mp.name_match_groups.file2_only, format!("{}_name_match_groups_file2_only", output_filename_base).as_str())?;
    common::write_match_groups_to_file(&mp.name_match_groups.file1_and_file2, format!("{}_name_match_groups_file1_and_file2", output_filename_base).as_str())?;
    common::write_match_pairs_to_file(&mp.hashes_match_pairs, format!("{}_hashes_match_pairs", output_filename_base).as_str())?;
    common::write_match_groups_to_file(&mp.hashes_match_groups.file1_only, format!("{}_hashes_match_groups_file1_only", output_filename_base).as_str())?;
    common::write_match_groups_to_file(&mp.hashes_match_groups.file2_only, format!("{}_hashes_match_groups_file2_only", output_filename_base).as_str())?;
    common::write_match_groups_to_file(&mp.hashes_match_groups.file1_and_file2, format!("{}_hashes_match_groups_file1_and_file2", output_filename_base).as_str())?;
    common::write_log_entries_to_file(&mp.no_match_file1, format!("{}_no_match_entries_file1", output_filename_base).as_str())?;
    common::write_log_entries_to_file(&mp.no_match_file2, format!("{}_no_match_entries_file2", output_filename_base).as_str())?;


    let mut stats_string = String::new();
    stats_string.push_str("log partition statistics:\n");
    stats_string.push_str("   (note: \"pairs\" have 1 entry in each file)\n");
    stats_string.push_str(format!(" {} full match pairs\n", mp.full_match_pairs.len()).as_str());
    stats_string.push_str(format!(" {} full match groups in file 1 only (should be 0)\n", mp.full_match_groups.file1_only.len()).as_str());
    stats_string.push_str(format!(" {} full match groups in file 2 only (should be 0)\n", mp.full_match_groups.file2_only.len()).as_str());
    stats_string.push_str(format!(" {} full match groups in both files (should be 0)\n", mp.full_match_groups.file1_and_file2.len()).as_str());
    stats_string.push_str(format!(" {} name match pairs\n", mp.name_match_pairs.len()).as_str());
    stats_string.push_str(format!(" {} name match groups in file 1 only (should be 0)\n", mp.name_match_groups.file1_only.len()).as_str());
    stats_string.push_str(format!(" {} name match groups in file 2 only (should be 0)\n", mp.name_match_groups.file2_only.len()).as_str());
    stats_string.push_str(format!(" {} name match groups in both files (should be 0)\n", mp.name_match_groups.file1_and_file2.len()).as_str());
    stats_string.push_str(format!(" {} hashes match pairs\n", mp.hashes_match_pairs.len()).as_str());
    stats_string.push_str(format!(" {} hashes match groups in file 1 only\n", mp.hashes_match_groups.file1_only.len()).as_str());
    stats_string.push_str(format!(" {} hashes match groups in file 2 only\n", mp.hashes_match_groups.file2_only.len()).as_str());
    stats_string.push_str(format!(" {} hashes match groups in both files\n", mp.hashes_match_groups.file1_and_file2.len()).as_str());
    stats_string.push_str(format!(" {} entries in file 1 with no match\n", mp.no_match_file1.len()).as_str());
    stats_string.push_str(format!(" {} entries in file 2 with no match\n", mp.no_match_file2.len()).as_str());

    println!("{}", stats_string);

    Ok(())
}