use std::process::{Command,Stdio};
use std::fs::OpenOptions;
use std::io::ErrorKind;

use thiserror::Error;
use anyhow::anyhow;
use which::which;

const CANNOT_FIND_BINARY_PATH_STR : &str = "external hashdeep binary cannot be found (is hashdeep installed?)";


#[derive(Error, Debug)]
pub enum RunHashdeepCommandError {

    #[error("{}",CANNOT_FIND_BINARY_PATH_STR)]
    CannotFindBinaryPath,

    #[error("{0} exists (will not overwrite existing files)")]
    OutputFileExists(String),

    #[error("{0} and {1} exist (will not overwrite existing files)")]
    OutputFilesExist(String, String),

    #[error("\"{0}\" cannot be opened for writing (does the directory exist?)")]
    OutputFileNotFound(String),

    #[error("\"{0}\" cannot be opened for writing ({})", .1)]
    OutputFileOtherError(String, #[source] std::io::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl RunHashdeepCommandError {

    fn new(e: std::io::Error, path: &str) -> Self {

        match e.kind() {
            ErrorKind::AlreadyExists => RunHashdeepCommandError::OutputFileExists(path.to_string()),
            ErrorKind::NotFound      => RunHashdeepCommandError::OutputFileNotFound(path.to_string()),
            ErrorKind::Other         => RunHashdeepCommandError::OutputFileOtherError(path.to_string(), e),
            _ => e.into(),
        }
    }
}

/// Runs hashdeep with the settings recommended for hashdeep-compare.
///
/// The log includes (recursively) all files and directories in `target_directory`,
/// and is written to `output_path_base`, with hashdeep's stderr
/// written to `output_path_base` + ".errors".
///
/// # Errors
///
/// An error will be returned if
/// * the `hashdeep` command is not available
/// * the output log file or error file already exist (will not overwrite existing files)
/// * any other error occurs while creating the output files
/// * any other error occurs when running `hashdeep`
pub fn run_hashdeep_command(
    target_directory: &str,
    output_path_base: &str,
    hashdeep_command_name: &str,
) -> Result<(), RunHashdeepCommandError> {

    //confirm availability of external hashdeep binary
    match which(hashdeep_command_name) {
        Err(which::Error::CannotFindBinaryPath) => return Err(RunHashdeepCommandError::CannotFindBinaryPath),
        Err(x) => return Err(anyhow!(x).into()),
        _ => ()
    };

    let error_log_suffix = ".errors";

    let output_error_path = format!("{}{}", output_path_base, error_log_suffix);


    //try to open both output files
    let maybe_output_file =
        OpenOptions::new().write(true).create_new(true).open(&output_path_base)
            .map_err(|e| RunHashdeepCommandError::new(e, output_path_base));

    let maybe_error_file  =
        OpenOptions::new().write(true).create_new(true).open(&output_error_path)
            .map_err(|e| RunHashdeepCommandError::new(e, &output_error_path));


    let (output_file, error_file) =

    match (maybe_output_file, maybe_error_file) {

        (Ok(output_file), Ok(error_file)) => (output_file, error_file),


        //if either file failed to open, abort the command and clean up:

        (Err(output_file_error), Ok(_)) => {

            //delete the file that was successfully created
            std::fs::remove_file(&output_error_path)?;

            return Err(output_file_error);
        },

        (Ok(_), Err(error_file_error)) => {

            //delete the file that was successfully created
            std::fs::remove_file(&output_path_base)?;

            return Err(error_file_error);
        },

        (Err(output_file_error), Err(error_file_error)) => {

            //if present, combine 2 OutputFileExists errors into 1 OutputFilesExist error
            return Err(
                if let ( RunHashdeepCommandError::OutputFileExists(file1),
                         RunHashdeepCommandError::OutputFileExists(file2) )
                        = (&output_file_error, &error_file_error)
                {
                    RunHashdeepCommandError::OutputFilesExist(file1.clone(), file2.clone())
                }
                else {
                    //otherwise, just return the output file's error
                    output_file_error
                }
            );
        }
    };

    Command::new(hashdeep_command_name)

    .arg("-l")
    .arg("-r")
    .arg("-o").arg("f")
    .arg(target_directory)

    .stdin(Stdio::null())
    .stdout(output_file)
    .stderr(error_file)

    .status()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_hashdeep_command_missing_hashdeep_test() {

        //directly test the 'no hashdeep' error
        //(could eventually be replaced if integration testing can somehow hide hashdeep)

        assert_eq!(CANNOT_FIND_BINARY_PATH_STR,

            run_hashdeep_command("fake_target_dir",
                                 "fake_output_path_base",
                                 "nonexistent_program_name_Cmn2TMmwGO9U2j7")
            .unwrap_err().to_string()
        );
    }
}