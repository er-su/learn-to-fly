use std::f32::consts::*;
use rand::{Rng, RngCore};

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
}

mod tests {
    use super::*;

    #[test]
    fn testing_random() {
        let mut rng = rand::thread_rng();
        let cells_range: (usize, usize) = (5,13);
        let layers_range: (usize, usize) = (1,4);
        todo!();

    }
}