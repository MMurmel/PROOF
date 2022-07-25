// SPDX-License-Identifier: GPL-3.0-or-later

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

//! # _yourprojectname_
//!
//! TODO

use log::{info,};
use yourprojectname::arguments::Arguments;

/// ### `main`
///
/// A simple, plain old `main` function. Nothing mysterious here.
fn main() {
	let arguments = Arguments::cli_args();

	env_logger::Builder::new()
		.filter_level(arguments.log_level())
		.init();

	info!("Welcome to yourprojectname");
}
