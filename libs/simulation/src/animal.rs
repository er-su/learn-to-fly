use crate::*;


#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,

    pub(crate) eye: Eye,
    pub(crate) brain: MatrixBrain,

    pub(crate) satiation: usize,
}

impl Animal {
    fn new(eye: Eye, brain: MatrixBrain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn from_config(rng: &mut dyn RngCore, config: Config) -> Self {
        let eye = Eye::from_config(config);
        let brain = MatrixBrain::from_config(rng, config);

        Self::new(eye, brain, rng)
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = MatrixBrain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(
        chromosome: Chromosome,
        rng: &mut dyn RngCore
    ) -> Self {
        let eye = Eye::default();
        let brain = MatrixBrain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }
}