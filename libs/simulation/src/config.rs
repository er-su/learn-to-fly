use std::f32::consts::*;

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub(crate) num_eye_cells: usize,
    pub(crate) num_hidden_layers: usize, // Excluding input and output layers
    pub(crate) hidden_layer_size: usize,
    pub(crate) fov_range: f32,
    pub(crate) fov_angle:f32,
}

impl Config {
    pub fn new(
        num_eye_cells: usize,
        num_hidden_layers: usize,
        hidden_layer_size: usize,
        fov_range: f32,
    ) -> Self {
        Self {
            num_eye_cells,
            num_hidden_layers,
            hidden_layer_size,
            fov_range,
            fov_angle: PI + FRAC_PI_4, // Don't allow modification for angle as optimal fov angle is always 360
        }
    }
}