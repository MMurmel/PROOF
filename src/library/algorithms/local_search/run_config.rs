//! Provides configuration parameters for a run of the search algorithm.
use crate::algorithms::local_search::neighbourhood_generator::NeighbourhoodGenerator;
use crate::algorithms::local_search::regularizer::Regularizer;
use serde::{
	Serialize,
	Deserialize,
};
use crate::algorithms::local_search::algorithms::Algorithm;

/// Holds information about all possibly configurable parameters of a run.
#[derive(Debug, Serialize, Deserialize)]
pub struct RunConfig<const DATA_DIM: usize> {
	/// How often this configuration should be run.
	pub run_count:                u32,
	/// The path to the data sample file.
	pub data_path:                String,
	/// The metrics for this run.
	pub metrics:                  Option<Metrics>,
	/// Strategies for neighbourhood generation.
	pub neighbourhood_generators: Vec<NeighbourhoodGenerator>,
	/// Regularizer strategy.
	pub regularizer:              Regularizer,
	/// Which Algorithm to use.
	pub algorithm:                Algorithm,
}

impl<const DATA_DIM: usize> Default for RunConfig<DATA_DIM> {
	fn default() -> Self {
		Self {
			run_count:                1,
			data_path:                "data/prepared_data_short.json".to_string(),
			metrics:                  Some(Metrics {
				picture_frequency:     50,
				regularizer_frequency: 50,
			}),
			neighbourhood_generators: vec![
				// NeighbourhoodGenerator::RemoveOneLiteral {
				// 	neighbourhood_limit: Some(100),
				// 	shuffle:             true,
				// },
				NeighbourhoodGenerator::RemoveFromAllClauses,
			],
			regularizer:              Regularizer::DepthAndLength,
			algorithm:                Algorithm::BasicHillClimber { max_iterations: 1600 },
		}
	}
}

/// Holds information about which metrics should be run and where they should be stored.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
	/// How often the resulting DNFs should be converted to pictures.
	pub picture_frequency:     u32,
	/// How often the regularizer value of the DNFs should be stored.
	pub regularizer_frequency: u32,
}
