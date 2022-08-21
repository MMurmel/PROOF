//! Provides a common Error-Enum for everything that can go wrong when using this module.
use core::fmt::{
	Debug,
	Display,
	Formatter,
};
use std::error::Error;
use crate::boolean_formulae::data::FeatureID;

/// Error Enum for everything that could go wrong during evaluation of a boolean formula
/// under a given variable assignment
#[derive(PartialEq, Eq)]
pub enum ErrorKind {
	/// There is no data for one or more of the evaluated literals.
	InsufficientData(FeatureID),
	/// Access to the literal with `FeatureID` failed, because the clause contains too few
	/// literals.
	FeatureIdNotPresent(FeatureID),
}

impl Debug for ErrorKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InsufficientData(id) => {
				write!(f, "Insufficient data at FeatureID {}", id)
			},
			Self::FeatureIdNotPresent(id) => {
				write!(
					f,
					"Insufficient literals for accessing clause at FeatureID {}",
					id
				)
			},
		}
	}
}

impl Display for ErrorKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InsufficientData(id) => {
				write!(
					f,
					"The data you provided could not be evaluated, because it lacked an entry for the {} \
					 st/nd/th variable.",
					id
				)
			},
			Self::FeatureIdNotPresent(id) => {
				write!(
					f,
					"You tried to access the {}th literal of a clause that does not contain that many \
					 literals.",
					id
				)
			},
		}
	}
}

impl Error for ErrorKind {}
