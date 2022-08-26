use crate::algorithms::local_search::neighbour_generator::NeighbourhoodGenerator;
use crate::algorithms::local_search::regularizer::Regularizer;
use serde::{
	Serialize,
	Deserialize,
};

/// Holds information about all possibly configurable parameters of a run.
#[derive(Serialize, Deserialize)]
pub struct RunConfig {
	/// How often this configuration should be run.
	run_count:                u32,
	/// The path to the data sample file.
	data_path:                String,
	/// The path to the file where the DNFs will be stored.
	dnf_path:                 String,
	/// The metrics for this run.
	metrics:                  Option<Metrics>,
	/// Strategies for neighbourhood generation.
	neighbourhood_generators: Vec<NeighbourhoodGenerator>,
	/// Regularizer strategy.
	regularizer:              Regularizer,
}

/// How often, if at all, metrics should be printed.
type Frequency = Option<u32>;

/// Holds information about which metrics should be run and where they should be stored.
#[derive(Serialize, Deserialize)]
struct Metrics {
	/// The path to the folder where all metrics will be stored.
	metric_folder:         String,
	/// How often the resulting DNFs should be converted to pictures.
	picture_frequency:     Frequency,
	/// How often the regularizer value of the DNFs should be stored.
	regularizer_frequency: Frequency,
}
