//! Provides representation and tools for clauses of disjunctive normal forms,
//! i.e. conjunctions of boolean literals.

use serde::{
	Serialize,
	Deserialize,
};
use crate::boolean_formulae::data::{
	AtomID,
	Sample,
};
use crate::boolean_formulae::evaluation::{
	Evaluate,
	ErrorKind,
};
use crate::boolean_formulae::literal::Literal;

/// The representation of a DNF clause, i.e. a conjunction of literals.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Clause {
	/// The conjunction of literals.
	literals: Vec<Literal>,
}

impl Clause {
	/// Construct a `Clause` from a list of literals.
	#[must_use]
	pub fn new(literals: Vec<Literal>) -> Self { Self { literals } }

	/// Returns the length of the `Clause`.
	#[must_use]
	pub fn length(&self) -> usize { self.literals.len() }

	/// Returns the literals of the `Clause`.
	#[must_use]
	pub fn literals(&self) -> &[Literal] { self.literals.as_slice() }

	/// Returns the maximal `AtomID` for which a literal is present in the `Clause`.
	#[must_use]
	pub fn maximal_id(&self) -> AtomID { self.literals.iter().map(Literal::atom_id).max().unwrap_or(0) }

	/// Removes the literal with the given `AtomID` from the `Clause`.
	pub fn remove_literal(&mut self, atom_id: AtomID) {
		match self.literals.iter().position(|x| x.atom_id() == atom_id) {
			Some(index) => {
				self.literals.remove(index);
			},
			None => {},
		}
	}
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

impl From<&Sample> for Clause {
	fn from(sample: &Sample) -> Self {
		let mut literals = Vec::new();
		for (feature_id, feature_value) in sample.features().iter().enumerate() {
			literals.push(Literal::new(feature_id as AtomID, *feature_value));
		}

		Self { literals }
	}
}
