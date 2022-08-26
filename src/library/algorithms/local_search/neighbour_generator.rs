//! Provides neighbourhood generation methods for run state.

use crate::boolean_formulae::dnf::DNF;

use serde::{
	Serialize,
	Deserialize,
};

/// Distinguishes different methods for generating Neighbourhoods of a `DNF`.
#[derive(Debug, Serialize, Deserialize)]
pub enum NeighbourhoodGenerator {
	/// Remove one literal.
	RemoveOneLiteral,
	/// Remove all empty clauses.
	RemoveEmptyClauses,
}

impl NeighbourhoodGenerator {
	/// Generates the neighbourhood of the `DNF` according to the generator strategy.
	pub fn generate_neighbourhood(&self, dnf: &DNF) -> Vec<DNF> {
		let mut result = Vec::new();
		match self {
			Self::RemoveOneLiteral => {
				// let a = dnf.clauses().into_iter().enumerate();
				for (id, clause) in dnf.clauses().iter().enumerate() {
					for literal in clause.literals() {
						let mut cloned_dnf = dnf.clone();
						let selected_clause = cloned_dnf.mut_clauses().get_mut(id).unwrap();
						selected_clause.remove_literal(literal.feature_id());
						result.push(cloned_dnf);
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
		result
	}
}
