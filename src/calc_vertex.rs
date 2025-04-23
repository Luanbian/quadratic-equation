pub fn main(terms: [i32; 3]) -> (i32, i32) {
    let a = terms[0];
    let b = terms[1];
    let c = terms[2];

    let vertex_x: i32 = -b / 2 * a;
    let vertex_y: i32 = vertex_x.pow(2) + b * vertex_x + c;
    return (vertex_x, vertex_y);
}
