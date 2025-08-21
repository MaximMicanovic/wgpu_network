//In generall unsafe code

pub fn vector_to_bytes <T> (vector_input: &Vec<T> ) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            vector_input.as_ptr() as *const u8,
            vector_input.len() * std::mem::size_of::<T>(),
        )
    }
}

pub fn byte_to_vector(bytes_input: &[u8]) -> Vec<f32> {
    unsafe {
        std::slice::from_raw_parts(
            bytes_input.as_ptr() as *const f32,
            bytes_input.len() / std::mem::size_of::<f32>(),
        ).to_vec()
    }
}

