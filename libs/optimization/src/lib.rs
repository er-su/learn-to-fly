mod agent;

use lib_simulation::{Simulation, Statistics};
use agent::Agent;
use lib_config::{Config, ConfigRange};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng, RngCore};
use core::num;

pub struct Optimizer {
    population: Vec<Agent>,
    config_range: ConfigRange,
    num_agents: usize,
    time: usize,
    rng: ThreadRng,
}

impl Optimizer {
    pub fn random(num_agents: usize) -> Self {
        let config_range = ConfigRange::new(
            1..=20,
            1..=10,
            5..=25,
            0.05..=0.95,
            0.0..=1.0,
            0.0..=1.0
        );
        let mut rng = thread_rng();
        let population = (0..num_agents)
            .map(|_| Agent::random(&mut rng, config_range.clone()))
            .collect();
        
        Self {
            population,
            config_range,
            num_agents,
            time: 0,
            rng: rng,
        }
    }

    pub fn optimize() -> Config {
        todo!()
    }

    // At each step, let the simulations of each agent train for num_gens times
    // Then sort the population vector by weighted score in non-descending order
    fn step(&mut self, num_gens: u32, ) {
        // Advance the population forward num_gens times
        for agent in self.population.iter_mut() {
            for _ in 0..num_gens {
                let stats = agent.simulation.train(&mut self.rng);
                agent.last_stats = stats;
            }
        }
        
        // Sort by weighted score
        self.population.sort_unstable_by(|agent1, agent2|
            agent1.get_weighted_score().partial_cmp(&agent2.get_weighted_score()).unwrap()
        );

        Self::pbt(&mut self.rng, &mut self.population, self.num_agents, &self.config_range);

        self.time += 1;
    }

    fn pbt(rng: &mut dyn RngCore, population: &mut Vec<Agent>, num_agents: usize, config_range: &ConfigRange) {
        // Exploit
        Self::truncation_selection(rng, population, num_agents);
        // Explore
        Self::perturb(rng, population, num_agents, config_range);
    }

    fn truncation_selection(rng: &mut dyn RngCore, population: &mut Vec<Agent>, num_agents: usize) {
        let threshold = (population.len() as f32 * 0.20) as usize;
        assert!(threshold >= 1);

        for i in 0..threshold {
            // Pick a random top 20% fast learner
            let fast_index = rng.gen_range(num_agents - threshold..num_agents);
            
            // Copy over hyperparams
            population[i].config = population[fast_index].config.clone();
        }
    }

    fn perturb(rng: &mut dyn RngCore, population: &mut Vec<Agent>, num_agents: usize, config_range: &ConfigRange) {
        // Perturb hyperparams        
        for agent in population.iter_mut() {
            let up_down = (0..6)
                .map(|_| if rng.gen_bool(0.5) {0.8} else {1.2})
                .collect::<Vec<f32>>();
            
            let agent_config = &mut agent.config;
            
            // Shift num_eye_cells
            agent_config.num_eye_cells = ((agent_config.num_eye_cells as f32 * up_down[0]) as usize).clamp(
                *config_range.cells_range.start(), 
                *config_range.cells_range.end()
            );
            // Shift num_hidden_layers
            agent_config.num_hidden_layers = ((agent_config.num_hidden_layers as f32 * up_down[1]) as usize).clamp(
                *config_range.layers_range.start(), 
                *config_range.layers_range.end()
            );
            // Shift hidden layer size
            agent_config.hidden_layer_size = ((agent_config.hidden_layer_size as f32 * up_down[2]) as usize).clamp(
                *config_range.size_range.start(), 
                *config_range.size_range.end()
            );
            // Shift fov range
            agent_config.fov_range = (agent_config.fov_range * up_down[3]).clamp(
                *config_range.range_range.start(),
                *config_range.range_range.end(),
            );
            // Shift mutation chance
            agent_config.mutation_chance = (agent_config.mutation_chance * up_down[3]).clamp(
                *config_range.chance_range.start(),
                *config_range.chance_range.end(),
            );
            // Shift mutation coef
            agent_config.mutation_coef = (agent_config.mutation_coef * up_down[3]).clamp(
                *config_range.coef_range.start(),
                *config_range.coef_range.end(),
            );

            // Verify the range is preserved
            assert!(config_range.cells_range.contains(&agent_config.num_eye_cells));
            assert!(config_range.layers_range.contains(&agent_config.num_hidden_layers));
            assert!(config_range.size_range.contains(&agent_config.hidden_layer_size));
            assert!(config_range.range_range.contains(&agent_config.fov_range));
            assert!(config_range.chance_range.contains(&agent_config.mutation_chance));
            assert!(config_range.coef_range.contains(&agent_config.mutation_coef));

            // Then reinitalize the world
            agent.simulation = Simulation::from_config(rng, agent.config);
        }
    }
}
