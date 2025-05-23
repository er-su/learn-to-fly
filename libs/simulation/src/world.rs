use crate::*;

#[derive(Clone, Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        Self{
            animals: animals,
            foods: foods,
        }
    }

    pub fn from_config(rng: &mut dyn RngCore, config: Config) -> Self {
        let animals = (0..40)
            .map(|_| Animal::from_config(rng, config))
            .collect();
        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();
        
        Self{
            animals: animals,
            foods: foods,
        }

    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}