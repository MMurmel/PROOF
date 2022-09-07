//! Provides the `Evaluate` trait every boolean formula should implement.

use bitmaps::{
	Bits,
	BitsImpl,
};
use crate::boolean_formulae::data::{Sample,};

/// The general semantic of everything that can be evaluated under a variable assignment
/// (e.g. Clauses and DNFs).
pub trait Evaluate<const SIZE: usize>
where
	BitsImpl<SIZE>: Bits,
{
	/// Evaluates itself under the given variable assignment.
	fn evaluate(&self, data: &Sample<SIZE>) -> bool;
}
