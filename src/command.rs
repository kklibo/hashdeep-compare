use std::process::{Command,Stdio,ExitStatus};

pub fn run_hashdeep_command(target_directory: &str, output_path_base: &str) -> std::io::Result<ExitStatus> {

    use std::fs::File;

    Command::new("hashdeep")

    .arg("-l")
    .arg("-r")
    .arg("-o").arg("f")
    .arg(target_directory)

    .stdin(Stdio::null())
    .stdout(File::create(output_path_base)?)
    .stderr(File::create(format!("{}.errors", output_path_base))?)

    .status()
}
