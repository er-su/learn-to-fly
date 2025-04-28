use lib_config::{Config, ConfigRange};
use lib_simulation::{Simulation, Statistics};
use rand::{Rng, RngCore};

pub struct Agent {
    pub simulation: Simulation,
    pub config: Config,
    pub last_stats: Statistics,
}

impl Agent {
    pub fn random(rng: &mut dyn RngCore, random_range: ConfigRange) -> Self {
        let simulation = Simulation::random(rng);
        let config = Config::config_range_random(rng, random_range);
        let last_stats = Statistics::default();

        Self {
            simulation,
            config,
            last_stats,
        }
    }

    pub fn get_weighted_score(&self) -> f32 {
        let stats = self.last_stats;
        0.25 * (stats.get_min() as f32) + 0.5 * stats.get_avg() + 0.25 * (stats.get_max() as f32)
    }
}