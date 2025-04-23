pub fn main(terms: [f64; 3]) -> (f64, f64) {
    let term_a: f64 = terms[0];
    let term_b: f64 = terms[1];
    let term_c: f64 = terms[2];

    let delta: f64 = term_b * term_b - 4.0 * term_a * term_c;
    if delta < 0.0 {
        let complex_roots: String = complex_roots(delta, term_a, term_b);
        println!("Raízes complexas: {}", complex_roots);
        return (0.0, 0.0);
    }

    let root_one = (-term_b + delta.sqrt()) / (2.0 * term_a);
    let root_two = (-term_b - delta.sqrt()) / (2.0 * term_a);

    return (root_one, root_two);
}

fn complex_roots(delta: f64, a: f64, b: f64) -> String {
    let real = format!("{}/{}", -b, (2.0 * a));
    let imag = format!("√{}i/{}", -delta, (2.0 * a));
    format!("{} +- {}", real, imag)
}
