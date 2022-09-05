//! Provides representation and tools for clauses of disjunctive normal forms,
//! i.e. conjunctions of boolean literals.

use bitmaps::Bitmap;
use log::{trace,};
use serde::{
	Serialize,
	Deserialize,
};
use crate::boolean_formulae::data::{
	FeatureID,
	Sample,
};
use crate::boolean_formulae::evaluation::{Evaluate,};

/// The representation of a DNF clause, i.e. a conjunction of literals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<const SIZE: usize> {
	/// The conjunction of literals.
	appearances: Bitmap<SIZE>,
	polarities:  Bitmap<SIZE>,
}

impl<const SIZE: usize> Clause<SIZE> {
	/// Construct a `Clause` from a set of literals.
	#[must_use]
	pub const fn new() -> Self {
		Self {
			appearances: Bitmap::new(),
			polarities:  Bitmap::new(),
		}
	}

	/// Whether the clause is empty, i.e. contains only `None` for every `FeatureID`
	#[must_use]
	pub fn is_empty(&self) -> bool { self.appearances.is_empty() }

	/// Returns the length of the `Clause`, i.e. the number of literals it contains.
	#[must_use]
	pub fn length(&self) -> usize { self.appearances.len() }

	/// Returns a reference to the literals of the `Clause`.
	#[must_use]
	pub const fn literals(&self) -> Vec<FeatureID> { self.appearances.into_iter().collect() }

	/// Adds a literal to the clause, potentially replacing a previously contained literal
	/// with the same `FeatureID` and different polarity.
	/// Returns the replaced value.
	pub fn insert_literal(&mut self, feature_id: FeatureID, parity: bool) -> Option<bool> {
		assert!(
			feature_id < SIZE,
			"Index {} was out of bounce for clause of size {}!",
			feature_id,
			SIZE
		);
		trace!("Trying to insert {{x_{}: {}}} into clause.", feature_id, parity);
		return if self.appearances.get(feature_id) {
			let result = Some(self.polarities.get(feature_id));
			self.polarities.set(feature_id, parity);
			result
		} else {
			self.appearances.set(feature_id, true);
			self.polarities.set(feature_id, parity);
			None
		};
	}

	/// Removes the literal with the given `FeatureID` from the `Clause`
	/// and returns whether it was present.
	pub fn remove_literal(&mut self, feature_id: FeatureID) -> bool {
		assert!(
			feature_id < SIZE,
			"Index {} was out of bounce for clause of size {}!",
			feature_id,
			SIZE
		);
		self.appearances.set(feature_id, false)
	}
}

impl<const SIZE: usize> Evaluate<SIZE> for Clause<SIZE> {
	fn evaluate(&self, data: &Sample<SIZE>) -> bool {
		// XOR is a toggled inverter
		// polarity of atom			| 0 0 1 1
		// !polarity of atom		| 1 1 0 0
		// feature assignment		| 0 1 0 1
		// ===================================
		// !p.o.a XOR f.a.			| 1 0 0 1
		let all_evaluated = data.features() ^ !self.polarities.clone();

		self.appearances
			.into_iter()
			.map(|index| all_evaluated.get(index))
			.all(|x| x)
	}
}

impl<const SIZE: usize> From<&Sample<SIZE>> for Clause<SIZE> {
	fn from(sample: &Sample<SIZE>) -> Self {
		Self {
			appearances: Bitmap::mask(SIZE),
			polarities:  sample.features(),
		}
	}
}
