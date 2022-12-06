pub mod day6 {
    pub mod part_1 {
        use std::{cmp, char, cell::RefCell, collections::{HashMap, LinkedList}, borrow::{Borrow, BorrowMut}};
        use regex::Regex;
        pub fn firstStartOfPacket(chars: Vec<char>) -> u32 {
            let mut hm: HashMap<char, u32> = HashMap::new();
            let mut currentLetters:LinkedList<char>= LinkedList::new();
            let mut letterCount:u32 = 1;
            for c in chars.iter() {
                let x = c.clone();
                currentLetters.push_back(x);
                if hm.contains_key(&c) {
                    let v = hm.get(&c).unwrap();
                    hm.insert(c.clone(), v+1);
                } else {
                    hm.insert(c.clone(), 1);
                }
                letterCount += 1;
                if currentLetters.len() >= 4 { 
                    let mut foundMoreThanOne: bool = false;
                    for cc in &currentLetters {
                        if *hm.get(cc).unwrap() != 1 {
                            foundMoreThanOne = true;
                            break;
                        }
                    }
                    if foundMoreThanOne == false {
                        return letterCount-1;
                    }
                    let deletedChar = currentLetters.pop_front().unwrap();
                    let v = hm.get(&deletedChar).unwrap();
                    hm.insert(deletedChar, v-1);
                }
            }
            return 0;
        }

        pub fn run() {
            if let Ok(lines) = crate::read_lines("data/day6/input2.txt") {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.len() > 0 {
                            let rowInput = format!("{}", ip);
                            let chars = rowInput.chars().collect();
                            println!("First Start of Packet: {}", firstStartOfPacket(chars));
                        }
                    }
                }
            }
        }
    }
    pub mod part_2 {
        use std::{cmp, char, cell::RefCell, collections::{HashMap, LinkedList}, borrow::{Borrow, BorrowMut}};
        use regex::Regex;
        pub fn firstStartOfPacket(chars: Vec<char>) -> u32 {
            let mut hm: HashMap<char, u32> = HashMap::new();
            let mut currentLetters:LinkedList<char>= LinkedList::new();
            let mut letterCount:u32 = 1;
            for c in chars.iter() {
                let x = c.clone();
                currentLetters.push_back(x);
                if hm.contains_key(&c) {
                    let v = hm.get(&c).unwrap();
                    hm.insert(c.clone(), v+1);
                } else {
                    hm.insert(c.clone(), 1);
                }
                letterCount += 1;
                if currentLetters.len() >= 14 { 
                    let mut foundMoreThanOne: bool = false;
                    for cc in &currentLetters {
                        if *hm.get(cc).unwrap() != 1 {
                            foundMoreThanOne = true;
                            break;
                        }
                    }
                    if foundMoreThanOne == false {
                        return letterCount-1;
                    }
                    let deletedChar = currentLetters.pop_front().unwrap();
                    let v = hm.get(&deletedChar).unwrap();
                    hm.insert(deletedChar, v-1);
                }
            }
            return 0;
        }

        pub fn run() {
            if let Ok(lines) = crate::read_lines("data/day6/input2.txt") {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.len() > 0 {
                            let rowInput = format!("{}", ip);
                            let chars = rowInput.chars().collect();
                            println!("First Start of Packet: {}", firstStartOfPacket(chars));
                        }
                    }
                }
            }
        }
    }
}