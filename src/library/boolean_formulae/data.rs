//! Provides representation of boolean data over multidimensional feature-space.

use std::hash::Hash;
use bitmaps::{
	Bitmap,
	Bits,
	BitsImpl,
};
use serde::{
	Serialize,
	Deserialize,
	Serializer,
	Deserializer,
};
/// Identifier type for features of data.
pub type FeatureID = usize;

#[derive(Debug, Serialize, Deserialize)]
/// A Wrapper for easier Serialization and Deserialization Access.
struct SampleWrapper {
	/// The Samples Label
	label:    bool,
	/// The Samples features, in order.
	features: Vec<bool>,
}

impl<const SIZE: usize> From<SampleWrapper> for Sample<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn from(wrapper: SampleWrapper) -> Self {
		assert_eq!(
			wrapper.features.len(),
			SIZE,
			"Could not cast ClauseWrapper to Clause due to difference in length: \
			 ClauseWrapper.features.len(): {}, SIZE: {}",
			wrapper.features.len(),
			SIZE
		);
		let mut features: Bitmap<SIZE> = Bitmap::new();
		for (index, &feature) in wrapper.features.iter().enumerate() {
			features.set(index, feature);
		}
		Self {
			label: wrapper.label,
			features,
		}
	}
}

impl<const SIZE: usize> From<Sample<SIZE>> for SampleWrapper
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn from(sample: Sample<SIZE>) -> Self {
		Self {
			label:    sample.label,
			features: (0..sample.features.len())
				.into_iter()
				.map(|index| sample.features.get(index))
				.collect(),
		}
	}
}

/// Represents a manifestation of the feature space in a bitmap format.
#[derive(Debug, Clone)]
pub struct Sample<const SIZE: usize>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	/// The label of the sample.
	label:    bool,
	/// The data of the sample.
	features: Bitmap<SIZE>,
}

impl<const SIZE: usize> Sample<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	/// Creates a sample from a manifestation of a feature space.
	#[must_use]
	pub const fn new(label: bool, features: Bitmap<SIZE>) -> Self { Self { label, features } }

	/// Returns the label of the sample.
	#[must_use]
	pub const fn label(&self) -> bool { self.label }

	/// Returns the features of the sample.
	#[must_use]
	pub const fn features(&self) -> Bitmap<{ SIZE }> { self.features }
}

impl<const SIZE: usize> Serialize for Sample<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let wrapper = SampleWrapper::from(self.clone());
		wrapper.serialize(serializer)
	}
}

impl<'de, const SIZE: usize> Deserialize<'de> for Sample<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let wrapper = SampleWrapper::deserialize(deserializer)?;
		Ok(Self::from(wrapper))
	}
}
unsafe impl<const SIZE: usize> Send for Sample<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
}
unsafe impl<const SIZE: usize> Sync for Sample<SIZE>
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
}
