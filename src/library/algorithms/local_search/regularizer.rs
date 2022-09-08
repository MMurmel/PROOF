//! Provides different regularizers for `DNF`s.

use std::hash::Hash;
use bitmaps::{
	Bits,
	BitsImpl,
};
use serde::{
	Serialize,
	Deserialize,
};
use crate::algorithms::local_search::state::State;

/// Distinguishes different strategies to regularize a DNF.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Regularizer {
	/// Only penalize depth of the DNF.
	Depth,
	/// Only penalize length of the DNF.
	Length,
	/// Penalize the sum of depth and length of the DNF.
	DepthAndLength,
}

impl Regularizer {
	/// Return the regularization value for the DNF according to the chosen strategy.
	pub fn regularize<const SIZE: usize>(self, state: &State<SIZE>) -> u32
	where
		BitsImpl<SIZE>: Bits,
		<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
	{
		match self {
			Self::Depth => state.positive_dnf.depth() + state.negative_dnf.depth(),
			Self::Length => state.positive_dnf.length() + state.negative_dnf.length(),
			Self::DepthAndLength => {
				state.positive_dnf.length()
					+ state.positive_dnf.depth()
					+ state.negative_dnf.length()
					+ state.negative_dnf.depth()
			},
		}
	}
}
