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
    common::write_log_entries_to_file(&mp.no_match.file1, format!("{}_no_match_entries_file1", output_filename_base).as_str())?;
    common::write_log_entries_to_file(&mp.no_match.file2, format!("{}_no_match_entries_file2", output_filename_base).as_str())?;


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
    stats_string.push_str(format!(" {} entries in file 1 with no match\n", mp.no_match.file1.len()).as_str());
    stats_string.push_str(format!(" {} entries in file 2 with no match\n", mp.no_match.file2.len()).as_str());

    println!("{}", stats_string);

    Ok(())
}