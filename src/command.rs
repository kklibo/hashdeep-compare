
pub fn get_hashdeep_command(target_directory: &str, output_path_base: &str) -> String {

    format!( "hashdeep -lro f \"{0}\" > \"{1}\" 2> \"{1}.errors\"", target_directory, output_path_base)
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