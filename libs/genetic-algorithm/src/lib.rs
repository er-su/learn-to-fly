pub mod chromosome;
pub mod selection_method;
pub mod crossover_method;
pub mod mutation_method;

use rand::{Rng,RngCore};
use rand::seq::SliceRandom;
use chromosome::Chromosome;
use selection_method::SelectionMethod;
use crossover_method::CrossoverMethod;
use mutation_method::MutationMethod;

pub trait Individual {
    fn fitness(&self) ->f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where 
    S: SelectionMethod
{
    pub fn new(selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method)
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where 
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                //crossover
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                //mutation
                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect()
    }
}