//! The **main_impl** module exists to add I/O redirection to the main program function, to allow
//! test coverage analysis for integration tests.
//!
//! ## Normal Program Behavior
//!
//! *main()* in main.rs calls *main_io_wrapper* in this module, supplying arguments and the standard
//! output and error streams.
//! *main_io_wrapper*, in turn, calls *main_impl*, which is the equivalent of a typical *main()*
//! function.
//!
//! ## Special handling: the **integration_test_coverage** feature
//!
//! Integration tests are defined in tests/integration.rs. These tests invoke the compiled program
//! binary and test its functions with command-line arguments. Because they call a
//! separate binary instead of code in hashdeep-compare's codebase, code coverage tools can't observe
//! what code they use.
//!
//! To work around this, hashdeep-compare has an **integration_test_coverage** feature. When enabled,
//! integration.rs uses a modified test function that runs the same tests as before, but through direct
//! calls in the codebase, instead of through a precompiled binary. The *main_io_wrapper* function is
//! its interface in this mode. These direct calls are part of the test binary, and can be observed by
//! a code coverage tool.
//!
//! ## Justification
//!
//! Separating the program's main function code between main.rs and **main_impl** does add some
//! complexity to the codebase.
//! The option of doing integration testing through direct calls to *main_io_wrapper*, as the
//! **integration_test_coverage** mode does now, would test almost all of the relevant code: the only code bypassed
//! is the code in *main()* in main.rs, which is a minimal wrapper around *main_io_wrapper*. However,
//! actual invocation through a separate binary provides a higher level of certainty against potential
//! future anomalies that disrupt the creation of the binary itself, or the processing of its inputs.
//! Integration testing with binaries is meant to replicate the actual use of the tool as closely as
//! possible: for this reason, the **integration_test_coverage** feature-based mode switch is preferred.

use crate::*;
use std::error::Error;
use std::io::Write;
use clap::{Parser, Subcommand};

/// Specifies program arguments and (re)direction of stdout/stderr, then runs the program
///
/// Returns the program's exit code
///
/// This is called by
/// - main() in normal execution
/// - integration.rs when the **integration_test_coverage** feature is enabled
pub fn main_io_wrapper(
    args: &[&str],
    mut stdout: impl Write,
    mut stderr: impl Write,
) -> Result<i32, Box<dyn Error>> {

    let exit_code =
    match main_impl(args, &mut stdout, &mut stderr)
    {
        Ok(()) => 0,
        Err(err) => {
            if let Some(err) = err.downcast_ref::<clap::Error>() {
                if err.use_stderr() {
                    write! (stderr, "{err}")?;
                    // Code 2 on error matches `clap`'s behavior.
                    2
                }
                else {
                    write! (stdout, "{err}")?;
                    0
                }
            }
            else {
                //conditionally use Display output for thiserror-based error types
                if let Some(err) = err.downcast_ref::<command::RunHashdeepCommandError>() {
                    writeln! (stderr, "Error: \"{err}\"")?;
                }
                else if let Some(err) = err.downcast_ref::<common::ReadLogEntriesFromFileError>() {
                    writeln! (stderr, "Error: \"{err}\"")?;
                }
                else if let Some(err) = err.downcast_ref::<common::WriteToFileError>() {
                    writeln! (stderr, "Error: \"{err}\"")?;
                }
                else if let Some(err) = err.downcast_ref::<partitioner::MatchPartitionError>() {
                    writeln! (stderr, "Error: \"{err}\"")?;
                }
                else {
                    writeln! (stderr, "Error: {err:?}")?;
                }

                1
            }
        }
    };

    Ok(exit_code)
}


fn print_hashdeep_log_warnings (
    filename: &str,
    warning_lines: Option<Vec<String>>,
    stderr: &mut impl Write) -> Result<(), Box<dyn Error>>
{
    if let Some(v) = warning_lines {
        writeln!(stderr, "Warnings emitted for hashdeep log at: {filename}")?;
        for line in v {
            writeln!(stderr, "  {line}")?;
        }
    }
    Ok(())
}

/// Called by main_io_wrapper: Accepts program arguments and runs the program
///
/// (This was the main() function before the **integration_test_coverage** feature was added)
fn main_impl(args: &[&str], stdout: &mut impl Write, stderr: &mut impl Write) -> Result<(), Box<dyn Error>> {

    const VERSION: &str = env!("CARGO_PKG_VERSION");

    #[derive(Parser, Debug)]
    #[command(about = help::help_string(VERSION))]
    struct CliArgs {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand, Debug)]
    #[command(disable_help_flag = true)]
    enum Commands {
        /// Display version string
        Version,
        #[command(after_long_help = help::help_hash_string())]
        #[command(long_about = help::long_about_hash_string())]
        /// Invoke hashdeep on a target directory
        Hash {
            #[arg(hide_long_help = true, id="path/to/target_dir")]
            target_directory: String,
            #[arg(hide_long_help = true, id="path/to/output_log.txt")]
            output_path_base: String,
        },
        #[command(after_long_help = help::help_sort_string())]
        #[command(long_about = help::long_about_sort_string())]
        /// Sort a hashdeep log (by file path)
        Sort {
            #[arg(hide_long_help = true, id="path/to/unsorted_input.txt")]
            input_file: String,
            #[arg(hide_long_help = true, id="path/to/sorted_output.txt")]
            output_file: String,
        },
        #[command(after_long_help = help::help_part_string())]
        #[command(long_about = help::long_about_part_string())]
        /// Partition contents of two hashdeep logs into category files
        Part {
            #[arg(hide_long_help = true, id="path/to/first_log.txt")]
            input_file1: String,
            #[arg(hide_long_help = true, id="path/to/second_log.txt")]
            input_file2: String,
            #[arg(hide_long_help = true, id="path/to/output_file_base")]
            output_file_base: String,
        },
    }

    let cli_args = CliArgs::try_parse_from(args)?;

    match cli_args.command {
        Commands::Hash {target_directory, output_path_base} => {
            command::run_hashdeep_command(
                target_directory.as_str(),
                output_path_base.as_str(),
                "hashdeep")?;
        },
        Commands::Sort {input_file, output_file} => {
            let warning_lines = sort::sort_log(
                input_file.as_str(),
                output_file.as_str()
            )?;
            print_hashdeep_log_warnings(input_file.as_str(), warning_lines, stderr)?;
        },
        Commands::Part {input_file1, input_file2, output_file_base} => {
            let partition_stats =
            partition::partition_log(
                input_file1.as_str(),
                input_file2.as_str(),
                output_file_base.as_str()
            )?;

            writeln!(stdout, "{}", partition_stats.stats_string)?;
            print_hashdeep_log_warnings(input_file1.as_str(), partition_stats.file1_warning_lines, stderr)?;
            print_hashdeep_log_warnings(input_file2.as_str(), partition_stats.file2_warning_lines, stderr)?;
        },
        Commands::Version => {
            writeln!(stdout, "hashdeep-compare version {VERSION}")?;
        }
    }

    Ok(())
}