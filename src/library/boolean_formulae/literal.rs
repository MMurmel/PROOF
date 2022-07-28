//! Provides representation and tools for boolean literals.
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

/// A representation for logical literals,
/// i.e. an atom or its negation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Literal {
	/// The variable from which the literal is created.
	atom_id: AtomID,
	/// The literals parity, i.e.
	/// `true` if it is an atom and
	/// `false` if it is the negation of an atom.
	parity:  bool,
}

impl Literal {
	/// Returns a new `Literal` of the provided atom with the given parity.
	#[must_use]
	pub const fn new(atom_id: AtomID, parity: bool) -> Self { Self { atom_id, parity } }

	/// Returns the `Literal` atom id.
	#[must_use]
	pub const fn atom_id(&self) -> AtomID { self.atom_id }

	/// Returns the `Literal`s parity.
	#[must_use]
	pub const fn parity(&self) -> bool { self.parity }

	/// Returns a negated variation of the `Literal`.
	#[must_use]
	pub const fn to_negated(&self) -> Self {
		Self {
			atom_id: self.atom_id,
			parity:  !self.parity,
		}
	}

	/// Negates the `Literal`, i.e. flips its parity.
	pub fn negate(&mut self) { self.parity = !self.parity; }
}

impl Evaluate for Literal {
	/// Evaluates the literal on a variable assignment.
	fn evaluate(&self, data: &Sample) -> Result<bool, ErrorKind> {
		match data.at_feature(self.atom_id) {
			None => Err(ErrorKind::InsufficientData(self.atom_id)),
			// XOR is a toggled inverter
			// self.parity	| 0 0 1 1
			// !self.parity	| 1 1 0 0
			// assignment	| 0 1 0 1
			// outcome		| 1 0 0 1
			Some(assignment) => Ok(!self.parity ^ assignment),
		}
	}
}
