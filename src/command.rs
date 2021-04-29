use std::process::{Command,Stdio};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::ErrorKind;

pub fn run_hashdeep_command(target_directory: &str, output_path_base: &str) -> Result<(), Box<dyn Error>> {

    let error_log_suffix = ".errors";

    let output_error_path = format!("{}{}", output_path_base, error_log_suffix);


    //try to open both output files
    let maybe_output_file = OpenOptions::new().write(true).create_new(true).open(&output_path_base);
    let maybe_error_file  = OpenOptions::new().write(true).create_new(true).open(&output_error_path);


    let (output_file, error_file) =

    match (maybe_output_file, maybe_error_file) {

        (Ok(output_file), Ok(error_file)) => (output_file, error_file),


        //if either file failed to open, abort command and clean up:

        (Err(output_file_error), Ok(_)) => {

            //delete the file that was successfully created
            std::fs::remove_file(&output_error_path)?;

            return match output_file_error.kind() == ErrorKind::AlreadyExists {
                true  => Err(format!("{} exists (will not overwrite existing files)", output_path_base).into()),
                false => Err(output_file_error.into()),
            }
        },

        (Ok(_), Err(error_file_error)) => {

            //delete the file that was successfully created
            std::fs::remove_file(&output_path_base)?;

            return match error_file_error.kind() == ErrorKind::AlreadyExists {
                true  => Err(format!("{} exists (will not overwrite existing files)", output_error_path).into()),
                false => Err(error_file_error.into()),
            }
        },

        (Err(output_file_error), Err(error_file_error)) => {

            return match ( output_file_error.kind() == ErrorKind::AlreadyExists,
                            error_file_error.kind() == ErrorKind::AlreadyExists ) {

                (true,  false) => Err(format!("{} exists (will not overwrite existing files)", output_path_base).into()),
                (false, true ) => Err(format!("{} exists (will not overwrite existing files)", output_error_path).into()),
                (true,  true ) => Err(format!("{} and {} exist (will not overwrite existing files)", output_path_base, output_error_path).into()),

                //just return the output file's error (todo: do something more useful here?)
                (false, false) => Err(output_file_error.into()),
            };
        }
    };

    Command::new("hashdeep")

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
