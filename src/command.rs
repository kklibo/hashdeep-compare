use std::process::{Command,Stdio};
use std::error::Error;

pub fn run_hashdeep_command(target_directory: &str, output_path_base: &str) -> Result<(), Box<dyn Error>> {

    use std::fs::File;

    if std::path::Path::new(output_path_base).exists() {
        return Err(format!("{} exists (will not overwrite existing files)", output_path_base).into());
    }

    Command::new("hashdeep")

    .arg("-l")
    .arg("-r")
    .arg("-o").arg("f")
    .arg(target_directory)

    .stdin(Stdio::null())
    .stdout(File::create(output_path_base)?)
    .stderr(File::create(format!("{}.errors", output_path_base))?)

    .status()?;
    Ok(())
}
