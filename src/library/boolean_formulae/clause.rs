//! Provides representation and tools for clauses of disjunctive normal forms,
//! i.e. conjunctions of boolean literals.

use log::trace;
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Clause {
	/// The conjunction of literals.
	literals: Vec<Option<Literal>>,
}

impl Clause {
	/// Construct a `Clause` from a list of literals.
	#[must_use]
	pub fn new(literals: Vec<Option<Literal>>) -> Self { Self { literals } }

	/// Returns the length of the `Clause`.
	#[must_use]
	pub fn length(&self) -> usize { self.literals.len() }

	/// Returns the literals of the `Clause`.
	#[must_use]
	pub fn literals(&self) -> &[Option<Literal>] { self.literals.as_slice() }

	/// Removes the literal with the given `FeatureID` from the `Clause`.
	///
	/// If the literal with `FeatureID` was present before, it is returned.
	/// If the literal was not present in the first place, `Ok(None)` is returned.
	///
	/// # Errors
	/// If the `FeatureID` is larger than the length of the clause,
	/// `Err(ErrorKind::FeatureIDOutOfScope(feature_id))` is returned.
	pub fn remove_literal(&mut self, feature_id: FeatureID) -> Result<Option<Literal>, ErrorKind> {
		trace!(
			"Trying to remove literal with FeatureID {} from a clause.",
			feature_id
		);
		match self.literals.get_mut(feature_id as usize) {
			Some(lit) => match lit {
				Some(value) => {
					trace!(
						"Clause contained a literal with FeatureID {} and polarity <{}>, removed \
						 successfully.",
						feature_id,
						value.parity()
					);
					let helper = *value;
					*lit = None;
					Ok(Some(helper))
				},
				None => {
					trace!(
						"Clause did not contain a literal with FeatureID {}, but was in viable range.",
						feature_id
					);
					Ok(None)
				},
			},
			None => {
				trace!(
					"FeatureID {} was out of range for clause, returning an Error.",
					feature_id
				);
				Err(ErrorKind::FeatureIdNotPresent(feature_id))
			},
		}
	}

	/// Whether the clause is empty, i.e. contains only `None` for every `FeatureID`
	#[must_use]
	pub fn is_empty(&self) -> bool { self.literals.iter().all(Option::is_none) }
}

impl Evaluate for Clause {
	fn evaluate(&self, data: &Sample) -> Result<bool, ErrorKind> {
		let values: Result<Vec<bool>, ErrorKind> = self
			.literals
			.iter()
			// only consider `Some(lit)` values and discard `None`s
			.filter_map(|&x| x)
			.map(|x| x.evaluate(data))
			.collect();
		Ok(values?.iter().all(|&x| x))
	}
}

impl From<&Sample> for Clause {
	fn from(sample: &Sample) -> Self {
		let mut literals = Vec::new();
		for (feature_id, feature_value) in sample.features().iter().enumerate() {
			#[allow(clippy::cast_possible_truncation)]
			literals.push(Some(Literal::new(feature_id as FeatureID, *feature_value)));
		}

		Self { literals }
	}
}
