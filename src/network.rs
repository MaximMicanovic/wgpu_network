
use rand::Rng; // Add this at the top


// ...existing code...

fn random_between_minus_one_and_one() -> f32 {
    let mut rng = rand::rng();
    rng.random_range(-1.0..=1.0)
}

pub struct NeuralNetwork {
    pub weights: Vec<f32>,
    pub biases: Vec<f32>,
    pub output: Vec<f32>,
}


impl NeuralNetwork {
    pub fn new(structure: &[usize]) -> Self {
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        let mut output = Vec::new();

        for i in 0..structure.len() - 1 {
            for _ in 0..structure[i] * structure[i + 1] {
                weights.push(random_between_minus_one_and_one());
            }
            for _ in 0..structure[i + 1] {
                biases.push(random_between_minus_one_and_one());
            }
        }

        NeuralNetwork {
            weights,
            biases,
            output,
        }
    }
}