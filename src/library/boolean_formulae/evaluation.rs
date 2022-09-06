//! Provides the `Evaluate` trait every boolean formula should implement.

use bitmaps::{
	Bits,
	BitsImpl,
};
use crate::boolean_formulae::data::{Sample,};

/// The general semantic of everything that can be evaluated under a variable assignment
/// (e.g. Literals, Clauses and DNFs).
pub trait Evaluate<const SIZE: usize>
where
	BitsImpl<SIZE>: Bits,
{
	/// Evaluates itself under the given variable assignment.
	///
	/// # Errors
	/// If the data is to short, i.e. there is a `Literal` with a `FeatureID` that is not
	/// present in the dataset, this will return an `Err(ErrorKind::InsufficientData)`.
	fn evaluate(&self, data: &Sample<SIZE>) -> bool;
}
