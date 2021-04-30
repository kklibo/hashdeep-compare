use indoc::formatdoc;

pub fn help_string(version: &str) -> String {

    formatdoc!("
        hashdeep-compare v{}

          Options:

            version
              Display version string

            hash <target_directory> <output_path_base>
              Invoke hashdeep on a target directory

            sort <input_file> <output_file>
              Sort a hashdeep log (by file path)

            part <input_file1> <input_file2> <output_file_base>
              Partition contents of two hashdeep logs into category files",

        version
    )
}