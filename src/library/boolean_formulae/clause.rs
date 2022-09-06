//! Provides representation and tools for clauses of disjunctive normal forms,
//! i.e. conjunctions of boolean literals.

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Literal {
	id:     FeatureID,
	parity: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ClauseWrapper {
	literals: Vec<Literal>,
}

impl<const SIZE: usize> From<ClauseWrapper> for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
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
{
	fn from(clause: Clause<SIZE>) -> Self {
		let mut literals = Vec::new();
		for index in clause.appearances.into_iter() {
			literals.push(Literal {
				id:     index,
				parity: clause.polarities.get(index),
			})
		}
		Self { literals }
	}
}

/// The representation of a DNF clause, i.e. a conjunction of literals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<const SIZE: usize>
where
	BitsImpl<SIZE>: Bits,
{
	/// The conjunction of literals.
	appearances: Bitmap<SIZE>,
	polarities:  Bitmap<SIZE>,
}

impl<const SIZE: usize> Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
{
	/// Construct a `Clause` from a set of literals.
	#[must_use]
	pub fn new() -> Self {
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
	pub fn literals(&self) -> Vec<FeatureID> { self.appearances.into_iter().collect() }

	pub fn literal_at(&self, feature_id: FeatureID) -> bool {
		assert!(
			feature_id < SIZE,
			"Index {} was out of bounce for clause of size {}!",
			feature_id,
			SIZE
		);
		self.polarities.get(feature_id)
	}

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

impl<const SIZE: usize> Evaluate<SIZE> for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
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
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let wrapper = ClauseWrapper::deserialize(deserializer)?;
		Ok(Clause::from(wrapper))
	}
}
