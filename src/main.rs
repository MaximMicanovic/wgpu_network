mod network; // Ensure this is imported if needed
use wgpu::util::DeviceExt;
use futures::executor::block_on;
use bytemuck::cast_slice;



fn main() {

    let mut n = network::NeuralNetwork::new(&[2,2]);
    println!("Initial network: {:?}", n.weights);


    let instance = wgpu::Instance::default();
    let adapter = block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) = block_on(adapter.request_device(&Default::default())).unwrap();
    

    let buffer_size = (n.flat().len() * std::mem::size_of::<f32>()) as u64;
    
    let network_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some("Network Buffer"),
        contents: bytemuck::cast_slice(&n.flat().as_slice()),
        usage: wgpu::BufferUsages::STORAGE |wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
    });

    let read_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Read Buffer"),
        size: network_buffer.size(),


        //MAP_READ is for reading back data from the GPU to the CPU
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,

        //means its empty at creation
        mapped_at_creation: false,
    });


    //(Staging buffer) is used to copy data from the GPU to the CPU
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Copy Encoder"),
    });



    encoder.copy_buffer_to_buffer(&network_buffer, 0, &read_buffer, 0, buffer_size);
    queue.submit(Some(encoder.finish()));

    // Wait for the GPU to finish the copy operation
    {
        let buffer_slice = read_buffer.slice(..);
        let (sender, receiver) = futures::channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        
        //Wait for the gpu to finish the operation
        instance.poll_all(true);

        // Wait for the map_async to complete
        block_on(receiver).unwrap().unwrap();

        // Get the mapped data
        let data = buffer_slice.get_mapped_range();
        let result: &[f32] = bytemuck::cast_slice(&data);

        println!("Network buffer contents: {:?}", result);

        // Unmap the buffer
        read_buffer.unmap();
    }


    // I should write in a then way to make children on the GPU


}