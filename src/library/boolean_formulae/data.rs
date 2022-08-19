//! Provides representation of boolean data over multidimensional feature-space.

use serde::{
	Serialize,
	Deserialize,
};
/// Each atom is identified by a single number.
pub type AtomID = u32;

/// The general struct to represent a manifestation of the feature space.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
	/// The label of the sample.
	label:    bool,
	/// The data of the sample.
	features: Vec<bool>,
}

impl Sample {
	/// Creates a sample from a manifestation of a feature space.
	#[must_use]
	pub fn new(label: bool, features: Vec<bool>) -> Self { Self { label, features } }

	/// Returns the label of the sample.
	#[must_use]
	pub const fn label(&self) -> bool { self.label }

	/// Returns the features of the sample.
	#[must_use]
	pub fn features(&self) -> &[bool] { self.features.as_slice() }

	/// Gets the data at the specified index, i.e. the assignment of the specified
	/// variable.
	#[must_use]
	pub fn at_feature(&self, index: AtomID) -> Option<&bool> { self.features.get(index as usize) }
}
