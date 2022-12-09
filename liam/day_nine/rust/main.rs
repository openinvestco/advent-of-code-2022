use std::fs;
use std::collections::HashSet;

fn main() {
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
        println!("Move {dir}, {x}");
        for _ in 0..x {
            h_1 = h;
            match dir {
                "R" => h = add_tuples(h, r),
                "L" => h = add_tuples(h, l),
                "U" => h = add_tuples(h, u),
                "D" => h = add_tuples(h, d),
                _ => println!("Invalid move")
            }

            if !tuples_within_one(h, t) {
                t = h_1;
            }
            t_pos.insert(t);
            
            // Sanity checker
            println!("h: {:?}, t: {:?}", &h, &t);
        }
    }
    println!("{:?} - {}", t_pos, t_pos.len());

}

fn add_tuples(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32) {
    return (t1.0 + t2.0, t1.1 + t2.1);
}

fn tuples_within_one(t1: (i32, i32), t2: (i32, i32)) -> bool {
    // println!("{}, {}", (t1.0 - t2.0).abs(), (t1.1 - t2.1).abs());
    return (t1.0 - t2.0).abs() <= 1 && (t1.1 - t2.1).abs() <= 1;
}


fn part_one() {}
fn part_two() {}