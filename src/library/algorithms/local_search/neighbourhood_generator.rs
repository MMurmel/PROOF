//! Provides neighbourhood generation methods for run state.

use bitmaps::{
	Bits,
	BitsImpl,
};
use log::debug;

use serde::{
	Serialize,
	Deserialize,
};
use crate::algorithms::local_search::state::State;

/// Distinguishes different methods for generating Neighbourhoods of a `DNF`.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum NeighbourhoodGenerator {
	/// Remove one literal.
	RemoveOneLiteral,
	/// Remove all empty clauses.
	RemoveEmptyClauses,
}

impl NeighbourhoodGenerator {
	/// Generates the neighbourhood of the `DNF` according to the generator strategy.
	pub fn generate_neighbourhood<const SIZE: usize>(&self, state: &State<SIZE>) -> Vec<State<SIZE>>
	where
		BitsImpl<SIZE>: Bits,
	{
		debug!("Started generating neighbourhood.");
		let mut result = Vec::new();
		match self {
			Self::RemoveOneLiteral => {
				// Neighbours of the state by removing one literal from the positive dnf.
				for (id, clause) in state.positive_dnf.clauses().iter().enumerate() {
					for present_id in clause.literal_indices() {
						let mut cloned_dnf = state.positive_dnf.clone();
						let selected_clause = cloned_dnf.mut_clauses().get_mut(id).unwrap();
						selected_clause.remove_literal(present_id);
						result.push(State {
							positive_dnf: cloned_dnf,
							negative_dnf: state.negative_dnf.clone(),
						});
					}
				}
				// Neighbours of the state by removing one literal from the negative dnf.
				for (id, clause) in state.negative_dnf.clauses().iter().enumerate() {
					for present_id in clause.literal_indices() {
						let mut cloned_dnf = state.negative_dnf.clone();
						let selected_clause = cloned_dnf.mut_clauses().get_mut(id).unwrap();
						selected_clause.remove_literal(present_id);
						result.push(State {
							positive_dnf: state.positive_dnf.clone(),
							negative_dnf: cloned_dnf,
						});
					}
				}
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
		}
		debug!("Found {} neighbours.", result.len());
		result
	}
}
