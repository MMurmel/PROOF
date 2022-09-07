use bitmaps::{
	Bits,
	BitsImpl,
};
use rayon::prelude::*;
use serde::{
	Serialize,
	Deserialize,
};
use crate::algorithms::local_search::neighbourhood_generator::NeighbourhoodGenerator;
use crate::algorithms::local_search::regularizer::Regularizer;
use crate::algorithms::local_search::state::State;
use crate::boolean_formulae::data::Sample;

/// Differentiates local search algorithms.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Algorithm {
	/// Most basic hill climbing algorithm.
	BasicHillClimber {
		/// Abort algorithm after a maximum number of iterations.
		max_iterations: u32,
	},
	/// Stochastic
	StochasticHillClimber {
		/// Abort algorithm after a maximum number of iterations.
		max_iterations: u32,
	},
}

/// Applies the specified `Algorithm` to a state, w.r.t. the samples, neighbourhood
/// generators and the regularizer.
pub struct AlgorithmRunner<const SIZE: usize>
where
	BitsImpl<SIZE>: Bits,
{
	/// The algorithm to use.
	algorithm:                Algorithm,
	/// The current state of the two-DNF-state.
	current_state:            State<SIZE>,
	/// All samples for which the positive DNF must be exact.
	positive_samples:         Vec<Sample<SIZE>>,
	/// All samples for which the negative DNF must be exact.
	negative_samples:         Vec<Sample<SIZE>>,
	/// By which strategy (or strategies) to generate new neighbours.
	neighbourhood_generators: Vec<NeighbourhoodGenerator>,
	/// By which strategy to judge feasible solutions.
	regularizer:              Regularizer,
	/// How many iterations of the algorithm have already elapsed.
	iterations:               u32,
}

impl<const SIZE: usize> AlgorithmRunner<SIZE>
where
	BitsImpl<SIZE>: Bits,
{
	/// Creates a new algorithms runner.
	pub fn new(
		algorithm: Algorithm,
		initial_state: State<SIZE>,
		positive_samples: Vec<Sample<SIZE>>,
		negative_samples: Vec<Sample<SIZE>>,
		neighbourhood_generators: Vec<NeighbourhoodGenerator>,
		regularizer: Regularizer,
	) -> Self {
		Self {
			algorithm,
			current_state: initial_state,
			positive_samples,
			negative_samples,
			neighbourhood_generators,
			regularizer,
			iterations: 0,
		}
	}

	/// Performs one step of the algorithm and returns its state afterwards.
	/// Returns None when the algorithm has terminated.
	pub fn step(&mut self) -> Option<State<SIZE>> {
		let neighbourhood = self
			.neighbourhood_generators
			.par_iter()
			.flat_map(|generator| generator.generate_neighbourhood(&self.current_state));

		match self.algorithm {
			Algorithm::BasicHillClimber { max_iterations } => {
				if self.iterations > max_iterations {
					return None;
				}

				let best_neighbour = neighbourhood
					.filter(|state| state.is_feasible(&self.positive_samples, &self.negative_samples))
					.min_by(|a, b| {
						self.regularizer
							.regularize(a)
							.cmp(&self.regularizer.regularize(b))
					})?;
				if self.regularizer.regularize(&best_neighbour)
					< self.regularizer.regularize(&self.current_state)
				{
					self.current_state = best_neighbour;
				} else {
					return None;
				}
			},
			Algorithm::StochasticHillClimber { .. } => {},
		}
		self.iterations += 1;
		Some(self.current_state.clone())
	}

	/// Returns the current iteration count of the algorithm.
	pub const fn iteration(&self) -> u32 { self.iterations }
}
