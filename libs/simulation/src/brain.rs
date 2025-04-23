use lib_neural_network::{matrix_network::MatrixNetwork, LayerTopology};

use crate::*;

#[derive(Debug)]
pub struct Brain {
    pub(crate) nn: nn::Network,
}

#[allow(unused)]
impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }

    pub fn from_config(rng: &mut dyn RngCore, eye: &Eye, config: Config) -> Self {
        let mut top: Vec<LayerTopology> = Vec::new();
        top.push(LayerTopology {
            neurons: config.num_eye_cells,
        });

        for _ in 0..config.num_hidden_layers {
            top.push(LayerTopology {
                neurons: config.hidden_layer_size,
            })
        }

        top.push(LayerTopology{
            neurons: 2
        });

        Self {
            nn: nn::Network::random(rng, top.as_slice())
        }
    }


    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(
        chromosome: Chromosome,
        eye: &Eye
    ) -> Self {
        Self {
            nn: nn::Network::from_weights(
                &Self::topology(eye), 
            chromosome,)
        }
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2
            },
        ]
    }

}

#[derive(Debug)]
pub struct MatrixBrain {
    pub(crate) nn: mn::MatrixNetwork,
}

impl MatrixBrain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: mn::MatrixNetwork::random(rng, &Self::topology(eye)),
        }
    }

    pub fn from_config(rng: &mut dyn RngCore, config: Config) -> Self {
        let mut top: Vec<LayerTopology> = Vec::new();
        top.push(LayerTopology {
            neurons: config.num_eye_cells,
        });

        for _ in 0..config.num_hidden_layers {
            top.push(LayerTopology {
                neurons: config.hidden_layer_size,
            })
        }

        top.push(LayerTopology{
            neurons: 2
        });

        MatrixBrain {
            nn: MatrixNetwork::random(rng, top.as_slice())
        }
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(
        chromosome: Chromosome,
        eye: &Eye
    ) -> Self {
        Self {
            nn: mn::MatrixNetwork::from_weights(
                &Self::topology(eye), 
            chromosome,)
        }
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2
            },
        ]
    }
}