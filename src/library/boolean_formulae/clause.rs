//! Provides representation and tools for clauses of disjunctive normal forms,
//! i.e. conjunctions of boolean literals.

use std::hash::Hash;
use bitmaps::{
	Bitmap,
	Bits,
	BitsImpl,
};
use log::{trace,};
use serde::{
	Serialize,
	Deserialize,
	Serializer,
	Deserializer,
};

use crate::boolean_formulae::data::{
	FeatureID,
	Sample,
};
use crate::boolean_formulae::evaluation::{Evaluate,};

/// A Helper for easier Serialization and Deserialization Access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Literal {
	/// The Feature this literal evaluates.
	id:     FeatureID,
	/// The Literal parity, i.e. `false` if the literal is negated
	/// and `true` if it is not negated.
	parity: bool,
}

/// A Wrapper for easier Serialization and Deserialization Access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ClauseWrapper {
	/// The Clauses Literals in order.
	literals: Vec<Literal>,
}

impl<const SIZE: usize> From<ClauseWrapper> for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn from(wrapper: ClauseWrapper) -> Self {
		assert_eq!(
			wrapper.literals.len(),
			SIZE,
			"Could not cast ClausWrapper to Clause due to difference in length."
		);
		let mut appearances: Bitmap<SIZE> = Bitmap::new();
		let mut polarities: Bitmap<SIZE> = Bitmap::new();

		for literal in wrapper.literals {
			appearances.set(literal.id, true);
			polarities.set(literal.id, literal.parity);
		}
		Self {
			appearances,
			polarities,
		}
	}
}

impl<const SIZE: usize> From<Clause<SIZE>> for ClauseWrapper
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn from(clause: Clause<SIZE>) -> Self {
		let mut literals = Vec::new();
		for index in &clause.appearances {
			literals.push(Literal {
				id:     index,
				parity: clause.polarities.get(index),
			});
		}
		Self { literals }
	}
}

/// The representation of a DNF clause, i.e. a conjunction of literals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clause<const SIZE: usize>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	/// A Bitmap representing, whether a literal is present in the clause.
	appearances: Bitmap<SIZE>,
	/// For the indices that are `true` in appearances, this specifies the literals
	/// parity. For all other indices this bitmaps content is meaningless.
	polarities:  Bitmap<SIZE>,
}

impl<const SIZE: usize> Default for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn default() -> Self {
		Self {
			appearances: Bitmap::new(),
			polarities:  Bitmap::new(),
		}
	}
}

impl<const SIZE: usize> Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	/// Whether the clause is empty.
	#[must_use]
	pub fn is_empty(&self) -> bool { self.appearances.is_empty() }

	/// Returns the length of the `Clause`, i.e. the number of literals it contains.
	#[must_use]
	pub fn length(&self) -> usize { self.appearances.len() }

	/// Returns all indices for which a literal is present in the clause.
	#[must_use]
	pub fn literal_indices(&self) -> Vec<FeatureID> { self.appearances.into_iter().collect() }

	/// Returns the parity of the literal with the specified `FeatureID`.
	/// If the literal is not present or the id is too big for this clause,
	/// `None` is returned.
	pub fn literal_at(&self, feature_id: FeatureID) -> Option<bool> {
		if feature_id >= SIZE || !self.appearances.get(feature_id) {
			None
		} else {
			Some(self.polarities.get(feature_id))
		}
	}

	/// Adds a literal to the clause, potentially replacing a previously contained literal
	/// with the same `FeatureID` and different polarity.
	/// Returns the replaced value.
	///
	/// # Panics
	/// Panics if `feature_id >= SIZE`.
	pub fn insert_literal(&mut self, feature_id: FeatureID, parity: bool) -> Option<bool> {
		assert!(
			feature_id < SIZE,
			"Index {} was out of bounce for clause of size {}!",
			feature_id,
			SIZE
		);
		trace!("Trying to insert {{x_{}: {}}} into clause.", feature_id, parity);
		if self.appearances.get(feature_id) {
			let result = Some(self.polarities.get(feature_id));
			self.polarities.set(feature_id, parity);
			result
		} else {
			self.appearances.set(feature_id, true);
			self.polarities.set(feature_id, parity);
			None
		}
	}

	/// Removes the literal with the given `FeatureID` from the `Clause`
	/// and returns whether it was present.
	///
	/// # Panics
	/// Panics if `feature_id >= SIZE`.
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

impl<const SIZE: usize> Evaluate<SIZE> for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
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

impl<const SIZE: usize> From<&Sample<SIZE>> for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn from(sample: &Sample<SIZE>) -> Self {
		Self {
			appearances: Bitmap::mask(SIZE),
			polarities:  sample.features(),
		}
	}
}
impl<const SIZE: usize> Serialize for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let wrapper = ClauseWrapper::from(self.clone());
		wrapper.serialize(serializer)
	}
}

impl<'de, const SIZE: usize> Deserialize<'de> for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let wrapper = ClauseWrapper::deserialize(deserializer)?;
		Ok(Clause::from(wrapper))
	}
}

unsafe impl<const SIZE: usize> Send for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
}
unsafe impl<const SIZE: usize> Sync for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
}
