pub fn main(terms: [i32; 3]) -> (i32, i32) {
    let term_a: i32 = terms[0];
    let term_b: i32 = terms[1];
    let term_c: i32 = terms[2];

    let delta: i32 = term_b.pow(2) - 4 * term_a * term_c;
    if delta < 0 {
        return (0, 0);
    }

    let root_one = (-term_b + delta.isqrt()) / 2 * term_a;
    let root_two = (-term_b - delta.isqrt()) / 2 * term_a;

    return (root_one, root_two);
}
