//! Representation and tools for clauses of disjunctive normal forms,
//! i.e. conjunctions of boolean literals.

use crate::boolean_formulae::data::Sample;
use crate::boolean_formulae::evaluation::{
	Evaluate,
	ErrorKind,
};
use crate::boolean_formulae::literal::Literal;

/// The representation of a DNF clause, i.e. a conjunction of literals.
#[derive(Debug, Clone)]
pub struct Clause {
	/// The conjunction of literals.
	literals: Vec<Literal>,
}

impl Clause {
	/// Construct a `Clause` from a list of literals.
	#[must_use]
	pub fn new(literals: Vec<Literal>) -> Self { Self { literals } }

	/// Returns the length of the clause.
	#[must_use]
	pub fn length(&self) -> usize { self.literals.len() }
}

impl Evaluate for Clause {
	fn evaluate(&self, data: &Sample) -> Result<bool, ErrorKind> {
		let values: Result<Vec<bool>, ErrorKind> = self
			.literals
			.iter()
			.map(|literal| literal.evaluate(data))
			.collect();
		Ok(values?.iter().all(|&x| x))
	}
}
