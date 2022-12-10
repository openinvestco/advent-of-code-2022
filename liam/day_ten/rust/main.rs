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
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    let mut cycle : i32 = 1;
    let mut register = 1;
    let mut total_signal = 0;
    let mut crt_output: String = "".to_owned();
    for op in input.replace("\n", " ").split_whitespace() {
        if ((register + 1) - (cycle % 40)).abs() <= 1 {
            crt_output.push_str("#")
        } else {
            crt_output.push_str(" ")
        }
        if cycle < 240 && cycle % 40 == 0 {
            crt_output.push_str("\n")
        }
        let parsed = op.parse::<i32>();
        match parsed {
            Ok(ok) => register += ok,
            _ => ()
        }
        cycle += 1;
    }
    println!("{crt_output}");

}