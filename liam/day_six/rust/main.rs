use std::fs;

fn main() {
    part_one();
    part_two();
}

fn solution(n_distinct: usize) {
    let input = fs::read_to_string("../input.txt")
    .expect("Bad input, no presents for you");

    for stream in input.split("\n") {
        for i in 0..stream.len()-n_distinct{
            let mut substream = stream[i..i+n_distinct].chars().collect::<Vec<char>>();
            substream.sort();
            substream.dedup();
            if substream.len() == n_distinct {
                println!("{stream}: {}", i+n_distinct);
                break;
            }
        }
    }
}

fn part_one() {
    solution(4);
}

fn part_two() {
    solution(14);
}