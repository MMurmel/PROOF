//! Provides configuration parameters for a run of the search algorithm.
use crate::algorithms::local_search::neighbour_generator::NeighbourhoodGenerator;
use crate::algorithms::local_search::regularizer::Regularizer;
use serde::{
	Serialize,
	Deserialize,
};

/// Holds information about all possibly configurable parameters of a run.
#[derive(Debug, Serialize, Deserialize)]
pub struct RunConfig {
	/// How often this configuration should be run.
	run_count:                u32,
	/// The path to the data sample file.
	data_path:                String,
	/// The metrics for this run.
	metrics:                  Option<Metrics>,
	/// Strategies for neighbourhood generation.
	neighbourhood_generators: Vec<NeighbourhoodGenerator>,
	/// Regularizer strategy.
	regularizer:              Regularizer,
}

impl Default for RunConfig {
	fn default() -> Self {
		Self {
			run_count:                1,
			data_path:                "data/prepared_data.json".to_string(),
			metrics:                  Some(Metrics {
				picture_frequency:     Some(10),
				regularizer_frequency: Some(10),
			}),
			neighbourhood_generators: vec![NeighbourhoodGenerator::RemoveOneLiteral],
			regularizer:              Regularizer::DepthAndLength,
		}
	}
}

/// How often, if at all, metrics should be printed.
type Frequency = Option<u32>;

/// Holds information about which metrics should be run and where they should be stored.
#[derive(Debug, Serialize, Deserialize)]
struct Metrics {
	/// How often the resulting DNFs should be converted to pictures.
	picture_frequency:     Frequency,
	/// How often the regularizer value of the DNFs should be stored.
	regularizer_frequency: Frequency,
}
