//! Provides representation and tools for clauses of disjunctive normal forms,
//! i.e. conjunctions of boolean literals.

use std::collections::HashSet;
use serde::{
	Serialize,
	Deserialize,
};
use crate::boolean_formulae::data::{
	FeatureID,
	Sample,
};
use crate::boolean_formulae::ErrorKind;
use crate::boolean_formulae::evaluation::{Evaluate,};
use crate::boolean_formulae::literal::Literal;

/// The representation of a DNF clause, i.e. a conjunction of literals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clause {
	/// The conjunction of literals.
	literals: HashSet<Literal>,
}

impl Clause {
	/// Construct a `Clause` from a set of literals.
	#[must_use]
	pub const fn new(literals: HashSet<Literal>) -> Self { Self { literals } }

	/// Returns the length of the `Clause`, i.e. the number of literals it contains.
	#[must_use]
	pub fn length(&self) -> usize { self.literals.len() }

	/// Returns a reference to the literals of the `Clause`.
	#[must_use]
	pub const fn literals(&self) -> &HashSet<Literal> { &self.literals }

	/// Removes the literal with the given `FeatureID` from the `Clause`
	/// and returns whether it was present.
	pub fn remove_literal(&mut self, feature_id: FeatureID) -> bool {
		// Since literals are only hashed by their feature_id, the parity does not matter here.
		let lit = Literal::new(feature_id, true);
		self.literals.remove(&lit)
	}

	/// Adds a literal to the clause, potentially replacing a previously contained literal
	/// with the same `FeatureID`. Returns the replaced value.
	pub fn insert_literal(&mut self, literal: Literal) -> Option<Literal> { self.literals.replace(literal) }

	/// Whether the clause is empty, i.e. contains only `None` for every `FeatureID`
	#[must_use]
	pub fn is_empty(&self) -> bool { self.literals.is_empty() }
}

impl Evaluate for Clause {
	fn evaluate(&self, data: &Sample) -> Result<bool, ErrorKind> {
		// Try to evaluate every literal in the clause under the data.
		let values: Result<Vec<bool>, ErrorKind> = self.literals.iter().map(|x| x.evaluate(data)).collect();
		// If any literal evaluation resulted in an error, return that error;
		// otherwise return the conjunction of the evaluations.
		Ok(values?.iter().all(|&x| x))
	}
}

impl From<&Sample> for Clause {
	fn from(sample: &Sample) -> Self {
		let mut literals = HashSet::new();
		for (feature_id, feature_value) in sample.features().iter().enumerate() {
			#[allow(clippy::cast_possible_truncation)]
			literals.insert(Literal::new(feature_id as FeatureID, *feature_value));
		}

		Self { literals }
	}
}
