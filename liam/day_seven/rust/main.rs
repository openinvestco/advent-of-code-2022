use std::fs;
use std::collections::HashMap;

fn main() {
    part_one();
    part_two();
}

fn file_system_iterator(input: &str) -> HashMap<String, usize> {
    let input = fs::read_to_string(input)
        .expect("Bad input, no presents for you");

    let mut path: Vec<&str> = Vec::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    for tout in input.split("\n") {
        if tout.starts_with("$") {
            if tout.starts_with("$ cd") {
                let dir = tout.strip_prefix("$ cd ").unwrap();
                if dir == ".." {
                    path.pop();
                } else {
                    path.push(dir);
                }

            }
        } else {
            if !tout.starts_with("dir") {
                for i in 1..path.len()+1 {
                    let location = path[0..i].join("/");
                    let n: usize = tout.split_whitespace().collect::<Vec<&str>>()[0].parse().unwrap();
                    dir_sizes.entry(location).and_modify(|count| *count += n).or_insert(n);
                }
            }
        }
    }
    return dir_sizes
}

fn part_one() {
    let dir_sizes = file_system_iterator("../input.txt");
    println!("{}", dir_sizes.values().cloned().filter(|count| count <= &100000).sum::<usize>());
}

fn part_two() {
    let dir_sizes = file_system_iterator("../input.txt");
    let used_space = dir_sizes.get("/").unwrap();
    let space_to_free = used_space - 40000000;
    println!("{}", dir_sizes.values().cloned().filter(|count| count >= &space_to_free).min().unwrap());
}