use std::process::{Command,Stdio,ExitStatus};

pub fn get_hashdeep_command(target_directory: &str, output_path_base: &str) -> String {

    format!( "hashdeep -lro f \"{0}\" > \"{1}\" 2> \"{1}.errors\"", target_directory, output_path_base)
}

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

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn get_hashdeep_command_test() {

        assert_eq!(r#"hashdeep -lro f "." > "~/filename with spaces" 2> "~/filename with spaces.errors""#,
            get_hashdeep_command(".", "~/filename with spaces")
        );

        assert_eq!(r#"hashdeep -lro f "/mnt/usb1/" > "/mnt/usb2/outfile" 2> "/mnt/usb2/outfile.errors""#,
            get_hashdeep_command("/mnt/usb1/", "/mnt/usb2/outfile")
        );

    }

}