struct Structure {
    data: array<usize>
};

struct Matrix {
    data: array<f32>;
};

@group(0) @binding(0)
var<storage, read> network_structure: Structure;

@group(0) @binding(1)
var<storage, read> inputs: Matrix;

@group(0) @binding(2)
var<storage, read_write> network_weight: Matrix;

@compute @workgroup_size(8,8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let row = gid.y;
    let col = gid.x;

    if (row >= rows_a || col >= cols_b) {
        return;
    }

    var sum: f32 = 0.0;
    for (var k: u32 = 0u; k < cols_a; k = k + 1u) {
        let a_index = row * cols_a + k;
        let b_index = k * cols_b + col;
        sum = sum + a.data[a_index] * b.data[b_index];
    }

    let c_index = row * cols_b + col;
    c.data[c_index] = sum;
}
