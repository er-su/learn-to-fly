use std::{collections::btree_map::Range, f32::consts::*, ops::RangeInclusive};
use rand::{distributions::uniform::SampleRange, Rng, RngCore};

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub num_eye_cells: usize,
    pub num_hidden_layers: usize, // Excluding input and output layers
    pub hidden_layer_size: usize,
    pub fov_range: f32,
    pub fov_angle:f32,
    pub mutation_chance: f32,
    pub mutation_coef: f32,
}

impl Config {
    pub fn new(
        num_eye_cells: usize,
        num_hidden_layers: usize,
        hidden_layer_size: usize,
        fov_range: f32,
        mutation_chance: f32,
        mutation_coef: f32,
    ) -> Self {
        Self {
            num_eye_cells,
            num_hidden_layers,
            hidden_layer_size,
            fov_range,
            fov_angle: PI + FRAC_PI_4, // Don't allow modification for angle as optimal fov angle is always 360
            mutation_chance,
            mutation_coef,
        }
    }

    pub fn random(rng: &mut dyn RngCore,
        cells_range: (usize, usize),
        layers_range: (usize, usize),
        size_range: (usize, usize),
        range_range: (f32, f32),
        chance_range: (f32, f32),
        coef_range: (f32, f32),
    ) -> Self {
        let num_eye_cells = rng.gen_range(cells_range.0..=cells_range.1);
        let num_hidden_layers = rng.gen_range(layers_range.0..=layers_range.1);
        let hidden_layer_size = rng.gen_range(size_range.0..=size_range.1);
        let fov_range = rng.gen_range(range_range.0..=range_range.1);
        let mutation_chance = rng.gen_range(chance_range.0..=chance_range.1);
        let mutation_coef = rng.gen_range(coef_range.0..=coef_range.1);

        Self {
            num_eye_cells,
            num_hidden_layers,
            hidden_layer_size,
            fov_range,
            fov_angle: PI + FRAC_PI_4,
            mutation_chance,
            mutation_coef,
        }
    }

    pub fn range_random(
        rng: &mut dyn RngCore,
        cells_range: impl SampleRange<usize>,
        layers_range: impl SampleRange<usize>,
        size_range: impl SampleRange<usize>,
        range_range: impl SampleRange<f32>,
        chance_range: impl SampleRange<f32>,
        coef_range: impl SampleRange<f32>,
    ) -> Self {
        let num_eye_cells = rng.gen_range(cells_range);
        let num_hidden_layers = rng.gen_range(layers_range);
        let hidden_layer_size = rng.gen_range(size_range);
        let fov_range = rng.gen_range(range_range);
        let mutation_chance = rng.gen_range(chance_range);
        let mutation_coef = rng.gen_range(coef_range);

        Self {
            num_eye_cells,
            num_hidden_layers,
            hidden_layer_size,
            fov_range,
            fov_angle: PI + FRAC_PI_4,
            mutation_chance,
            mutation_coef,
        }
    }

    pub fn config_range_random(rng: &mut dyn RngCore, config_range: ConfigRange) -> Self {
        let num_eye_cells = rng.gen_range(config_range.cells_range);
        let num_hidden_layers = rng.gen_range(config_range.layers_range);
        let hidden_layer_size = rng.gen_range(config_range.size_range);
        let fov_range = rng.gen_range(config_range.range_range);
        let mutation_chance = rng.gen_range(config_range.chance_range);
        let mutation_coef = rng.gen_range(config_range.coef_range);

        Self {
            num_eye_cells,
            num_hidden_layers,
            hidden_layer_size,
            fov_range,
            fov_angle: PI + FRAC_PI_4,
            mutation_chance,
            mutation_coef,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConfigRange {
    pub cells_range: RangeInclusive<usize>,
    pub layers_range: RangeInclusive<usize>,
    pub size_range: RangeInclusive<usize>,
    pub range_range: RangeInclusive<f32>,
    pub chance_range: RangeInclusive<f32>,
    pub coef_range: RangeInclusive<f32>,
}

impl ConfigRange {
    pub fn new(
        cells_range: RangeInclusive<usize>,
        layers_range: RangeInclusive<usize>,
        size_range: RangeInclusive<usize>,
        range_range: RangeInclusive<f32>,
        chance_range: RangeInclusive<f32>,
        coef_range: RangeInclusive<f32>,
    ) -> Self {
        Self {
            cells_range,
            layers_range,
            size_range,
            range_range,
            chance_range,
            coef_range
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;
    use super::*;

    #[test]
    fn testing_random() {
        let mut rng = rand::thread_rng();
        let cells_range: RangeInclusive<usize> = 5..=13;
        let layers_range: RangeInclusive<usize> = 1..=4;
        let size_range: RangeInclusive<usize> = 6..=20;
        let range_range: RangeInclusive<f32> = 0.1..=0.9;
        let chance_range: RangeInclusive<f32> = 0.0..=1.0;
        let coef_range: RangeInclusive<f32> = 0.0..=1.0;

        let config = Config::range_random(&mut rng, cells_range, layers_range, size_range, range_range, chance_range, coef_range);
        println!("{:?}", config);
        assert!(config.num_eye_cells <= 13 && config.num_eye_cells >= 5);
        assert!(config.num_hidden_layers <= 4 && config.num_hidden_layers >= 1);
        assert!(config.hidden_layer_size <= 20 && config.hidden_layer_size >= 6);
        assert!(config.fov_range <= 0.9 && config.fov_range >= 0.1);
        assert!(config.mutation_chance <= 1.0 && config.mutation_chance >= 0.0);
        assert!(config.mutation_coef <= 1.0 && config.mutation_coef >= 0.0);

    }
}