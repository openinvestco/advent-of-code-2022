pub mod day2 {
    pub mod part_1 {
        use std::collections::HashMap;

        const WIN_POINTS: u64 = 6;
        const DRAW_POINTS: u64 = 3;
        const LOSE_POINTS: u64 = 0;

        pub fn initMaps(m: &mut HashMap<String,u64>, wm: &mut HashMap<String, String>, dm: &mut HashMap<String, String>) {
            // Insert rock, paper, scissors (what you play)
            m.insert(String::from("X"), 1);
            m.insert(String::from("Y"), 2);
            m.insert(String::from("Z"), 3);

            // Insert win conditions
            wm.insert(String::from("A"), String::from("Y"));
            wm.insert(String::from("B"), String::from("Z"));
            wm.insert(String::from("C"), String::from("X"));

            // Insert draw conditions
            dm.insert(String::from("A"), String::from("X"));
            dm.insert(String::from("B"), String::from("Y"));
            dm.insert(String::from("C"), String::from("Z"));
        }

        pub fn didWeWin(theirMove: &String, ourMove: &String, wm: &HashMap<String, String>) -> bool {
            let winningCondition = wm.get(theirMove).unwrap().as_str();
            if ourMove == winningCondition {
                return true;
            }
            return false;
        }

        pub fn didWeDraw(theirMove: &String, ourMove: &String, wm: &HashMap<String, String>) -> bool {
            let drawCondition = wm.get(theirMove).unwrap().as_str();
            if ourMove == drawCondition {
                return true;
            }
            return false;
        }

        pub fn run() {
            let m: &mut HashMap<String, u64> = &mut HashMap::new();
            let wm: &mut HashMap<String, String> = &mut HashMap::new();
            let dm: &mut HashMap<String, String> = &mut HashMap::new();
            let mut runningScore: u64 = 0;
            initMaps(m,wm,dm);
            if let Ok(lines) = crate::read_lines("data/day2/input2.txt") {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.len() > 0 {
                            let round = format!("{}", ip);
                            let roundSplit:Vec<&str> = round.split(" ").collect();

                            let opponentMove = &roundSplit.get(0).unwrap().to_string();
                            let yourMove = &roundSplit.get(1).unwrap().to_string();
                            
                            // Did we win?
                            let win = didWeWin(opponentMove, yourMove, &wm);
                            let draw =  didWeDraw(opponentMove, yourMove, &dm);
                            let lose = (win == false) && (draw == false);
                            
                            if win {
                                runningScore += WIN_POINTS
                            } else if draw {
                                runningScore += DRAW_POINTS;
                            } else if lose {
                                runningScore += LOSE_POINTS;
                            }
                            runningScore += m.get(yourMove).unwrap();
                        }
                    }
                }
                println!("Total Points: {}", runningScore);
            } 
        }
    }
    pub mod part_2 {
        use std::collections::HashMap;

        const WIN_POINTS: u64 = 6;
        const DRAW_POINTS: u64 = 3;
        const LOSE_POINTS: u64 = 0;

        pub fn initMaps(m: &mut HashMap<String,u64>, wm: &mut HashMap<String, String>, dm: &mut HashMap<String, String>, lm: &mut HashMap<String, String>) {
            // Insert rock, paper, scissors (what you play)
            m.insert(String::from("X"), 1);
            m.insert(String::from("Y"), 2);
            m.insert(String::from("Z"), 3);

            // Insert win conditions
            wm.insert(String::from("A"), String::from("Y"));
            wm.insert(String::from("B"), String::from("Z"));
            wm.insert(String::from("C"), String::from("X"));

            // Insert draw conditions
            dm.insert(String::from("A"), String::from("X"));
            dm.insert(String::from("B"), String::from("Y"));
            dm.insert(String::from("C"), String::from("Z"));

            // Insert lose conditions
            lm.insert(String::from("A"), String::from("Z"));
            lm.insert(String::from("B"), String::from("X"));
            lm.insert(String::from("C"), String::from("Y"));
        }

        pub fn didWeWin(theirMove: &String, ourMove: &String, wm: &HashMap<String, String>) -> bool {
            let winningCondition = wm.get(theirMove).unwrap().as_str();
            if ourMove == winningCondition {
                return true;
            }
            return false;
        }

        pub fn didWeDraw(theirMove: &String, ourMove: &String, wm: &HashMap<String, String>) -> bool {
            let drawCondition = wm.get(theirMove).unwrap().as_str();
            if ourMove == drawCondition {
                return true;
            }
            return false;
        }

        pub fn run() {
            let m: &mut HashMap<String, u64> = &mut HashMap::new();
            let wm: &mut HashMap<String, String> = &mut HashMap::new();
            let dm: &mut HashMap<String, String> = &mut HashMap::new();
            let lm: &mut HashMap<String, String> = &mut HashMap::new();
            let mut runningScore: u64 = 0;
            initMaps(m,wm,dm,lm);
            if let Ok(lines) = crate::read_lines("data/day2/input2.txt") {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.len() > 0 {
                            let round = format!("{}", ip);
                            let roundSplit:Vec<&str> = round.split(" ").collect();

                            let opponentMove = &roundSplit.get(0).unwrap().to_string();
                            let howTheRoundEnds= &roundSplit.get(1).unwrap().to_string();

                            // Let's get our move
                            if howTheRoundEnds == "X" {
                                let yourMove = lm.get(opponentMove).unwrap();
                                runningScore += LOSE_POINTS + m.get(yourMove).unwrap();
                            } else if howTheRoundEnds == "Y" {
                                let yourMove = dm.get(opponentMove).unwrap();
                                runningScore += DRAW_POINTS + m.get(yourMove).unwrap();
                            } else if howTheRoundEnds == "Z" {
                                let yourMove = wm.get(opponentMove).unwrap();
                                runningScore += WIN_POINTS + m.get(yourMove).unwrap();
                            }
                        }
                    }
                }
                println!("Total Points: {}", runningScore);
            } 
        }
    }
}