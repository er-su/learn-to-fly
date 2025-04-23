mod animal;
mod food;
mod world;
mod eye;
mod animal_individual;
mod brain;

use lib_neural_network as nn;
use nn::matrix_network as mn;
use lib_genetic_algorithm as ga;
use lib_config::Config;
use nalgebra as na;
use rand::{RngCore, Rng};
use std::f32::consts::FRAC_PI_2;
pub use self::{animal::*, food::*, world::*, eye::*, animal_individual::*, brain::*};

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GEN_LEN: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.03),
        );

        Self {
            world,
            ga,
            age: 0,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    // Perform a single step forward
    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GEN_LEN {
            let stats = Statistics::find_stats(&self.world.animals);
            self.evolve(rng);
            Some(stats)
            
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> Statistics {
        loop {
            match self.step(rng) {
                None => continue,
                Some(stats) => return stats,
            }
        }
    }

    pub fn optimize_from_config(&mut self, rng: &mut dyn RngCore, config: Config) -> Vec<Statistics> {
        todo!()
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        let current_population: Vec<_> = self.world
            .animals
            .iter()
            .map(|animal| AnimalIndividual::from_animal(animal))
            .collect();
        let evolved_population = self.ga.evolve(rng, &current_population);
        self.world.animals = evolved_population
                .into_iter()
                .map(|individual| individual.into_animal(rng))
                .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.007 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position, 
                animal.rotation, 
                &self.world.foods
        );
    
        let response = animal.brain.nn.forward(vision);

        let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
        let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

        animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
        animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);

        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Statistics {
    pub(crate) min: usize,
    pub(crate) avg: f32,
    pub(crate) max: usize,
}

impl Statistics {
    pub fn find_stats(population: &[Animal]) -> Self {
        let min = population
            .into_iter()
            .min_by(|x, y| x.satiation.cmp(&y.satiation))
            .unwrap()
            .satiation;

        let max = population
            .into_iter()
            .max_by(|x, y| x.satiation.cmp(&y.satiation))
            .unwrap()
            .satiation;

        let sum: usize = population
            .iter()
            .map(|animal| animal.satiation)
            .sum::<usize>();
        
        let len: f32 = population.len() as f32;

        let avg = (sum as f32) / len;

        Self {
            min,
            avg,
            max,
        }    
    }

    pub fn get_min(self) -> usize {
        self.min
    }

    pub fn get_avg(self) -> f32 {
        self.avg
    }

    pub fn get_max(self) -> usize {
        self.max
    }
}