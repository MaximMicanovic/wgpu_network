mod network; // Ensure this is imported if needed
mod byte_handling;
mod gpu_code;

use futures::executor::block_on;



fn main() {

    // Initialize WGPU
    let instance = wgpu::Instance::default();
    let adapter = block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) = block_on(adapter.request_device(&Default::default())).unwrap();


    let structure: Vec<usize>  = vec![4, 4, 2]; // Example structure for the neural network
    let inputs: Vec<f32> = vec![0.0, 1.0, 2.0, 3.0]; // Example inputs

    let mut n: network::NeuralNetwork = network::NeuralNetwork::new(&structure);

    // What the fuck is this

    



}