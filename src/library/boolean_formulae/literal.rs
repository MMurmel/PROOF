//! Provides representation and tools for boolean literals.
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

/// A representation for logical literals,
/// i.e. an atomic variable (here called 'feature') or its negation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Literal {
	/// The variable from which the literal is created.
	feature_id: FeatureID,
	/// The literals parity, i.e.
	/// `true` if it is an atom and
	/// `false` if it is the negation of an atom.
	parity:     bool,
}

impl Literal {
	/// Returns a new `Literal` of the provided feature with the given parity.
	#[must_use]
	pub const fn new(feature_id: FeatureID, parity: bool) -> Self { Self { feature_id, parity } }

	/// Returns the `Literal`s feature id.
	#[must_use]
	pub const fn feature_id(&self) -> FeatureID { self.feature_id }

	/// Returns the `Literal`s parity.
	#[must_use]
	pub const fn parity(&self) -> bool { self.parity }

	/// Returns a negated variation of the `Literal`.
	#[must_use]
	pub const fn to_negated(&self) -> Self {
		Self {
			feature_id: self.feature_id,
			parity:     !self.parity,
		}
	}

	/// Negates the `Literal`, i.e. flips its parity.
	pub fn negate(&mut self) { self.parity = !self.parity; }
}

impl Evaluate for Literal {
	/// Evaluates the literal on a variable assignment (i.e. on data).
	fn evaluate(&self, data: &Sample) -> Result<bool, ErrorKind> {
		match data.at_feature(self.feature_id) {
			None => Err(ErrorKind::InsufficientData(self.feature_id)),
			// XOR is a toggled inverter
			// self.parity			| 0 0 1 1
			// !self.parity			| 1 1 0 0
			// feature assignment	| 0 1 0 1
			// ==============================
			// outcome				| 1 0 0 1
			Some(feature_assignment) => Ok(!self.parity ^ feature_assignment),
		}
	}
}
