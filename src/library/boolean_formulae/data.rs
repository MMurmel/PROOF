//! Provides representation of boolean data over multidimensional feature-space.

use bitmaps::Bitmap;
use serde::{
	Serialize,
	Deserialize,
};
/// Each feature of the data is identified by a single number.
pub type FeatureID = usize;

/// The general struct to represent a manifestation of the feature space.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sample<const SIZE: usize> {
	/// The label of the sample.
	label:    bool,
	/// The data of the sample.
	features: Bitmap<SIZE>,
}

impl<const SIZE: usize> Sample<SIZE> {
	/// Creates a sample from a manifestation of a feature space.
	#[must_use]
	pub fn new(label: bool, features: Bitmap<SIZE>) -> Self { Self { label, features } }

	/// Returns the label of the sample.
	#[must_use]
	pub const fn label(&self) -> bool { self.label }

	/// Returns the features of the sample.
	#[must_use]
	pub fn features(&self) -> Bitmap<{ SIZE }> { self.features.clone() }

	/// Gets the data at the specified index, i.e. the assignment of the specified
	/// variable.
	#[must_use]
	pub fn at_feature(&self, index: FeatureID) -> Option<bool> {
		if index >= SIZE {
			None
		}
		Some(self.features.get(index))
	}
}
