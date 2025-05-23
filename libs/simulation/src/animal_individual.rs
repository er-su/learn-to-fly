use crate::*;

pub struct AnimalIndividual {
    pub(crate) fitness: f32,
    pub(crate) chromosome: Chromosome
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn fitness(&self) ->f32 {
        self.fitness
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}