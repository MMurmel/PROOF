//! Any feasible solution to the local search problem is represented using this state.
use bitmaps::{
	Bits,
	BitsImpl,
};
use log::{trace,};
use crate::boolean_formulae::data::Sample;
use crate::boolean_formulae::dnf::DNF;
use crate::boolean_formulae::evaluation::Evaluate;

/// The current state of the algorithm, i.e. the two DNFs.
#[derive(Debug, Clone)]
pub struct State<const SIZE: usize>
where
	BitsImpl<SIZE>: Bits,
{
	/// Classifying DNF of the positive samples.
	pub(crate) positive_dnf: DNF<SIZE>,
	/// Classifying DNF of the negative samples.
	pub(crate) negative_dnf: DNF<SIZE>,
}

impl<const SIZE: usize> State<SIZE>
where
	BitsImpl<SIZE>: Bits,
{
	/// Whether the state is feasible under the data,
	/// i.e. the positive and negative DNF exactly classify the positive and negative
	/// samples, respectively.
	pub fn is_feasible(
		&self,
		positive_samples: &Vec<Sample<SIZE>>,
		negative_samples: &Vec<Sample<SIZE>>,
	) -> bool {
		trace!("Starting feasibility testing.");
		let positive_feasible = positive_samples
			.iter()
			.map(|x| self.positive_dnf.evaluate(x))
			.all(|x| x);
		let negative_feasible = negative_samples
			.iter()
			.map(|x| self.negative_dnf.evaluate(x))
			.all(|x| x);

		positive_feasible && negative_feasible
	}
}
