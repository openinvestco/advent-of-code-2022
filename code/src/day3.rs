pub mod day3 {
    pub mod part_1 {
        use std::{collections::HashSet, char};
        pub fn getPriority(item: &char) -> u32 {
            if *item >= 'a' && *item <= 'z' {
                return *item as u32 - 'a' as u32 + 1;
            }
            if *item >= 'A' && *item <= 'Z' {
                return *item as u32 - 'A' as u32 + 27;
            }
            return 0;
        }

        pub fn run() {
                let mut totalSum: u32 = 0;
                if let Ok(lines) = crate::read_lines("data/day3/input2.txt") {
                    for line in lines {
                        if let Ok(ip) = line {
                            if ip.len() > 0 {
                                let rucksack = format!("{}", ip);
                                let rucksackItems:Vec<char> = rucksack.chars().collect();
                                

                                let firstHalfSet: &mut HashSet<char> = &mut HashSet::new();
                                let secondHalfSet: &mut HashSet<char> = &mut HashSet::new();

                                for idx in 0..rucksackItems.len()/2 {
                                    firstHalfSet.insert(*rucksackItems.get(idx).unwrap());
                                }

                                for idx in rucksackItems.len()/2..rucksackItems.len() {
                                    secondHalfSet.insert(*rucksackItems.get(idx).unwrap());
                                }
                                
                                
                                for item in firstHalfSet.iter() {
                                    if secondHalfSet.contains(item) {
                                        totalSum += getPriority(item);
                                    }
                                }
                            }
                        }
                    }
                    println!("Total Sum: {}", totalSum);
                }
        }
    }
    pub mod part_2 {
        use std::{collections::HashSet, char};

        pub fn run() {
                let mut totalSum: u32 = 0;
                let firstSet: &mut HashSet<char> = &mut HashSet::new();
                let secondSet: &mut HashSet<char> = &mut HashSet::new();
                let thirdSet: &mut HashSet<char> = &mut HashSet::new();
                let mut lineNum: u32 = 1;
                if let Ok(lines) = crate::read_lines("data/day3/input2.txt") {
                    for line in lines {
                        if let Ok(ip) = line {
                            if ip.len() > 0 {
                                let rucksack = format!("{}", ip);
                                let rucksackItems:Vec<char> = rucksack.chars().collect();
                                match lineNum % 3 {
                                    0 => {
                                        for idx in 0..rucksackItems.len() {
                                            thirdSet.insert(*rucksackItems.get(idx).unwrap());
                                        }
                                        if firstSet.len() > 0 {
                                            for item in firstSet.iter() {
                                                if secondSet.contains(item) && thirdSet.contains(item) {
                                                    totalSum += crate::day3::day3::part_1::getPriority(item);
                                                }
                                            }
                                            firstSet.clear();
                                            secondSet.clear();
                                            thirdSet.clear();
                                        }
                                    },
                                    1 => {
                                        for idx in 0..rucksackItems.len() {
                                            firstSet.insert(*rucksackItems.get(idx).unwrap());
                                        }
                                        
                                    },
                                    2 => {
                                        for idx in 0..rucksackItems.len() {
                                            secondSet.insert(*rucksackItems.get(idx).unwrap());
                                        }
                                    },
                                    _ => {
                                        println!("That's unexpected.");
                                    }
                                }
                                lineNum += 1;
                            }
                        }
                    }

                    println!("Total Sum: {}", totalSum);
                }
        }
    }
}