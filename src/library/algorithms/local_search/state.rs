//! Any feasible solution to the local search problem is represented using this state.
use std::hash::Hash;
use bitmaps::{
	Bits,
	BitsImpl,
};
use log::{trace,};
use rayon::prelude::*;
use crate::boolean_formulae::data::Sample;
use crate::boolean_formulae::dnf::DNF;
use crate::boolean_formulae::evaluation::Evaluate;

/// The current state of the algorithm, i.e. the two DNFs.
#[derive(Debug, Clone)]
pub struct State<const SIZE: usize>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	/// Classifying DNF of the positive samples.
	pub(crate) positive_dnf: DNF<SIZE>,
	/// Classifying DNF of the negative samples.
	pub(crate) negative_dnf: DNF<SIZE>,
}

impl<const SIZE: usize> State<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	/// Whether the state is feasible under the data,
	/// i.e. the positive and negative DNF exactly classify the positive and negative
	/// samples, respectively.
	pub fn is_feasible(&self, positive_samples: &[Sample<SIZE>], negative_samples: &[Sample<SIZE>]) -> bool {
		trace!("Starting feasibility testing.");
		let positive_feasible = positive_samples
			.par_iter()
			.map(|x| self.positive_dnf.evaluate(x) && !self.negative_dnf.evaluate(x))
			.all(|x| x);
		let negative_feasible = negative_samples
			.par_iter()
			.map(|x| self.negative_dnf.evaluate(x) && !self.positive_dnf.evaluate(x))
			.all(|x| x);

		positive_feasible && negative_feasible
	}

	/// A reference to this state's `DNF`s together with a boolean indicating whether it
	/// is the positive `DNF`.
	pub fn dnfs(&self) -> Vec<(&DNF<SIZE>, bool)> {
		vec![(&self.positive_dnf, true), (&self.negative_dnf, false)]
	}

	/// Whether the state's positive `DNF` is equal to the provided one.
	pub fn positive_eq(&self, other: &DNF<SIZE>) -> bool { self.positive_dnf == *other }

	/// Whether the state's negative `DNF` is equal to the provided one.
	pub fn negative_eq(&self, other: &DNF<SIZE>) -> bool { self.negative_dnf == *other }
}
