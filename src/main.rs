mod network; // Ensure this is imported if needed
mod byte_handling;

use wgpu::util::DeviceExt;
use futures::executor::block_on;


fn main() {

    let structure: [usize; 3]  = [4, 4, 2]; // Example structure for the neural network
    let inputs: Vec<f32> = vec![0.0, 1.0, 2.0, 3.0]; // Example inputs

    let mut n: network::NeuralNetwork = network::NeuralNetwork::new(&structure);
    println!("Initial network: {:?}", n.weights);


    let instance = wgpu::Instance::default();
    let adapter = block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) = block_on(adapter.request_device(&Default::default())).unwrap();
    

//BIND GROUP LAYOUT
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
        label: Some("Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ], 
    });






//BUFFERS


    let structure_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Structure Buffer"),
        contents: byte_handling::vector_to_bytes(&structure.iter().map(|&x| x as f32).collect::<Vec<f32>>()),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
    });

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Buffer"),
        contents: byte_handling::vector_to_bytes(&inputs),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
    });

    let network_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Network Buffer"),
        contents: byte_handling::vector_to_bytes(&n.weights),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
    });


    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Network Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: structure_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: network_buffer.as_entire_binding(),
            },
        ]
     });


    // What the fuck is this
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Network Command Encoder"),
    });

}