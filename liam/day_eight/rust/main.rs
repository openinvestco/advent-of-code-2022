use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = fs::read_to_string("../input.txt")
    .expect("Bad input, no presents for you");

    let trees: Vec<Vec<u32>> = input
        .split("\n")
        .map(|row| row
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
        )
        .collect();

    let h = trees.len();
    let w = trees[0].len();
    let mut total = (w*2) + (h-2)*2;
    for r in 1..h-1 {
        for c in 1..w-1 {
            let tree = trees[r][c];
            let left = tree > *trees[r][0..c].iter().max().unwrap();
            let right = tree > *trees[r][c+1..w].iter().max().unwrap();
            let top = tree > trees[0..r].iter().map(|t| t[c]).max().unwrap();
            let bottom = tree > trees[r+1..w].iter().map(|t| t[c]).max().unwrap();
            if left || right || top || bottom {
                total += 1
                // println!("{tree} at [{r}, {c}] is visible from left {:?}, from right {:?}, from top {:?}, from bottom {:?}", left, right, top, bottom);
            }
        }
    }
    println!("{total} trees visible");
}
fn part_two() {
    let input = fs::read_to_string("../input.txt")
    .expect("Bad input, no presents for you");

    let trees: Vec<Vec<i32>> = input
        .split("\n")
        .map(|row| row
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect()
        )
        .collect();

    let h = trees.len();
    let w = trees[0].len();
    let mut max_score = 0;
    for r in 0..h {
        for c in 0..w {
            let tree = trees[r][c];
            let left = match trees[r][0..c].into_iter().rev().position(|t| t >= &tree) {
                Some(ind) => ind +1,
                None => trees[r][0..c].len()
            };
            let right = match trees[r][c+1..w].into_iter().position(|t| t >= &tree) {
                Some(ind) => ind +1,
                None => trees[r][c+1..w].len()
            };
            let up = match trees[0..r].iter().map(|t| t[c]).into_iter().rev().position(|t| t >= tree) {
                Some(ind) => ind +1,
                None => trees[0..r].len()
            };
            let down = match trees[r+1..w].iter().map(|t| t[c]).into_iter().position(|t| t >= tree) {
                Some(ind) => ind +1,
                None => trees[r+1..w].len()
            };
            let score = up*left*right*down;
            if score > max_score {
                max_score = score;
            }
            // println!("{tree} up: {up}, left: {left}, right: {right}, down: {down}, score: {max_score}")

        }
    }
    println!("Highest scenic score: {max_score}");
}