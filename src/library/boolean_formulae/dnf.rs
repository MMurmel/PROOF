//! Provides representation for disjunctive normal form boolean formulae.

use std::collections::{HashSet,};
use std::hash::Hash;
use bitmaps::{
	Bits,
	BitsImpl,
};
use rayon::prelude::*;
use serde::{
	Serialize,
	Deserialize,
};

use crate::boolean_formulae::clause::Clause;
use crate::boolean_formulae::data::{Sample,};
use crate::boolean_formulae::evaluation::{Evaluate,};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
/// The representation of a DNF, i.e. a disjunction of clauses.
pub struct DNF<const SIZE: usize>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	/// The disjunction of clauses.
	clauses: HashSet<Clause<SIZE>>,
}

impl<const SIZE: usize> DNF<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	/// Constructs a new `DNF` from a vector of clauses.
	#[must_use]
	pub fn new(clauses: HashSet<Clause<SIZE>>) -> Self { Self { clauses } }

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

	/// Returns a reference to the clauses of the `DNF`.
	#[must_use]
	pub const fn clauses(&self) -> &HashSet<Clause<SIZE>> { &self.clauses }

	/// Returns a mutable reference to the clauses of the `DNF`.
	#[must_use]
	pub fn mut_clauses(&mut self) -> &mut HashSet<Clause<SIZE>> { &mut self.clauses }

	/// Removes the clause from the `DNF` and returns whether it was present.
	pub fn remove_clause(&mut self, clause: &Clause<SIZE>) -> bool { self.clauses.remove(clause) }

	/// Inserts a clause into the `DNF` and returns whether it was newly inserted.
	pub fn insert_clause(&mut self, clause: Clause<SIZE>) -> bool { self.clauses.insert(clause) }
}

impl<const SIZE: usize> Evaluate<SIZE> for DNF<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn evaluate(&self, data: &Sample<SIZE>) -> bool {
		self.clauses
			.par_iter()
			.map(|literal| literal.evaluate(data))
			.any(|x| x)
	}
}
