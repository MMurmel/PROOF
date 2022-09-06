//! This module provides the conversion to an image from clauses and DNFs respectively.
use std::collections::HashMap;
use bitmaps::{
	Bits,
	BitsImpl,
};
use image::{
	Rgb,
	RgbImage,
};

/// Red in rgb.
const RED: (u8, u8, u8) = (255, 0, 0);
/// Green in rgb.
const GREEN: (u8, u8, u8) = (0, 255, 0);
/// White in rgb.
const WHITE: (u8, u8, u8) = (0, 0, 0);
/// Black in rgb.
const BLACK: (u8, u8, u8) = (255, 255, 255);

use crate::boolean_formulae::clause::Clause;
use crate::boolean_formulae::data::Sample;
use crate::boolean_formulae::dnf::DNF;

/// Error Enum for converting logic formulas into images.
#[derive(Debug)]
pub enum ErrorKind {
	/// The provided Image Dimensions were too small or too big to fit the whole formula.
	WrongDimensions,
	/// Something unexpected happened.
	UnknownError,
}

/// The common semantic of converting a logic formula into an image.
pub trait ToImage {
	/// Convert into an image with the given dimensions.
	///
	/// # Errors
	/// Will return `ErrorKind::WrongDimensions` if `width * height` is not the same as
	/// the clause dimensionality.
	fn to_image(&self, width: u32, height: u32) -> Result<RgbImage, ErrorKind>;
}

impl<const SIZE: usize> ToImage for Clause<SIZE>
where
	BitsImpl<SIZE>: Bits,
{
	fn to_image(&self, width: u32, height: u32) -> Result<RgbImage, ErrorKind> {
		let mut image = RgbImage::new(width, height);

		for present_id in self.literals() {
			let present_id = present_id as u32;
			if present_id >= width * height {
				return Err(ErrorKind::WrongDimensions);
			}
			let column = present_id % width;
			let row = present_id / width;
			let color: (u8, u8, u8) = if self.literal_at(present_id as usize) {
				GREEN
			} else {
				RED
			};
			image.put_pixel(column, row, Rgb::from([color.0, color.1, color.2]));
		}
		Ok(image)
	}
}

impl<const SIZE: usize> ToImage for DNF<SIZE>
where
	BitsImpl<SIZE>: Bits,
{
	fn to_image(&self, width: u32, height: u32) -> Result<RgbImage, ErrorKind> {
		#[allow(clippy::cast_possible_truncation)]
		let clause_count = self.clauses().len() as u32;
		let mut helper_map: HashMap<(u32, u32), (u32, u32, u32)> = HashMap::new();

		let mut image = RgbImage::new(width, height);
		for clause_image in self.clauses().iter().map(|clause| clause.to_image(width, height)) {
			for (x, y, Rgb([r, g, b])) in clause_image?.enumerate_pixels() {
				helper_map
					.entry((x, y))
					.and_modify(|(r_map, g_map, b_map)| {
						*r_map += u32::from(*r);
						*g_map += u32::from(*g);
						*b_map += u32::from(*b);
					})
					.or_insert((u32::from(*r), u32::from(*g), u32::from(*b)));
			}
		}

		let helper_map = helper_map
			.into_iter()
			.map(|((x, y), (r, g, b))| {
				(
					(x, y),
					(
						u8::try_from(r / clause_count).unwrap(),
						u8::try_from(g / clause_count).unwrap(),
						u8::try_from(b / clause_count).unwrap(),
					),
				)
			})
			.collect::<HashMap<(u32, u32), (u8, u8, u8)>>();

		for ((x, y), (r, g, b)) in helper_map {
			image.put_pixel(x, y, Rgb::from([r, g, b]));
		}

		Ok(image)
	}
}

impl<const SIZE: usize> ToImage for Sample<SIZE>
where
	BitsImpl<SIZE>: Bits,
{
	fn to_image(&self, width: u32, height: u32) -> Result<RgbImage, ErrorKind> {
		let mut image = RgbImage::new(width, height);

		for id in 0..SIZE {
			let id = id as u32;
			if id >= width * height {
				return Err(ErrorKind::WrongDimensions);
			}
			let column = id % width;
			let row = id / width;
			let color: (u8, u8, u8) = if self.feature_at(id as usize) {
				WHITE
			} else {
				BLACK
			};
			image.put_pixel(column, row, Rgb::from([color.0, color.1, color.2]));
		}
		Ok(image)
	}
}
