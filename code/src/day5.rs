pub mod day5 {
    pub mod part_1 {
        use std::{cmp, char, cell::RefCell, borrow::{Borrow, BorrowMut}};
        use regex::Regex;

        pub fn grabBox(line: Vec<char>, idx: usize, row: &mut Vec<char>) {
            // grab the next 3 chars
            if idx+2 >= line.len() {
                return;
            }

            let firstChar: char = *line.get(idx).unwrap();
            let secondChar: char = *line.get(idx+1).unwrap();
            let thirdChar: char = *line.get(idx+2).unwrap();
            if firstChar == '[' && secondChar != ' ' && thirdChar == ']' {
                row.push(secondChar);
            } else {
                row.push(' ');
            }
            grabBox(line, idx+4, row);
        }

        pub fn storeBlocks(line: Vec<char>) -> Vec<char> {
            let row: &mut Vec<char> = &mut Vec::new();
            grabBox(line, 0, row);
            return row.to_vec();
        }

        pub fn moveBlocks(numTimes: u32, from: &RefCell<Vec<char>>, to: &RefCell<Vec<char>>) {
            for _ in 0..numTimes {
                let charPopped = from.borrow_mut().pop().unwrap();
                to.borrow_mut().push(charPopped);
            }
        }

        pub fn constructStacks(blocks: &mut Vec<RefCell<Vec<char>>>, stacks: &mut Vec<RefCell<Vec<char>>>) {
            let mut numStacksToCreate = blocks.len();
            for row in blocks.iter() {
                numStacksToCreate = cmp::max(numStacksToCreate, row.borrow().len());
            }
            
            // Create each stack
            for num in 0..numStacksToCreate {
                stacks.push(RefCell::new(Vec::new()));
            }

            //for num in 0..blocks.len() {
            for num in 0..blocks.len() {
                let row = blocks.get(num).unwrap().borrow();
                for idx in 0..row.len() {
                    let c = *row.get(idx).unwrap();
                    if c != ' ' {
                        let mut curStack = stacks.get(idx).borrow_mut().unwrap().borrow_mut();
                        curStack.push(c);
                    }
                }
            }
            for idx in 0..numStacksToCreate {
                let mut curStack = stacks.get(idx).borrow_mut().unwrap().borrow_mut();
                curStack.reverse();
            }
        }

        pub fn run() {
            // rowRegexp: Skip the row label ( 1 2 3 4 5 etc. )
            let rowRegexp: Regex  = Regex::new(r"^(\s*\[\w+\].*)$").unwrap();
            let moveRegexp:Regex  = Regex::new(r"^(move [0-9]+ from [0-9]+ to [0-9]+)$").unwrap();
            let rowLabelRegexp:Regex  = Regex::new(r"^(\s+[0-9]+\s+[0-9]+.*)$").unwrap();
            let blocks:&mut Vec<RefCell<Vec<char>>> = &mut Vec::new();
            let stacks:&mut Vec<RefCell<Vec<char>>> = &mut Vec::new();
            if let Ok(lines) = crate::read_lines("data/day5/input2.txt") {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.len() > 0 {
                            let rowInput = format!("{}", ip);
                            if rowRegexp.is_match(&*rowInput) {
                                let row: Vec<char> = rowInput.chars().collect();
                                let expectedRow: Vec<char> = storeBlocks(row);
                                blocks.push(RefCell::new(expectedRow));
                                //blocks.push(storeBlocks(row));
                            } else if moveRegexp.is_match(&*rowInput) {
                                // Execute move list
                                // grab the number of times
                                let words:Vec<&str> = rowInput.split(" ").collect();
                                let numTimes:u32 = words[1].parse::<u32>().unwrap();
                                let fromIdx:usize = usize::try_from(words[3].parse::<u32>().unwrap()-1).unwrap();
                                let toIdx:usize = usize::try_from(words[5].parse::<u32>().unwrap()-1).unwrap();

                                moveBlocks(numTimes, stacks.get(fromIdx).unwrap(), stacks.get(toIdx).unwrap());
                            } else if rowLabelRegexp.is_match(&*rowInput) {
                                // construct the stacks
                                constructStacks(blocks, stacks);
                            }
                        }
                    }
                }
                let mut result: String = String::new();
                for rowIdx in 0..stacks.len() {
                    let row = stacks.get(rowIdx).unwrap();
                    let lastItemIdx = row.borrow().len()-1;
                    result.push(*row.borrow().get(lastItemIdx).unwrap());
                }
                println!("Result: {}", result);
            }
        }
    }
    pub mod part_2 {
        use std::{cmp, char, cell::RefCell, borrow::{Borrow, BorrowMut}};
        use regex::Regex;

        pub fn moveBlocks(numTimes: u32, from: &RefCell<Vec<char>>, to: &RefCell<Vec<char>>) {
            let mut charsToMove: String = String::new();
            for _ in 0..numTimes {
                let charPopped = from.borrow_mut().pop().unwrap();
                charsToMove.push(charPopped);
            }

            for ch in charsToMove.chars().rev() {
                to.borrow_mut().push(ch);
            }
        }
        pub fn run() {
            // rowRegexp: Skip the row label ( 1 2 3 4 5 etc. )
            let rowRegexp: Regex  = Regex::new(r"^(\s*\[\w+\].*)$").unwrap();
            let moveRegexp:Regex  = Regex::new(r"^(move [0-9]+ from [0-9]+ to [0-9]+)$").unwrap();
            let rowLabelRegexp:Regex  = Regex::new(r"^(\s+[0-9]+\s+[0-9]+.*)$").unwrap();
            let blocks:&mut Vec<RefCell<Vec<char>>> = &mut Vec::new();
            let stacks:&mut Vec<RefCell<Vec<char>>> = &mut Vec::new();
            if let Ok(lines) = crate::read_lines("data/day5/input2.txt") {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.len() > 0 {
                            let rowInput = format!("{}", ip);
                            if rowRegexp.is_match(&*rowInput) {
                                let row: Vec<char> = rowInput.chars().collect();
                                let expectedRow: Vec<char> = crate::day5::day5::part_1::storeBlocks(row);
                                blocks.push(RefCell::new(expectedRow));
                                //blocks.push(storeBlocks(row));
                            } else if moveRegexp.is_match(&*rowInput) {
                                // Execute move list
                                // grab the number of times
                                let words:Vec<&str> = rowInput.split(" ").collect();
                                let numTimes:u32 = words[1].parse::<u32>().unwrap();
                                let fromIdx:usize = usize::try_from(words[3].parse::<u32>().unwrap()-1).unwrap();
                                let toIdx:usize = usize::try_from(words[5].parse::<u32>().unwrap()-1).unwrap();

                                moveBlocks(numTimes, stacks.get(fromIdx).unwrap(), stacks.get(toIdx).unwrap());
                            } else if rowLabelRegexp.is_match(&*rowInput) {
                                // construct the stacks
                                crate::day5::day5::part_1::constructStacks(blocks, stacks);
                            }
                        }
                    }
                }
                let mut result: String = String::new();
                for rowIdx in 0..stacks.len() {
                    let row = stacks.get(rowIdx).unwrap();
                    let lastItemIdx = row.borrow().len()-1;
                    result.push(*row.borrow().get(lastItemIdx).unwrap());
                }
                println!("Result: {}", result);
            }
        }
    }
}