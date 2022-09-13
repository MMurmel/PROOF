//! Provides neighbourhood generation methods for run state.

use std::hash::Hash;
use bitmaps::{
	Bitmap,
	Bits,
	BitsImpl,
};
use log::{trace,};
use rand::prelude::{SliceRandom,};
use rand::thread_rng;

use serde::{
	Serialize,
	Deserialize,
};
use crate::algorithms::local_search::state::State;
use crate::boolean_formulae::clause::Clause;
use crate::boolean_formulae::data::FeatureID;
use crate::boolean_formulae::dnf::DNF;

/// Distinguishes different methods for generating Neighbourhoods of a `DNF`.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum NeighbourhoodGenerator {
	/// Remove one literal.
	RemoveOneLiteral {
		/// Truncates the generated neighbourhood to the provided size.
		neighbourhood_limit: Option<usize>,
		/// Shuffles the generated neighbourhood,
		/// introduces randomness,
		/// you most likely want this to be `True`.
		shuffle:             bool,
	},
	/// Removes literals of one `FeatureID` from all clauses.
	RemoveFromAllClauses {
		/// If this is set to true, only those literals will be removed, that have the
		/// same polarity in all clauses of the dnf.
		only_same_polarities: bool,
	},
	/// Insert a literal into one clause and remove it from all others.
	InsertOneRemoveElsewhere,
	/// Inserts one literal into a clause.
	InsertOneLiteral {
		/// Truncates the generated neighbourhood to the provided size.
		neighbourhood_limit: Option<usize>,
		/// Shuffles the generated neighbourhood,
		/// introduces randomness,
		/// you most likely want this to be `True`.
		shuffle:             bool,
	},
}

impl NeighbourhoodGenerator {
	/// Helper for creating the resulting state after cloning and modifying a dnf from an
	/// original state. Useful because in neighbourhood generation it does seldom matter
	/// which dnf was modified. By using the state.dnfs() we can just iterate over them
	/// and distinguish the creation of the new state here.
	fn create_modified_state<const SIZE: usize>(
		original_state: &State<SIZE>,
		modified_dnf: DNF<SIZE>,
		which_dnf: bool,
	) -> State<SIZE>
	where
		BitsImpl<SIZE>: Bits,
		<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
	{
		if which_dnf {
			State {
				positive_dnf: modified_dnf,
				negative_dnf: original_state.negative_dnf.clone(),
			}
		} else {
			State {
				positive_dnf: original_state.positive_dnf.clone(),
				negative_dnf: modified_dnf,
			}
		}
	}

	/// Generates the neighbourhood of the `DNF` according to the generator strategy.
	#[allow(clippy::too_many_lines)]
	pub fn generate_neighbourhood<const SIZE: usize>(&self, state: &State<SIZE>) -> Vec<State<SIZE>>
	where
		BitsImpl<SIZE>: Bits,
		<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
	{
		let mut result: Vec<State<SIZE>> = Vec::new();
		match self {
			Self::RemoveOneLiteral {
				neighbourhood_limit,
				shuffle,
			} => {
				let mut combinations: Vec<((&DNF<SIZE>, bool), Clause<SIZE>, FeatureID)> = state
					.dnfs()
					.iter()
					.flat_map(|(dnf, which_dnf)| {
						dnf.clauses().iter().flat_map(|clause| {
							clause
								.literal_indices()
								.iter()
								.map(|index| ((*dnf, *which_dnf), *clause, *index))
								.collect::<Vec<((&DNF<SIZE>, bool), Clause<SIZE>, FeatureID)>>()
						})
					})
					.collect();

				if *shuffle {
					combinations.shuffle(&mut thread_rng());
				}

				if let Some(limit) = neighbourhood_limit {
					combinations = combinations
						.choose_multiple(&mut thread_rng(), *limit)
						.cloned()
						.collect::<Vec<((&DNF<SIZE>, bool), Clause<SIZE>, FeatureID)>>();
				}

				for ((dnf, which_dnf), mut clause, present_id) in combinations {
					// Remove the original clause from the DNF …
					let mut cloned_dnf = dnf.clone();
					cloned_dnf.remove_clause(&clause);
					// … modify it …
					clause.remove_literal(present_id);
					// … and re-inject it.
					cloned_dnf.insert_clause(clause);

					// store the result.
					let modified_state = Self::create_modified_state(state, cloned_dnf, which_dnf);

					result.push(modified_state);
				}
				trace!("Found {} neighbours by RemoveOneLiteral.", result.len());
			},
			Self::InsertOneLiteral {
				neighbourhood_limit,
				shuffle,
			} => {
				// TODO for completeness of search space
			},
			Self::RemoveFromAllClauses { only_same_polarities } => {
				for (dnf, which_dnf) in state.dnfs() {
					let (mut indices_present_in_all, positive_in_all, negative_in_all_inverted) =
						dnf.clauses().iter().fold(
							(Bitmap::mask(SIZE), Bitmap::mask(SIZE), Bitmap::new()),
							|(acc, positives_same, negatives_same), curr_clause| {
								(
									acc & *curr_clause.appearances(),
									positives_same & *curr_clause.polarities(),
									negatives_same | *curr_clause.polarities(),
								)
							},
						);

					if *only_same_polarities {
						indices_present_in_all &= positive_in_all | !negative_in_all_inverted;
					}

					for index in &indices_present_in_all {
						let cloned_dnf = DNF::new(
							dnf.clauses()
								.iter()
								.copied()
								.map(|mut clause| {
									clause.remove_literal(index);
									clause
								})
								.collect(),
						);
						let modified_state = Self::create_modified_state(state, cloned_dnf, which_dnf);
						result.push(modified_state);
						result.shuffle(&mut thread_rng());
					}
				}
				trace!("Found {} neighbours by RemoveFromAllClauses.", result.len());
			},
			NeighbourhoodGenerator::InsertOneRemoveElsewhere => {},
		}
		result
	}
}
