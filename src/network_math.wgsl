// For where it folds instead of just summarizing it
let CUT_OFF_AMOUNT = 10000u;


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
var<storage, read_write> data: Matrix;

@compute @workgroup_size(256)
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

fn neuron_forward(layerOffset: u32, inOffsett: u32, k:u32, M: u32) {
    let gid = @biuildtin(global_invocation_id).x
    if(gid >= M) { return; }


}

//Function that decides sums if below 5000 values and fold if above
fn sum(data: ptr<storage, array<f32>>, length: u32, local_id: u32) -> f32 {
    if(length < CUT_OFF_AMOUNT) {
        return sum_serial(data, length);
    } else {
        return sum_parallel(data, lenght, local_id);
    }
}

fn sum_serial(data ptr<storage, array<f32>>, length) ->  {
    var s: f32 = 0.0;
    for(var i: u32 = 0; i < length; i = i + 1u) {
        s = s + data[i]
    }
    return s;
}


var<workgroup> temp: array<f32, 256>;
fn sum_parallel(data: ptr<storage, array<f32>>, start: u32, length: u32, local_id: u32) -> f32 {
    var stride: u32 = 256u;
    var s: f32 = 0f32;

    for (var i:u32 = local_id; i < length; i = i + stride) {
        s = s + data[i];
    }
    temp[local_id] = s;

    var offset: u32 = stride / 2u;
    while (offset > 0u) {
        if (local_id < offset){
            temp[local_id] = temp[local_id] + temp[local_id + offset]
        }
        workgroupBarrier();
        offset = offset / 2u;
    }
    return temp[0];
}
