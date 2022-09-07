//! This module provides the actual local search algorithms of this project.

use std::fs::{
	create_dir_all,
	File,
};
use std::io::{
	BufRead,
	BufReader,
	Write,
};
use std::path::Path;
use bitmaps::{
	Bits,
	BitsImpl,
};
use log::{
	debug,
	info,
};
use rayon::prelude::*;
use crate::algorithms::local_search::run_config::Algorithm::BasicHillClimber;
use crate::algorithms::local_search::run_config::RunConfig;
use crate::algorithms::local_search::state::State;
use crate::algorithms::visualization::to_image::ToImage;
use crate::boolean_formulae::clause::Clause;
use crate::boolean_formulae::data::Sample;
use crate::boolean_formulae::dnf::DNF;

mod regularizer;
mod neighbourhood_generator;
mod state;
pub mod run_config;

/// A basic hill climber
///
/// # Panics
pub fn basic_hill_climber<const DATA_DIM: usize>(run_config: &RunConfig<DATA_DIM>)
where
	BitsImpl<DATA_DIM>: Bits,
{
	debug!("Starting hill climber with config {:?}", run_config);
	let regularizer = run_config.regularizer;
	let BasicHillClimber(max_iterations) = run_config.algorithm;

	let data_file =
		File::open(Path::new(&run_config.data_path)).expect("Could not open the data file you provided.");

	let output_path = Path::new("output");
	let metrics_path = output_path.join("metrics");
	create_dir_all(&metrics_path).expect("Could not create output directory.");
	let mut output_file = File::create(&output_path.join("output")).expect("Could not create output file.");
	let mut metrics_file =
		File::create(&metrics_path.join("metrics")).expect("Could not create metrics file.");

	let (positive_samples, negative_samples): (Vec<Sample<DATA_DIM>>, Vec<Sample<DATA_DIM>>) =
		BufReader::new(data_file)
			.lines()
			.filter_map(Result::ok)
			.filter_map(|line| serde_json::from_str(&line).ok())
			.partition(Sample::label);

	let positive_dnf = DNF::new(positive_samples.par_iter().map(Clause::from).collect());
	let negative_dnf = DNF::new(negative_samples.par_iter().map(Clause::from).collect());

	let mut current_state: State<DATA_DIM> = State {
		positive_dnf,
		negative_dnf,
	};
	let mut best_state = current_state.clone();

	let mut iteration: u32 = 0;

	while iteration <= max_iterations {
		info!("{}", iteration);
		if let Some(metrics) = &run_config.metrics {
			if iteration % metrics.regularizer_frequency == 0 {
				metrics_file
					.write_all(
						format!(
							"Iteration: {}: DNF-regularizer value: {}\n",
							iteration,
							regularizer.regularize(&current_state),
						)
						.as_bytes(),
					)
					.expect("Could not write to the metrics file.");
			}
			if iteration % metrics.picture_frequency == 0 {
				current_state
					.positive_dnf
					.to_image(28, 28)
					.unwrap()
					.save(metrics_path.join(format!("iteration-{}-positive.png", iteration).as_str()))
					.unwrap();
				current_state
					.negative_dnf
					.to_image(28, 28)
					.unwrap()
					.save(metrics_path.join(format!("iteration-{}-negative.png", iteration).as_str()))
					.unwrap();
			}
		}

		let best_neighbour = run_config
			.neighbourhood_generators
			.par_iter()
			.flat_map(|generator| generator.generate_neighbourhood(&current_state))
			.filter(|state| state.is_feasible(&positive_samples, &negative_samples))
			.min_by(|a, b| regularizer.regularize(a).cmp(&regularizer.regularize(b)));

		match best_neighbour {
			None => break,
			Some(neighbour) => {
				if regularizer.regularize(&neighbour) < regularizer.regularize(&best_state) {
					best_state = neighbour.clone();
				}
				if regularizer.regularize(&neighbour) < regularizer.regularize(&current_state) {
					current_state = neighbour.clone();
				}
			},
		}

		iteration += 1;
	}
	output_file
		.write_all(
			format!(
				"Best state after {} iterations:\n Positive DNF: {}\n Negative DNF: {}",
				iteration,
				serde_json::to_string(&best_state.positive_dnf).unwrap(),
				serde_json::to_string(&best_state.negative_dnf).unwrap()
			)
			.as_bytes(),
		)
		.expect("Could not write final DNFs to output file.");
}
