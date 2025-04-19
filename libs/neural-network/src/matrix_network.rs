use rand::{Rng, RngCore};

use crate::{util::{matrix_vector_mult, vector_vector_add}, Layer, LayerTopology};

#[derive(Debug)]
pub struct MatrixNetwork {
    layers: Vec<MatrixLayer>
}

impl MatrixNetwork {
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layer| MatrixLayer::random(rng, layer[0].neurons, layer[1].neurons))
            .collect();

        Self {
            layers
        }
    }

    pub fn forward(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.forward(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| {
                let mut temp = layer.weights.clone();
                temp.append(&mut layer.bias.clone());
                temp
            })
    }

    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item = f32>
    ) -> Self {

        let mut weights = weights.into_iter();

        let matrix_layers = layers
            .windows(2)
            .map(|layer| {
                let weight: Vec<f32> = (0..layer[0].neurons * layer[1].neurons)
                    .map(|_| weights.next().unwrap())
                    .collect();
                let bias: Vec<f32> = (0..layer[1].neurons)
                    .map(|_| weights.next().unwrap())
                    .collect();

                MatrixLayer {
                    weights: weight,
                    bias,
                    num_inputs: layer[0].neurons,
                    num_outputs: layer[1].neurons,
                }


            }).collect();

        Self {
            layers: matrix_layers,
        }

            
            
    }
}

#[derive(Clone, Debug)]
pub struct MatrixLayer {
    weights: Vec<f32>,
    bias: Vec<f32>,
    num_inputs: usize,
    num_outputs: usize,
}

impl MatrixLayer {
    fn random(rng: &mut dyn RngCore, num_inputs: usize, num_outputs: usize) -> Self {
        let weights = (0..num_inputs * num_outputs)
            .into_iter()
            .map(|_| rng.gen_range(-1.0..=1.0) as f32)
            .collect();

        let bias = (0..num_outputs)
            .into_iter()
            .map(|_| rng.gen_range(-1.0..=1.0) as f32)
            .collect();

        Self {
            weights,
            bias,
            num_inputs,
            num_outputs,
        }
    }

    fn forward(&self, inputs: Vec<f32>) -> Vec<f32> {
        assert_eq!(inputs.len(), self.num_inputs);
        
        let mut ans: Vec<f32> = matrix_vector_mult(&self.weights, &inputs, self.num_outputs, self.num_inputs);
        ans = vector_vector_add(&ans, &self.bias);
        ans.into_iter()
            .map(|val| val.max(0.0))
            .collect()
    }
}

mod tests {
    use rand::thread_rng;

    use super::*;


    #[test]
    pub fn testing_weights() {
        let topology  = [
            LayerTopology{
                neurons: 5
            },
            LayerTopology {
                neurons: 10
            },
            LayerTopology {
                neurons: 2
            }
        ];
        let mut rng = thread_rng();
        let network = MatrixNetwork::random(&mut rng, &topology);
        let weights = network.weights().collect::<Vec<f32>>();
        // Passed
        assert_eq!(weights.len(), 5 * 10 + 10 * 2 + 10 + 2);
    }
}
