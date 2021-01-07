use std::process::{Command,Stdio};
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn run_hashdeep_command(target_directory: &str, output_path_base: &str) -> Result<(), Box<dyn Error>> {

    let error_log_suffix = ".errors";

    let output_error_path = format!("{}{}", output_path_base, error_log_suffix);

    match ( Path::new(output_path_base).exists(), Path::new(&output_error_path).exists() ) {
        (true,  false) => return Err(format!("{} exists (will not overwrite existing files)", output_path_base).into()),
        (false, true ) => return Err(format!("{} exists (will not overwrite existing files)", output_error_path).into()),
        (true,  true ) => return Err(format!("{} and {} exist (will not overwrite existing files)", output_path_base, output_error_path).into()),
        (false, false) => (),
    };

    Command::new("hashdeep")

    .arg("-l")
    .arg("-r")
    .arg("-o").arg("f")
    .arg(target_directory)

    .stdin(Stdio::null())
    .stdout(File::create(output_path_base)?)
    .stderr(File::create(output_error_path)?)

    .status()?;
    Ok(())
}
