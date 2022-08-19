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

use std::io::{
	BufRead,
	stdin,
};
use log::{info,};
use proof::arguments::Arguments;
use proof::boolean_formulae::data::{Sample,};

use proof::boolean_formulae::clause::Clause;
use proof::boolean_formulae::dnf::DNF;

use proof::algorithms::visualization::to_image::{ToImage,};

/// ### `main`
///
/// A simple, plain old `main` function. Nothing mysterious here.
fn main() {
	/// Width of images
	const WIDTH: u32 = 28;
	/// Height of images
	const HEIGHT: u32 = 28;

	let arguments = Arguments::cli_args();

	env_logger::Builder::new()
		.filter_level(arguments.log_level())
		.init();

	info!("Welcome to PROOF");

	let (positives, negatives): (Vec<_>, Vec<_>) = stdin()
		.lock()
		.lines()
		.map(|line| serde_json::from_str(&line.unwrap()).unwrap())
		.partition(Sample::label);

	let pos_dnf = DNF::new(positives.iter().map(Clause::from).collect());
	let neg_dnf = DNF::new(negatives.iter().map(Clause::from).collect());

	println!("({},{})", pos_dnf.length(), neg_dnf.length());

	pos_dnf
		.to_image(WIDTH, HEIGHT)
		.unwrap()
		.save("positives.png")
		.unwrap();
	neg_dnf
		.to_image(WIDTH, HEIGHT)
		.unwrap()
		.save("negatives.png")
		.unwrap();
}
