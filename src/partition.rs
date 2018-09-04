use common;
use common::WhichFile::{File1,File2};
use log_entry::LogEntry;
use partitioner;

pub fn partition_log(filename1: &str, filename2: &str, output_filename_base: &str) -> ::std::io::Result<()> {

    let log_file1 = common::read_log_entries_from_file::<Vec<LogEntry>>(filename1, File1)?;
    assert_eq!(0, log_file1.invalid_lines.len());//todo: remove this

    let log_file2 = common::read_log_entries_from_file::<Vec<LogEntry>>(filename2, File2)?;
    assert_eq!(0, log_file2.invalid_lines.len());//todo: remove this

    let mut log_entries: Vec<&LogEntry> = Vec::new();
    log_entries.append( &mut log_file1.entries.iter().collect::<Vec<&LogEntry>>());
    log_entries.append( &mut log_file2.entries.iter().collect::<Vec<&LogEntry>>());

    let mp = partitioner::match_partition(&log_entries).unwrap(); //todo: remove unwrap

    common::write_match_pairs_to_file(&mp.full_match_pairs, format!("{}_full_match_pairs", output_filename_base).as_str());
    common::write_match_groups_to_file(&mp.full_match_groups, format!("{}_full_match_groups", output_filename_base).as_str());
    common::write_match_pairs_to_file(&mp.name_match_pairs, format!("{}_name_match_pairs", output_filename_base).as_str());
    common::write_match_groups_to_file(&mp.name_match_groups, format!("{}_name_match_groups", output_filename_base).as_str());
    common::write_match_pairs_to_file(&mp.hashes_match_pairs, format!("{}_hashes_match_pairs", output_filename_base).as_str());
    common::write_match_groups_to_file(&mp.hashes_match_groups, format!("{}_hashes_match_groups", output_filename_base).as_str());
    common::write_log_entries_to_file(&mp.no_match, format!("{}_no_match_entries", output_filename_base).as_str());

    Ok(())
}