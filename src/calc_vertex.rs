pub fn main(terms: [f64; 3]) -> (f64, f64) {
    let a = terms[0];
    let b = terms[1];
    let c = terms[2];

    let vertex_x: f64 = -b / (2.0 * a);
    let vertex_y: f64 = (a * (vertex_x * vertex_x)) + (b * vertex_x) + c;
    println!("vertex_x: {}, vertex_y: {}", vertex_x, vertex_y);
    return (vertex_x, vertex_y);
}
