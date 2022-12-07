use std::fs;
use std::collections::HashMap;

fn main() {
    part_one();
}

fn part_one() {
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    let mut position: Vec<&str> = Vec::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    for tout in input.split("\n") {
        if tout.starts_with("$") {
            if tout.starts_with("$ cd") {
                let dir = tout.strip_prefix("$ cd ").unwrap();
                if dir == ".." {
                    position.pop();
                } else {
                    position.push(dir);
                }

            }
        } else {
            if !tout.starts_with("dir") {
                for i in 1..position.len()+1 {
                    let location = position[0..i].join("/");
                    let n: usize = tout.split_whitespace().collect::<Vec<&str>>()[0].parse().unwrap();
                    dir_sizes.entry(location).and_modify(|count| *count += n).or_insert(n);
                }
            }
        }
    }
    println!("{}", dir_sizes.values().cloned().filter(|count| count <= &100000).sum::<usize>());
}

fn part_two() {}