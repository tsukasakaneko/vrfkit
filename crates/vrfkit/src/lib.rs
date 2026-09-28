//! vrfkit as a library: the pipeline behind the `vrfkit` binary, callable from
//! another Rust program without spawning the CLI.
//!
//! The binary is a thin wrapper over [`run_cli`]. Only the entry points are
//! public; the modules stay private, so the library exposes exactly what the
//! CLI already exposes as subcommands and nothing that could drift from it.

#![forbid(unsafe_code)]

mod cli;
mod diagnose;
#[cfg(feature = "export")]
mod driver;
mod error;
mod inspect;
#[cfg(feature = "export")]
mod manifest;
mod oracle;
mod pass;
mod report;
mod sink;

pub use error::CliError;

/// Runs the command line `args` (`args[0]` is the program name) and returns
/// the process exit code, exactly as the `vrfkit` binary does.
///
/// Not always 0 on success: `validate` is an oracle and returns its own code.
pub fn run_cli(args: &[String]) -> Result<u8, CliError> {
    cli::run(args)
}

/// Exports one replay to Parquet, the same as
/// `vrfkit export <vrf_path> --out <out_dir> [--checkpoints]`.
///
/// Writes the same files as the CLI into `out_dir` (the main tables,
/// `manifest.json`, and the checkpoint tables when `with_checkpoints` is set).
/// The replay is only read. Progress and the run summary go to stderr, as in
/// the CLI.
#[cfg(feature = "export")]
pub fn export(vrf_path: &str, out_dir: &str, with_checkpoints: bool) -> Result<(), CliError> {
    driver::run(vrf_path, out_dir, with_checkpoints)
}
