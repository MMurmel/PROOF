//! Provides neighbourhood generation methods for run state.

use std::hash::Hash;
use bitmaps::{
	Bitmap,
	Bits,
	BitsImpl,
};
use log::{debug,};
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
		neighbourhood_limit: Option<usize>,
		shuffle:             bool,
	},
	RemoveFromAllClauses,
	InsertOneLiteral {
		neighbourhood_limit: Option<usize>,
		shuffle:             bool,
	},
	/// Remove all empty clauses.
	RemoveEmptyClauses,
}

impl NeighbourhoodGenerator {
	/// Generates the neighbourhood of the `DNF` according to the generator strategy.
	#[allow(clippy::too_many_lines)]
	pub fn generate_neighbourhood<const SIZE: usize>(&self, state: &State<SIZE>) -> Vec<State<SIZE>>
	where
		BitsImpl<SIZE>: Bits,
		<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
	{
		debug!("Started generating neighbourhood.");
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
					let modified_state = if which_dnf {
						State {
							positive_dnf: cloned_dnf,
							negative_dnf: state.negative_dnf.clone(),
						}
					} else {
						State {
							positive_dnf: state.positive_dnf.clone(),
							negative_dnf: cloned_dnf,
						}
					};

					result.push(modified_state);
				}
			},
			Self::InsertOneLiteral {
				neighbourhood_limit,
				shuffle,
			} => {
				// TODO for completeness of search space
			},
			Self::RemoveEmptyClauses => {
				// TODO
				// let mut cloned_dnf = dnf.clone();
				// for clause in cloned_dnf.clauses() {
				// 	if clause.is_empty() {
				// 		cloned_dnf.remove_clause(clause);
				// 	}
				//}
			},
			Self::RemoveFromAllClauses => {
				for (dnf, which_dnf) in state.dnfs() {
					let indices_present_in_all =
						dnf.clauses().iter().fold(Bitmap::mask(SIZE), |acc, curr_clause| {
							acc & *curr_clause.appearances()
						});

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
						let modified_state = if which_dnf {
							State {
								positive_dnf: cloned_dnf,
								negative_dnf: state.negative_dnf.clone(),
							}
						} else {
							State {
								positive_dnf: state.positive_dnf.clone(),
								negative_dnf: cloned_dnf,
							}
						};
						result.push(modified_state);
						result.shuffle(&mut thread_rng());
					}
				}
			},
		}
		debug!("Found {} neighbours.", result.len());
		result
	}
}
