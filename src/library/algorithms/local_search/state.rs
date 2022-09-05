//! Any feasible solution to the local search problem is represented using this state.
use log::{trace,};
use rayon::iter::{
	IntoParallelRefIterator,
	ParallelIterator,
};
use crate::boolean_formulae::data::Sample;
use crate::boolean_formulae::dnf::DNF;
use crate::boolean_formulae::evaluation::Evaluate;

/// The current state of the algorithm, i.e. the two DNFs.
#[derive(Debug, Clone)]
pub struct State<const SIZE: usize> {
	/// Classifying DNF of the positive samples.
	pub(crate) positive_dnf: DNF<SIZE>,
	/// Classifying DNF of the negative samples.
	pub(crate) negative_dnf: DNF<SIZE>,
}

impl<const SIZE: usize> State<SIZE> {
	/// Whether the state is feasible under the data,
	/// i.e. the positive and negative DNF exactly classify the positive and negative
	/// samples, respectively.
	pub fn is_feasible(&self, positive_samples: &[Sample<SIZE>], negative_samples: &[Sample<SIZE>]) -> bool {
		trace!("Starting feasibility testing.");
		let positive_feasible = positive_samples
			.par_iter()
			.map(|x| self.positive_dnf.evaluate(x))
			.all(|evaluation| {
				// In order to be a feasible solution, all positive data must evaluate to true under the
				// positive_dnf.
				if let Ok(value) = evaluation {
					value
				} else {
					false
				}
			});
		let negative_feasible = negative_samples
			.par_iter()
			.map(|x| self.negative_dnf.evaluate(x))
			.all(|evaluation| {
				// In order to be a feasible solution, all positive data must evaluate to true under the
				// positive_dnf.
				if let Ok(value) = evaluation {
					value
				} else {
					false
				}
			});

		positive_feasible && negative_feasible
	}
}
