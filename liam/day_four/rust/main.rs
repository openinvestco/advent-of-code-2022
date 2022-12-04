use std::fs;

fn main() {
    part_one();
}

fn part_one() {
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    let pairs = input.split("\n");
    let mut fully_contained = 0;
    for pair in pairs {
        let t: Vec<Vec<i32>> = pair.split(",")
            .map(|x: &str| x.split("-")
                .map(|x: &str| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
            )
            .collect();
        if (t[0][0] >= t[1][0] && t[0][1] <= t[1][1]) || (t[0][0] <= t[1][0] && t[0][1] >= t[1][1]) {
            // Sanity check
            println!("{pair}");
            fully_contained += 1
        }

    }

    println!("Total: {fully_contained}");
}

fn part_two() {

    
}