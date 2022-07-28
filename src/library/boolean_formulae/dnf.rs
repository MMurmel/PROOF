//! Provides representation for disjunctive normal form boolean formulae.

use crate::boolean_formulae::clause::Clause;
use crate::boolean_formulae::data::Sample;
use crate::boolean_formulae::evaluation::{
	ErrorKind,
	Evaluate,
};

#[derive(Debug, Clone, Eq, PartialEq)]
/// The representation of a DNF, i.e. a disjunction of clauses.
pub struct DNF {
	/// The disjunction of clauses.
	clauses: Vec<Clause>,
}

impl DNF {
	/// Constructs a new `DNF` from a vector of clauses.
	#[must_use]
	pub fn new(clauses: Vec<Clause>) -> Self { Self { clauses } }

	/// Returns the length of the `DNF`.
	#[must_use]
	pub fn length(&self) -> usize { self.clauses.len() }
}

impl Evaluate for DNF {
	fn evaluate(&self, data: &Sample) -> Result<bool, ErrorKind> {
		let values: Result<Vec<bool>, ErrorKind> = self
			.clauses
			.iter()
			.map(|literal| literal.evaluate(data))
			.collect();
		Ok(values?.iter().any(|&x| x))
	}
}
