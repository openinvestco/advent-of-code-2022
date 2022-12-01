use std::fs;

fn main() {
    part_one();
}

fn part_one() {
    let input = fs::read_to_string("input.txt")
    .expect("Bad input, no presents for you");

    let calories = input.split("\n");
    let mut elf_no = 0;
    let mut elf_calories = Vec::new();
    for calorie in calories {
        if calorie == "" {
            elf_no += 1
        } else {
            if elf_calories.len() <= elf_no {
                elf_calories.push(0)
            }
            elf_calories[elf_no] += calorie.parse::<i32>().unwrap();
        }
    }

    // Sanity checker
    // for (i, calories) in elf_calories.iter().enumerate() {
    //     println!("Elf {} calories: {calories}", i+1)
    // }

    let max_calorie_elf_index = elf_calories.iter().position(|e| e == elf_calories.iter().max().unwrap()).unwrap();
    println!("Elf {} has the most calories: {}\n", max_calorie_elf_index+1, elf_calories[max_calorie_elf_index]);
}