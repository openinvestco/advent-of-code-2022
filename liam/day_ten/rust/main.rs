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
        // println!("cycle: {cycle}, register: {register}");
        if ((register + 1) - (cycle % 40)).abs() <= 1 {
            crt_output.push_str("#")
        } else {
            crt_output.push_str(".")
        }
        // println!("{crt_output}");
        let parsed = op.parse::<i32>();
        match parsed {
            Ok(ok) => register += ok,
            _ => ()
        }
        cycle += 1;
    }
    let crt_wrapped = crt_output
        .chars()
        .enumerate()
        .fold(String::new(), |acc, (i, c)| {
            if i != 0 && i % 40 == 0 {
                format!("{}\n{}", acc, c)
            } else{
                format!("{}{}", acc, c)
            }
        });
    println!("{crt_wrapped}");

}