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

use log::{info,};
use proof::arguments::Arguments;
use proof::boolean_formulae::data::AtomID;
use proof::boolean_formulae::literal::Literal;

use rand::Rng;
use proof::boolean_formulae::clause::Clause;
use proof::boolean_formulae::dnf::DNF;

use proof::algorithms::visualization::to_image::{ToImage,};

/// ### `main`
///
/// A simple, plain old `main` function. Nothing mysterious here.
fn main() {
	const WIDTH: u32 = 20;
	const HEIGHT: u32 = 20;

	let arguments = Arguments::cli_args();

	env_logger::Builder::new()
		.filter_level(arguments.log_level())
		.init();

	info!("Welcome to PROOF");
	let mut rng = rand::thread_rng();
	let mut clauses: Vec<Clause> = Vec::new();
	for _ in 0..10 {
		let literals: Vec<Literal> = (0..399)
			.map(|x| Literal::new(AtomID::try_from(x).unwrap(), rng.gen_bool(0.5)))
			.collect();
		clauses.push(Clause::new(literals));
	}

	for clause in &mut clauses.iter_mut().skip(5) {
		clause.remove_literal(255);
	}

	let dnf = DNF::new(clauses);

	println!("{}", serde_json::to_string(&dnf).unwrap());

	for (index, clause) in dnf.clauses().iter().enumerate() {
		clause
			.to_image(WIDTH, HEIGHT)
			.unwrap()
			.save(format!("clause{}.png", index))
			.unwrap();
	}

	dnf.to_image(WIDTH, HEIGHT).unwrap().save("average.png").unwrap();
}
