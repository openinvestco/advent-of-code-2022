use std::fs;

fn main() {
    // part_one();
    part_two();
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
        
        let first_contains_second = t[0][0] <= t[1][0] && t[0][1] >= t[1][1];
        let second_contains_first= t[0][0] >= t[1][0] && t[0][1] <= t[1][1];
        if first_contains_second || second_contains_first {
            // Sanity check
            // println!("{pair}");
            fully_contained += 1
        }

    }

    println!("Total: {fully_contained}");
}

fn part_two() {
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    let pairs = input.split("\n");
    let mut partially_contained = 0;
    for pair in pairs {
        let t: Vec<Vec<i32>> = pair.split(",")
            .map(|x: &str| x.split("-")
                .map(|x: &str| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
            )
            .collect();
        
        // Check for overlaps
        let t0_contains_t1_low = t[0][0] <= t[1][0] && t[0][1] >= t[1][0];
        let t0_contains_t1_high = t[0][0] <= t[1][1] && t[0][1] >= t[1][1];
        let t1_contains_t0_low = t[1][0] <= t[0][0] && t[1][1] >= t[0][0];
        let t1_contains_t0_high = t[1][0] <= t[0][1] && t[1][1] >= t[0][1];
        
        if t0_contains_t1_low || t0_contains_t1_high || t1_contains_t0_low || t1_contains_t0_high {
            // Sanity check
            println!("{pair}");
            partially_contained += 1
        }

    }

    println!("Total: {partially_contained}");

}