use wasm_bindgen::prelude::*;
use lib_simulation as sim;
use rand::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self {
            rng: rng,
            sim: sim,
        }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }

    pub fn train(&mut self) -> Statistics {
        Statistics::from_other(self.sim.train(&mut self.rng))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,

    #[wasm_bindgen(getter_with_clone)]
    pub foods:Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals()
            .iter()
            .map(Animal::from)
            .collect();

        let foods = world.foods()
            .iter()
            .map(Food::from)
            .collect();

        Self {
            animals,
            foods
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Statistics {
    #[wasm_bindgen(getter_with_clone)]
    pub min: usize,
    #[wasm_bindgen(getter_with_clone)]
    pub avg: f32,
    #[wasm_bindgen(getter_with_clone)]
    pub max: usize,
}

impl Statistics {
    pub fn from_other(other: sim::Statistics) -> Self {
        Self {
            min: other.get_min(),
            avg: other.get_avg(),
            max: other.get_max(),
        }
    }
}