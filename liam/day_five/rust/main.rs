use std::fs;


// [V]         [T]         [J]        
// [Q]         [M] [P]     [Q]     [J]
// [W] [B]     [N] [Q]     [C]     [T]
// [M] [C]     [F] [N]     [G] [W] [G]
// [B] [W] [J] [H] [L]     [R] [B] [C]
// [N] [R] [R] [W] [W] [W] [D] [N] [F]
// [Z] [Z] [Q] [S] [F] [P] [B] [Q] [L]
// [C] [H] [F] [Z] [G] [L] [V] [Z] [H]
//  1   2   3   4   5   6   7   8   9 

fn main() {
    part_two();
}

fn part_one() {
    let mut stacks = vec![
        vec!["C", "Z", "N", "B", "M", "W", "Q", "V"], 
        vec!["H", "Z", "R", "W", "C", "B"], 
        vec!["F", "Q", "R", "J"], 
        vec!["Z", "S", "W", "H", "F", "N", "M", "T"], 
        vec!["G", "F", "W", "L", "N", "Q", "P"], 
        vec!["L", "P", "W"], 
        vec!["V", "B", "D", "R", "G", "C", "Q", "J"],
        vec!["Z", "Q", "N", "B", "W"],
        vec!["H", "L", "F", "C", "G", "T", "J"],
    ];

    println!("{:?}", stacks);
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    let moves = input.split("\n");
    for _move in moves {
        let move_set= _move.split_whitespace().collect::<Vec<&str>>();
        let count = move_set[1].parse::<i32>().unwrap();
        let v1 = move_set[3].parse::<usize>().unwrap() -1 ;
        let v2 = move_set[5].parse::<usize>().unwrap() -1;
        for n in 0..count {
            let buf = stacks[v1].pop().unwrap();
            stacks[v2].push(buf);
        }
    }
    for stack in stacks {
        println!("{}", stack.last().unwrap())
    }
    
}
fn part_two() {
    let mut stacks = vec![
        vec!["C", "Z", "N", "B", "M", "W", "Q", "V"], 
        vec!["H", "Z", "R", "W", "C", "B"], 
        vec!["F", "Q", "R", "J"], 
        vec!["Z", "S", "W", "H", "F", "N", "M", "T"], 
        vec!["G", "F", "W", "L", "N", "Q", "P"], 
        vec!["L", "P", "W"], 
        vec!["V", "B", "D", "R", "G", "C", "Q", "J"],
        vec!["Z", "Q", "N", "B", "W"],
        vec!["H", "L", "F", "C", "G", "T", "J"],
    ];

    println!("{:?}", stacks);
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    let moves = input.split("\n");
    for _move in moves {
        let move_set= _move.split_whitespace().collect::<Vec<&str>>();
        let count = move_set[1].parse::<usize>().unwrap();
        let v1 = move_set[3].parse::<usize>().unwrap() -1 ;
        let v2 = move_set[5].parse::<usize>().unwrap() -1;
        let v1_pos = stacks[v1].len() - count;
        let mut buf: Vec<&str> = stacks[v1].drain(v1_pos..).collect();
        stacks[v2].append(&mut buf);
        println!("{:?}", stacks);
    }
    for stack in stacks {
        println!("{}", stack.last().unwrap())
    }
}