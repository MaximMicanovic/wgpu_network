
use rand::Rng; // Add this at the top


// ...existing code...

fn random_between_minus_one_and_one() -> f32 {
    let mut rng = rand::rng();
    rng.random_range(-1.0..=1.0)
}

pub struct NeuralNetwork {
    pub weights: Vec<Vec<Vec<f32>>>,
    pub biases: Vec<Vec<f32>>,
    pub output: Vec<Vec<f32>>,
}

//This is vibe coded damn is it cool
impl NeuralNetwork {
    pub fn new(layers: &[usize]) -> Self {
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        let mut output = Vec::new();

        for i in 0..layers.len() - 1 {
            let weight_matrix = (0..layers[i + 1])
                .map(|_| (0..layers[i]).map(|_| random_between_minus_one_and_one()).collect())
                .collect();
            weights.push(weight_matrix);

            let bias_vector = (0..layers[i + 1]).map(|_| random_between_minus_one_and_one()).collect();
            biases.push(bias_vector);

            output.push(vec![0.0; layers[i + 1]]);
        }

        NeuralNetwork { weights, biases, output }
    }

    pub fn flat(&self) -> Vec<f32> {
        let mut flat_weights = Vec::new();
        for weight_matrix in &self.weights {
            for row in weight_matrix {
                flat_weights.extend(row.iter().cloned());
            }
        }
        flat_weights.extend(self.biases.iter().flat_map(|b| b.iter().cloned()));
        flat_weights
    }

}