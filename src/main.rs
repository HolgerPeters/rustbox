extern crate parquet;
extern crate s3;
extern crate serde_derive;
extern crate toml;

extern crate build_id;
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::*;

use anyhow::Error;

use std::path::PathBuf;
use structopt::StructOpt;

mod config;
mod core;
mod err;

/// Explode an ODBC data source at store the result in a Parquet file.
#[derive(StructOpt)]
struct Cli {
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long, parse(from_occurrences))]
    verbose: usize,
    #[structopt(parse(from_os_str))]
    config: PathBuf,
}

fn main() -> std::result::Result<(), Error> {
    let opt = Cli::from_args();

    let cfg = config::Config::from_path(opt.verbose, opt.config);
    let log = &cfg.logger();

    info!(
        log,
        "Mission defined";
        "Source" => &cfg.source,
        "Destination" => &cfg.destination
    );

    core::update(cfg).unwrap();

    info!(log, "Hello World!");

    Ok(())
}
