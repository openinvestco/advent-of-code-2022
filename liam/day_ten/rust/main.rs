use std::fs;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    let mut cycle : i32 = 1;
    let mut register = 1;
    let mut total_signal = 0;
    for op in input.replace("\n", " ").split_whitespace() {
        if (cycle - 20) % 40 == 0 {
            total_signal += cycle*register;
        }
        let parsed = op.parse::<i32>();
        match parsed {
            Ok(ok) => register += ok,
            _ => ()
        }
        cycle += 1;
    }
    println!("{total_signal}, {cycle}");
}
fn part_two() {

}