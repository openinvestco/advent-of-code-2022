use std::fs;



fn main() {
    part_one();
}

fn part_one() {
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    let rucksacks = input.split("\n");
        
    let mut total = 0;
    for rucksack in rucksacks {
        
        let size = rucksack.chars().count() / 2;
        let r1: Vec<char> = rucksack[..size].chars().collect();
        let r2 = &rucksack[size..];
        let shared = r1.into_iter().filter(|&c| r2.contains(c)).collect::<Vec<_>>()[0];
        let priority = if shared as u32 <= 90 {shared as u32 - 64 + 26} else {shared as u32 - 96};
        total += priority;
    }
    println!("Total {total}\n",);
}

