use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Bad input, no presents for you");

    println!("With text:\n{input}");
}