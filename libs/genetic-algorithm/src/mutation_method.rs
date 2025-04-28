use crate::*;
use lib_config::Config;
pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

#[derive(Debug)]
pub struct GaussianMutation {
    chance: f32,
    coeff: f32,
}

#[allow(unused)]
impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);
        Self {
            chance,
            coeff,
        }
    }

    pub fn from_config(config: Config) -> Self {
        assert!(config.mutation_chance >= 0.0 && config.mutation_chance <= 1.0);
        assert!(config.mutation_coef >= 0.0 && config.mutation_coef <= 1.0);
        Self {
            chance: config.mutation_chance,
            coeff: config.mutation_coef,
        }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign: f32 = if rng.gen_bool(0.5) {-1.0} else {1.0};

            if rng.gen_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}