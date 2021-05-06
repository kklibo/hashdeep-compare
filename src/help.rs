use indoc::formatdoc;

pub fn help_string(version: &str) -> String {

    formatdoc!("
        hashdeep-compare v{} options:
            version
              Display version string

            help <hash|sort|part>
              Display subcommand help

            hash <target_directory> <output_path_base>
              Invoke hashdeep on a target directory

            sort <input_file> <output_file>
              Sort a hashdeep log (by file path)

            part <input_file1> <input_file2> <output_file_base>
              Partition contents of two hashdeep logs into category files",

        version
    )
}

pub fn help_hash_string() -> String {

    formatdoc!("
        hashdeep-compare function: hash

        Invokes hashdeep and generates a log file compatible with hashdeep-compare.

        Syntax:
            hashdeep-compare hash path/to/target_dir path/to/output_log.txt

        Notes:
            This function is optional, but recommended to ensure log compatibility.

            The above function call is equivalent to directly calling
                hashdeep -l -r -o f path/to/target_dir \\
                  > path/to/output_log.txt \\
                  2> path/to/output_log.txt.errors

            Note that if the output file or the error file already exist, the command
            will be aborted (hashdeep-compare will not overwrite existing files).
        "
    )
}

pub fn help_sort_string() -> String {

    formatdoc!("
        hashdeep-compare function: sort

        Sorts the entries in a hashdeep log by file path.

        Syntax:
            hashdeep-compare sort path/to/unsorted_input.txt path/to/sorted_output.txt

        Notes:
            hashdeep does not guarantee ordering of log entries, and ordering tends to
            be inconsistent between runs in practice. Sorting allows comparison of
            hashdeep logs in a text-diff tool, which may be the easiest way to compare
            logs with uncomplicated differences.

            Note that if the output file already exists, the command will be aborted
            (hashdeep-compare will not overwrite existing files).
        "
    )
}

pub fn help_part_string() -> String {

    formatdoc!("
        hashdeep-compare function: part

        The real power of hashdeep-compare:
        All entries will be partitioned into sets that efficiently describe the
        similarities and differences of the two log files.

        Syntax:
            hashdeep-compare part \\
                path/to/first_log.txt \\
                path/to/second_log.txt \\
                path/to/output_file_base

        Notes:
            The output file base path will be used to name the output files by adding
            suffixes that describe the log entries represented within; it may include
            subdirectories. Nonexistent subdirectories will not be created; if one is
            specified, the command will be aborted.

            Note that if any of the resulting output files already exist, the command
            will be aborted (hashdeep-compare will not overwrite existing files).
        "
    )
}