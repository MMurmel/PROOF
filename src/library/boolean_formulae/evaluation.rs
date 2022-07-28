//! Provides the `Evaluate` trait every boolean formula should implement.
use std::fmt::{
	Debug,
	Display,
	Formatter,
};
use crate::boolean_formulae::data::{
	AtomID,
	Sample,
};

/// Error Enum for everything that could go wrong during evaluation of a boolean formula
/// under a given variable assignment
#[derive(PartialEq, Eq)]
pub enum ErrorKind {
	/// There is no data for one or more of the evaluated literals.
	InsufficientData(AtomID),
}

impl Debug for ErrorKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InsufficientData(id) => {
				write!(f, "Insufficient data at AtomID {}", id)
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
		}
	}
}

impl std::error::Error for ErrorKind {}

/// The general semantic of everything that can be evaluated under a variable assignment
/// (i.e. data).
pub trait Evaluate {
	/// Evaluates itself under the given variable assignment.
	///
	/// # Errors
	/// If the data is to short, i.e. there is a `Literal` of an `AtomIndex` not present
	/// in the dataset, this will return an `Err(ErrorKind::InsufficientData)`.
	fn evaluate(&self, data: &Sample) -> Result<bool, ErrorKind>;
}
