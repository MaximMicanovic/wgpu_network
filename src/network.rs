
use rand::Rng; // Add this at the top


// ...existing code...

fn random_between_minus_one_and_one() -> f32 {
    let mut rng = rand::rng();
    rng.random_range(-1.0..=1.0)
}

pub struct NeuralNetwork {
    pub data: Vec<f32>,
    pub output: Vec<f32>,
}

impl NeuralNetwork {
    pub fn new(structure: &[usize]) -> Self {
        let mut data = Vec::new();
        let mut output = Vec::new();

        for layer in 1..structure.len() {
            
            //Weights
            for _perceptron in 1..structure[layer] {
                for _weight in 0..structure[layer - 1] {
                    data.push(random_between_minus_one_and_one());
                }
            }
            //Bias once per perceptron
            data.push(random_between_minus_one_and_one())
        }

        return NeuralNetwork {
            data: data,
            output: output,
        }
    }
}