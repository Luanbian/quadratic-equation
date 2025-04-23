use std::io;
mod calc_root;
mod calc_vertex;
mod render;

fn main() {
    let mut terms: [i32; 3] = [0, 0, 0];

    let mut counter: u32 = 0;
    loop {
        println!("Digite o termo {}", ('A' as u8 + counter as u8) as char);

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler o input");

        let converted_number: i32 = input
            .trim()
            .parse()
            .expect("Por favor, insira um numero válido");

        terms[counter as usize] = converted_number;

        counter += 1;
        if counter == terms.len().try_into().unwrap() {
            terms.iter().enumerate().for_each(|(index, element)| {
                println!("input {}: {}", ('A' as u8 + index as u8) as char, element);
            });
            break;
        }
    }

    let roots: (i32, i32) = calc_root::main(terms);
    let vertex: (i32, i32) = calc_vertex::main(terms);

    match render::run(terms[0], terms[1], terms[2], roots, vertex) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
