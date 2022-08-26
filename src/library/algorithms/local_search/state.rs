//! Any feasible solution to the local search problem is represented using this state.
use crate::boolean_formulae::dnf::DNF;

///
pub struct State {
	/// Characterizes all positive samples.
	positive_dnf: DNF,
	/// Characterizes all negative samples.
	negative_dnf: DNF,
}
