//! This module provides the conversion to an image from clauses and DNFs respectively.
use std::collections::HashMap;
use image::{
	Rgb,
	RgbImage,
};

/// Red in rgb.
const RED: (u8, u8, u8) = (255, 0, 0);
/// Green in rgb.
const GREEN: (u8, u8, u8) = (0, 255, 0);

use crate::boolean_formulae::clause::Clause;
use crate::boolean_formulae::dnf::DNF;

/// Error Enum for converting logic formulas into images.
#[derive(Debug)]
pub enum ErrorKind {
	/// The provided Image Dimensions were to small to fit the whole formula.
	DimensionsTooSmall,
	/// Something unexpected happened.
	UnknownError,
}

/// The common semantic of converting a logic formula into an image.
pub trait ToImage {
	/// Convert into an image with the given dimensions.
	///
	/// # Errors
	/// Will
	fn to_image(&self, width: u32, height: u32) -> Result<RgbImage, ErrorKind>;
}

impl ToImage for Clause {
	fn to_image(&self, width: u32, height: u32) -> Result<RgbImage, ErrorKind> {
		let mut image = RgbImage::new(width, height);

		for literal in self.literals() {
			let atom_id = literal.atom_id();
			if atom_id >= width * height {
				return Err(ErrorKind::DimensionsTooSmall);
			}
			let column = atom_id % width;
			let row = atom_id / width;
			let color: (u8, u8, u8) = if literal.parity() { GREEN } else { RED };
			image.put_pixel(column, row, Rgb::from([color.0, color.1, color.2]));
		}
		Ok(image)
	}
}

impl ToImage for DNF {
	fn to_image(&self, width: u32, height: u32) -> Result<RgbImage, ErrorKind> {
		let clause_count = self.length() as u32;
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
					.or_insert((0, 0, 0));
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

		for ((x, y), (r, g, b)) in helper_map.into_iter() {
			image.put_pixel(x, y, Rgb::from([r, g, b]));
		}

		Ok(image)
	}
}