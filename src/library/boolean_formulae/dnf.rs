//! Provides representation for disjunctive normal form boolean formulae.

use serde::{
	Serialize,
	Deserialize,
};

use rayon::iter::{
	IntoParallelRefIterator,
	ParallelIterator,
};
use crate::boolean_formulae::clause::Clause;
use crate::boolean_formulae::data::{Sample,};
use crate::boolean_formulae::ErrorKind;
use crate::boolean_formulae::evaluation::{Evaluate,};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
/// The representation of a DNF, i.e. a disjunction of clauses.
pub struct DNF {
	/// The disjunction of clauses.
	clauses: Vec<Clause>,
}

impl DNF {
	/// Constructs a new `DNF` from a vector of clauses.
	#[must_use]
	pub fn new(clauses: Vec<Clause>) -> Self { Self { clauses } }

	/// Returns the length of the `DNF`, i.e. the sum of all its clauses' lengths.
	#[must_use]
	pub fn length(&self) -> u32 {
		self.clauses
			.iter()
			.map(|clause| u32::try_from(clause.length()).unwrap_or(u32::MAX))
			.sum()
	}

	/// Returns the depth of the `DNF`, i.e. the maximum of all its clauses' lengths.
	#[must_use]
	pub fn depth(&self) -> u32 {
		self.clauses
			.iter()
			.map(|clause| u32::try_from(clause.length()).unwrap_or(u32::MAX))
			.max()
			.unwrap_or_default()
	}

	/// Returns the clauses of the `DNF`.
	#[must_use]
	pub fn clauses(&self) -> &[Clause] { self.clauses.as_slice() }

	/// Returns a mutable reference to the clauses of the `DNF`.
	#[must_use]
	pub fn mut_clauses(&mut self) -> &mut [Clause] { self.clauses.as_mut_slice() }

	/// Removes the clause from the DNF and returns whether it was present.
	pub fn remove_clause(&mut self, clause: &Clause) -> bool {
		if let Some(position) = self.clauses.iter().position(|other| *other == *clause) {
			self.clauses.remove(position);
			return true;
		}
		false
	}
}

impl Evaluate for DNF {
	fn evaluate(&self, data: &Sample) -> Result<bool, ErrorKind> {
		let values: Result<Vec<bool>, ErrorKind> = self
			.clauses
			.par_iter()
			.map(|literal| literal.evaluate(data))
			.collect();
		Ok(values?.iter().any(|&x| x))
	}
}
