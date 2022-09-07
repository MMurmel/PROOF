// Clippy lint target one. Enables all lints that are on by
// default (correctness, suspicious, style, complexity, perf) .
#![warn(clippy::all)]
// Clippy lint target two. Enables lints which are rather strict
// or have occasional false positives.
#![warn(clippy::pedantic)]
// Clippy lint target three. Enables new lints that are still
// under development
#![warn(clippy::nursery)]
// Clippy lint target four. Enable lints for the cargo manifest
// file, a.k.a. Cargo.toml.
#![warn(clippy::cargo)]
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![warn(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
// All other, generic lint targets that were not
// covered previously
#![warn(missing_debug_implementations)]

//! # _PROOF_
//!
//! My implementation for a research project on locals search
//! for learning partial boolean functions.

use std::fs::File;
use std::io::{
	BufRead,
	BufReader,
};

use log::{
	debug,
	info,
};
use proof::algorithms::local_search::basic_hill_climber;
use proof::algorithms::local_search::run_config::{RunConfig,};
use proof::arguments::Arguments;

/// ### `main`
fn main() {
	info!("Welcome to PROOF.");
	let arguments = Arguments::cli_args();

	env_logger::Builder::new()
		.filter_level(arguments.log_level())
		.init();

	let config: RunConfig<784> = arguments.config.as_deref().map_or_else(
		|| {
			debug!("Starting PROOF with default config file.");
			RunConfig::default()
		},
		|config_path| {
			debug!("Starting PROOF with custom config file.");
			let config_file = File::open(config_path).unwrap();
			let mut config_string = String::new();
			for line in BufReader::new(config_file).lines().flatten() {
				config_string.push_str(&line);
			}
			let config = serde_json::from_str(&config_string).unwrap();
			debug!("Parsed custom config file. Result was {:?}.", config);
			config
		},
	);

	basic_hill_climber(&config);
}
