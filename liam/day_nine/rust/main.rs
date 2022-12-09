use std::fs;
use std::collections::HashSet;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    // Basically, realized you don't need to track the vector, just positions + math
    let mut h = (0, 4);
    let mut h_1 = (0, 4);
    let mut t = (0, 4);
    let mut t_pos = HashSet::new();

    let r = (1, 0);
    let l = (-1, 0);
    let u = (0, -1);
    let d = (0, 1);

    for mv in input.split("\n") {
        let v: Vec<&str> = mv.split_whitespace().collect();
        let dir = v[0];
        let x = v[1].parse::<u32>().unwrap();
        for _ in 0..x {
            h_1 = h;
            match dir {
                "R" => h = add_tuples(h, r),
                "L" => h = add_tuples(h, l),
                "U" => h = add_tuples(h, u),
                "D" => h = add_tuples(h, d),
                _ => println!("Invalid move")
            }

            if !((h.0 - t.0).abs() <= 1 && (h.1 - t.1).abs() <= 1) {
                t = h_1;
            }
            t_pos.insert(t);
            
            // Sanity checker
            println!("h: {:?}, t: {:?}", &h, &t);
        }
    }
    println!("{:?} - {}", t_pos, t_pos.len());
}

fn part_two() {
    let input = fs::read_to_string("../input.txt")
        .expect("Bad input, no presents for you");

    // Switched the starting position to (0, 0) - makes things easier
    // Start should probably not be 0, 0, but should be some sufficiently large number
    let start = (12, 5);
    let mut rope: Vec< (i32, i32)> = vec![start; 10];
    let mut t_pos = HashSet::<(i32, i32)>::new();
    t_pos.insert(start);

    let r = (1, 0);
    let l = (-1, 0);
    let u = (0, 1);
    let d = (0, -1);

    for mv in input.split("\n") {
        let v: Vec<&str> = mv.split_whitespace().collect();
        let dir = v[0];
        let x = v[1].parse::<u32>().unwrap();
        for _ in 0..x {
            // Move head (part of rope)
            match dir {
                "R" => rope[0] = add_tuples(rope[0], r),
                "L" => rope[0] = add_tuples(rope[0], l),
                "U" => rope[0] = add_tuples(rope[0], u),
                "D" => rope[0] = add_tuples(rope[0], d),
                _ => println!("Invalid move")
            }
            
            // Compare head to tail, update tail as required
            // If last element of tail moves, that's a tail position
            for i in 0..rope.len()-1 {
                let chain = rope[i];
                let chain_1 = rope[i+1];
                let chain_diff = (chain.0 - chain_1.0, chain.1 - chain_1.1);
                if chain_diff.0.abs() > 1 || chain_diff.1.abs() > 1 {
                    // If movement required, move by a vector proportionate to the size
                    let chain_shift = (chain_diff.0.signum(), chain_diff.1.signum());
                    rope[i+1] = add_tuples(rope[i+1], chain_shift);
                    if i+1 == 9 {
                        t_pos.insert(rope[i+1]);
                    }
                } 
            }
        }
    }
    println!("{:?} - {}", t_pos, t_pos.len());
}


fn add_tuples(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32) {
    return (t1.0 + t2.0, t1.1 + t2.1);
}


