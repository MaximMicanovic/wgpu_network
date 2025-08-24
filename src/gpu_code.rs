use wgpu::util::DeviceExt;
use futures::executor::block_on;

use crate::byte_handling;
use crate::network;

fn _create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    // Create bind group layout
    // This layout defines how the buffers will be used in the compute shader
    // It includes the structure, input, and network weights buffers
// BIND GROUP LAYOUT
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

    bind_group_layout
}

fn _create_bind_group(structure_buffer: wgpu::Buffer, input_buffer: wgpu::Buffer, network_buffer: wgpu::Buffer, bind_group_layout: wgpu::BindGroupLayout, device: &wgpu::Device) -> wgpu::BindGroup {
    // Create buffers
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

    bind_group
}

fn _create_buffers(structure: Vec<usize>, inputs: Vec<f32>, n: network::NeuralNetwork, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {

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
        contents: byte_handling::vector_to_bytes(&n.data),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
    });

    (structure_buffer, input_buffer, network_buffer)
}

fn pipeline_layout(bind_group_layout: wgpu::BindGroupLayout, device: &wgpu::Device) -> wgpu::PipelineLayout {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
        label: Some("Pipeline Layout"), 
        bind_group_layouts: &[&bind_group_layout], 
        push_constant_ranges: &[],
    });
    pipeline_layout
}

fn  _create_compute_pipeline(device: &wgpu::Device, pipeline_layout: &wgpu::PipelineLayout) -> wgpu::ComputePipeline {

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Network Command Encoder"),
    });

    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: Some("Compute Pass"),
        timestamp_writes: None,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Network Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(("network_math.wgsl").into()),
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Network Compute Pipeline"),
        layout: Some(pipeline_layout),
        module: &shader,
        cache: None,
        compilation_options: wgpu::PipelineCompilationOptions { constants:Default::default(), zero_initialize_workgroup_memory: true },
        entry_point: Some("main"),
    });

    return compute_pipeline;
}

fn _run_compute_pass(compute_pass: &mut wgpu::ComputePass, bind_group: &wgpu::BindGroup, compute_pipeline: &wgpu::ComputePipeline) {
    compute_pass.set_pipeline(compute_pipeline);
    compute_pass.set_bind_group(0, bind_group, &[]);
    compute_pass.dispatch_workgroups(1, 1, 1);
}


//Temporary functiont to test the code
pub fn run_network(structure: Vec<usize>, inputs: Vec<f32>, n: network::NeuralNetwork, device: &wgpu::Device, queue: &wgpu::Queue) {

    let (structure_buffer, input_buffer, network_buffer) = _create_buffers(structure, inputs, n, device);
    let bind_group = _create_bind_group(structure_buffer, input_buffer, network_buffer, _create_bind_group_layout(device), device);
    let pipeline_layout = pipeline_layout(_create_bind_group_layout(device), device);
    let compute_pipeline = _create_compute_pipeline(device, &pipeline_layout);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Network Command Encoder"),
    });

    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: Some("Compute Pass"),
        timestamp_writes: None,
    });

    _run_compute_pass(&mut compute_pass, &bind_group, &compute_pipeline);
    drop(compute_pass); // End the compute pass
    // Submit the command buffer
    
    queue.submit(Some(encoder.finish()));
}