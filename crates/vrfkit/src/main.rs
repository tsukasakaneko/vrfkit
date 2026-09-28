//! vrfkit -- CLI for VALORANT replay (.vrf) inspection, validation, and export.
//!
//! Subcommands:
//!   inspect <file.vrf>           -- print replay info, header, and chunk summary
//!   validate <file.vrf>          -- run the transform-validation oracle
//!   `diag <file.vrf> [--json <path>] [--include-payloads]` -- failure aggregate
//!   export `<file.vrf>` --out `<dir>` -- emit five Parquet tables + manifest.json
//!
//! `export` is behind the `export` feature (on by default). With it off the
//! binary still inspects, validates and runs diag -- all three drive the whole
//! decode pipeline -- and nothing links arrow, parquet or zstd.
//!
//! The pipeline itself lives in the library target (`src/lib.rs`) so other
//! programs can call it directly; this binary only forwards the command line.

#![forbid(unsafe_code)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match vrfkit::run_cli(&args) {
        // Not always SUCCESS: `validate` is an oracle and returns its own code.
        // See `oracle::Verdict`.
        Ok(code) => ExitCode::from(code),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
