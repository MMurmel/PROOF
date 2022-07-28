// SPDX-License-Identifier: GPL-3.0-or-later

// see https://github.com/clap-rs/clap/blob/v3.1.2/examples/derive_ref/README.md
// for the clap derive reference
use clap::Parser;

#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Parser, Debug)]
#[clap(author, version, about, name = "proof")]
pub struct Arguments {
	#[clap(flatten)]
	verbosity: clap_verbosity_flag::Verbosity,
}

impl Arguments {
	/// Returns the current command line arguments.
	#[must_use]
	pub fn cli_args() -> Self { Self::parse() }

	/// Returns the log level specified in the command line arguments.
	///
	/// Logging is off by default, adding -v increases verbosity with each occurrence.
	#[must_use]
	pub fn log_level(&self) -> log::LevelFilter { self.verbosity.log_level_filter() }
}
