use indoc::formatdoc;

/// Gets the main program help string
pub fn help_string(version: &str) -> String {
    format!("hashdeep-compare v{version}")
}

/// Gets the hash function's `clap` "long_about" string
pub fn long_about_hash_string() -> String {
    "Invokes hashdeep and generates a log file compatible with hashdeep-compare.".to_string()
}

/// Gets the hash function help string
pub fn help_hash_string() -> String {

    formatdoc!("
        Notes:
            This function is optional, but recommended to ensure log compatibility.

            The above function call is equivalent to directly calling
                hashdeep -l -r -o f path/to/target_dir \\
                  > path/to/output_log.txt \\
                  2> path/to/output_log.txt.errors

            Note that if the output file or the error file already exists, the command
            will be aborted (hashdeep-compare will not overwrite existing files).
        "
    )
}

/// Gets the sort function's `clap` "long_about" string
pub fn long_about_sort_string() -> String {
    "Sorts the entries in a hashdeep log by file path.".to_string()
}

/// Gets the sort function help string
pub fn help_sort_string() -> String {

    formatdoc!("
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

/// Gets the root function's `clap` "long_about" string
pub fn long_about_root_string() -> String {
    formatdoc!("
        Changes a hashdeep log root by removing a prefix from its filepaths.
        Any entries with filepaths that do not start with the prefix will be
        omitted from the output."
    )
}

/// Gets the root function help string
pub fn help_root_string() -> String {

    formatdoc!("
        Notes:
            This subcommand is an easy way to recover from a hashdeep run that prepended
            unintended parent directories on all of its filepaths because of its invocation
            directory.

            Warning: The prefix is applied as simple text, without any rules related to paths.
            If the prefix \"test\" were used on the filepath \"testdir/file.txt\",
            the resulting filepath would be \"dir/file.txt\".
            Splitting the text of a path component like this probably isn't what you want,
            but there may be some clever uses for it.

            Note that if the output file already exists, the command will be aborted
            (hashdeep-compare will not overwrite existing files).
        "
    )
}

/// Gets the part function's `clap` "long_about" string
pub fn long_about_part_string() -> String {
    formatdoc!("
        The real power of hashdeep-compare:
        All entries will be partitioned into sets that efficiently describe the
        similarities and differences of the two log files."
    )
}

/// Gets the part function help string
pub fn help_part_string() -> String {

    formatdoc!("
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