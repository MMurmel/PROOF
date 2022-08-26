//! Provides different regularizers for `DNF`s.

use crate::boolean_formulae::dnf::DNF;

use serde::{
	Serialize,
	Deserialize,
};
/// Distinguishes different ways to regularize a DNF.
#[derive(Serialize, Deserialize)]
pub enum Regularizer {
	/// Only penalize depth of the DNF.
	Depth,
	/// Only penalize length of the DNF.
	Length,
	/// Penalize the sum of depth and length of the DNF.
	DepthAndLength,
}

impl Regularizer {
	/// Return the regularization value for the DNF according to the `Strategy`.
	pub fn regularize(&self, dnf: &DNF) -> u32 {
		match self {
			Self::Depth => dnf.depth(),
			Self::Length => dnf.length(),
			Self::DepthAndLength => dnf.depth() + dnf.length(),
		}
	}
}
