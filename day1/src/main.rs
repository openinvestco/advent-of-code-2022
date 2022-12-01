use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;
use std::cmp;
use std::collections::BinaryHeap;

fn main() {
    let mut elf = Vec::new();
    let mut biggestSum = 0 as u64;
    let mut heap = BinaryHeap::new();
    if let Ok(lines) = read_lines("data/input2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() > 0 {
                    elf.push(ip.parse::<u64>().unwrap());
                    println!("{}", ip);
                } else {
                    // Sum up all of elf
                    let curElfSum = elf.iter().sum();
                    biggestSum = cmp::max(biggestSum, curElfSum);
                    elf.clear();
                    heap.push(curElfSum);
                }
            }
        }
    } 

    // Sum up all of elf
    let curElfSum = elf.iter().sum();
    heap.push(curElfSum);
    biggestSum = cmp::max(biggestSum, curElfSum);
    println!("Biggest number of calories: {}", biggestSum);
    let sumOfTopThreeVals: u64 = [heap.pop().unwrap(), heap.pop().unwrap(), heap.pop().unwrap()].iter().sum(); 
    println!("Biggest three number of calories: {}", sumOfTopThreeVals);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}