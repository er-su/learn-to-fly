use crate::*;

pub trait CrossoverMethod {
    fn crossover(&self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome
    ) -> Chromosome;
}

#[derive(Debug)]
pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
    fn crossover(&self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) {a} else {b})
            .collect()
    }
}