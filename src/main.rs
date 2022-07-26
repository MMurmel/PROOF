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
use std::time::{Instant,};

use log::{
	debug,
	info,
};
use proof::algorithms::local_search::local_search;
use proof::algorithms::local_search::run_config::{RunConfig,};
use proof::arguments::Arguments;

/// ### `main`
fn main() {
	let arguments = Arguments::cli_args();

	env_logger::Builder::new()
		.filter_level(arguments.log_level())
		.init();

	let config: RunConfig<784> = arguments.config.as_deref().map_or_else(
		|| {
			debug!("No custom config file provided. Starting PROOF with default config file.");
			RunConfig::default()
		},
		|config_path| {
			debug!("Starting PROOF with custom config file.");
			let config_file = File::open(config_path).unwrap();
			let mut config_string = String::new();
			for line in BufReader::new(config_file).lines().flatten() {
				config_string.push_str(&line);
			}
			let parsed_config = serde_json::from_str(&config_string);
			if let Ok(correct_config) = parsed_config {
				debug!("Parsed custom config file successfully.");
				correct_config
			} else {
				let default = RunConfig::default();
				debug!(
					"Custom config file contained an error, starting with default config instead: {}",
					serde_json::to_string(&default).unwrap()
				);
				default
			}
		},
	);

	if arguments.syntax_check {
		return;
	}

	let start_time = Instant::now();

	info!("Starting execution of local search algorithm.");
	local_search(&config);
	info!("Program execution took {:?}", start_time.elapsed());
}
