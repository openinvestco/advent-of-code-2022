use std::fs;

fn main() {
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
            if (left || right || top || bottom) {
                total += 1
                // println!("{tree} at [{r}, {c}] is visible from left {:?}, from right {:?}, from top {:?}, from bottom {:?}", left, right, top, bottom);
            }
        }
    }
    println!("{total} trees visible");
}

fn part_one() {}
fn part_two() {}