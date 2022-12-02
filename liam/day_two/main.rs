use std::fs;

fn main() {
    part_two()
}

fn part_one() {
    let input = fs::read_to_string("input.txt")
        .expect("Bad input, no presents for you");

    let games = input.split("\n");

    let mut total_score = 0;
    for game in games {
        let p:Vec<i32> = game.split(" ").map(|v| match v { "A"|"X" => 1, "B"|"Y" => 2, _ => 3}).collect();
        let score = match p[0] - p[1] {
            0 => p[1] + 3,
            -1|2 => p[1] + 6,
            _ => p[1] 
        };
        total_score += score;
        // Sanity Check
        // println!("P1: {}, P2: {}, Score: {}", p[0], p[1], score);
        
    }
    println!("Total Score: {total_score}");
}

