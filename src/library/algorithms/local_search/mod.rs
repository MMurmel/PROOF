//! This module provides the actual local search algorithms of this project.

use std::fs;
use std::fs::{
	create_dir_all,
	File,
};
use std::hash::Hash;
use std::io::{Write,};
use std::path::{Path,};
use std::time::Instant;
use bitmaps::{
	Bits,
	BitsImpl,
};
use log::{debug,};
use rayon::prelude::*;
use crate::algorithms::local_search::algorithms::AlgorithmRunner;

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
mod algorithms;

/// A basic hill climber
///
/// # Panics
pub fn local_search<const DATA_DIM: usize>(run_config: &RunConfig<DATA_DIM>)
where
	BitsImpl<DATA_DIM>: Bits,
	<BitsImpl<{ DATA_DIM }> as Bits>::Store: Hash,
{
	// Instantiate components from run_config.
	let regularizer = run_config.regularizer;
	let neighbourhood_generators = run_config.neighbourhood_generators.clone();
	let algorithm = run_config.algorithm;

	// Read data into memory.
	let (positive_samples, negative_samples): (Vec<Sample<DATA_DIM>>, Vec<Sample<DATA_DIM>>) =
		serde_json::from_str::<Vec<Sample<DATA_DIM>>>(
			&fs::read_to_string(Path::new(&run_config.data_path))
				.expect("Could not read from the provided datafile."),
		)
		.expect("The datafile could be read, but it contained an error and could not be parsed to Samples.")
		.into_iter()
		.partition(Sample::label);
	// Create starting DNFs from memory.
	let positive_dnf = DNF::new(positive_samples.par_iter().map(Clause::from).collect());
	let negative_dnf = DNF::new(negative_samples.par_iter().map(Clause::from).collect());
	let initial_state: State<DATA_DIM> = State {
		positive_dnf,
		negative_dnf,
	};

	// Create general output-paths.
	let output_dir = Path::new("output");

	for current_run in 1..=run_config.run_count {
		debug!("Starting run #{}", current_run);
		// Create run-specific output directories and files.
		let run_dir = output_dir.join(format!("run-{}", current_run));
		create_dir_all(&run_dir)
			.unwrap_or_else(|_| panic!("Could not create output directory for run {}.", current_run));
		let metrics_dir = run_dir.join("metrics");
		create_dir_all(&metrics_dir)
			.unwrap_or_else(|_| panic!("Could not create metrics directory in run {}.", current_run));
		let mut output_file =
			File::create(&run_dir.join("final_state")).expect("Could not create output file.");
		let mut metrics_file =
			File::create(&metrics_dir.join("metrics.csv")).expect("Could not create metrics file.");

		// Prepare tracking of current and best state.
		let current_state = initial_state.clone();
		let mut best_state = current_state.clone();

		// Pre-Run metrics
		if let Some(_metrics) = &run_config.metrics {
			metrics_file
				.write_all(b"Iteration,Elapsed-Time,Regularizer-Value\n")
				.expect("Could not write to metrics file.");
			save_metrics(
				&mut metrics_file,
				"0",
				"0",
				regularizer.regularize(&current_state).to_string().as_str(),
			);
			generate_pictures(&current_state, &metrics_dir, "0");
		}

		let mut algorithm_runner = AlgorithmRunner::new(
			algorithm,
			current_state,
			positive_samples.as_slice(),
			negative_samples.as_slice(),
			neighbourhood_generators.clone(),
			regularizer,
		);

		let time_0 = Instant::now();

		while let Some(current_state) = algorithm_runner.step() {
			debug!("In Iteration {}", algorithm_runner.iteration());
			if let Some(metrics) = &run_config.metrics {
				let iteration = algorithm_runner.iteration();
				if iteration % metrics.regularizer_frequency == 0 {
					save_metrics(
						&mut metrics_file,
						iteration.to_string().as_str(),
						format!("{:?}", time_0.elapsed()).as_str(),
						regularizer.regularize(&current_state).to_string().as_str(),
					);
				}
				if iteration % metrics.picture_frequency == 0 {
					generate_pictures(&current_state, &metrics_dir, iteration.to_string().as_str());
				}
			}

			if regularizer.regularize(&current_state) < regularizer.regularize(&best_state) {
				best_state = current_state.clone();
			}
		}

		if let Some(_metrics) = &run_config.metrics {
			save_metrics(
				&mut metrics_file,
				"final",
				format!("{:?}", time_0.elapsed()).as_str(),
				regularizer.regularize(&best_state).to_string().as_str(),
			);
			generate_pictures(&best_state, &metrics_dir, "final");
		}

		output_file
			.write_all(serde_json::to_string(&best_state).unwrap().as_bytes())
			.expect("Could not write final state to output file.");
	}
}

/// Creates Visualizations of the current state and saves them under the provided path
/// with filenames distinguished by the current iteration.
fn generate_pictures<const SIZE: usize>(state: &State<SIZE>, path: &Path, label: &str)
where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	state
		.positive_dnf
		.to_image(28, 28)
		.unwrap()
		.save(path.join(format!("iteration-{}-positive.png", label).as_str()))
		.unwrap();
	state
		.negative_dnf
		.to_image(28, 28)
		.unwrap()
		.save(path.join(format!("iteration-{}-negative.png", label).as_str()))
		.unwrap();
}

/// Writes metrics generated by the regularizer to the metrics file.
fn save_metrics<const SIZE: usize>(
	metrics_file: &mut File,
	iteration: &str,
	elapsed_time: &str,
	regularization: &str,
) where
	BitsImpl<SIZE>: Bits,
	<BitsImpl<{ SIZE }> as Bits>::Store: Hash,
{
	metrics_file
		.write_all(format!("{},{},{}\n", iteration, elapsed_time, regularization,).as_bytes())
		.expect("Could not write to the metrics file.");
}
