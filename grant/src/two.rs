#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid input!"),
        }
    }
}

impl Move {
    pub fn points(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Result {
    Win,
    Draw,
    Lose,
}

impl From<&str> for Result {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid input!"),
        }
    }
}

pub fn play_round(you: Move, opponent: Move) -> usize {
    you.points() + match (you, opponent) {
        (Move::Rock, Move::Paper) => 0,
        (Move::Rock, Move::Scissors) => 6,
        (Move::Paper, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 6,
        (Move::Scissors, Move::Rock) => 0,
        (Move::Scissors, Move::Paper) => 6,
        // draw always == 3
        _ => 3
    }
}

pub fn guess_move(opponent: Move, result: Result) -> usize {
    let your_move = if result == Result::Draw { opponent } else {
        match (opponent, result) {
            (Move::Rock, Result::Lose) => Move::Scissors,
            (Move::Rock, Result::Win) => Move::Paper,
            (Move::Paper, Result::Lose) => Move::Rock,
            (Move::Paper, Result::Win) => Move::Scissors,
            (Move::Scissors, Result::Lose) => Move::Paper,
            (Move::Scissors, Result::Win) => Move::Rock,
            _ => panic!("oops")
        }
    };

    play_round(your_move, opponent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_sample() {
       let input = "A Y
B X
C Z";
        let mut points = 0;
        for line in input.lines() {
            let mut split = line.split(" ").into_iter();
            let you = Move::from(split.next().unwrap());
            let opponent = Move::from(split.next().unwrap());
            points += play_round(you, opponent);
        }

        assert_eq!(15, points)
    }

    #[test]
    fn play_first_input() {
        let mut points: u64 = 0;
        for line in INPUT.lines() {
            let mut split = line.split(" ").into_iter();
            let opponent = Move::from(split.next().unwrap());
            let you = Move::from(split.next().unwrap());
            points += play_round(you, opponent) as u64;
        }

        assert_eq!(11767, points)
    }

    #[test]
    fn play_second_sample() {
        let input = "A Y
B X
C Z";
        let mut points = 0;
        for line in input.lines() {
            let mut split = line.split(" ").into_iter();
            let opponent = Move::from(split.next().unwrap());
            let result = Result::from(split.next().unwrap());
            points += guess_move(opponent, result);
        }

        assert_eq!(12, points)
    }

    #[test]
    fn play_second_input() {
        let mut points = 0;
        for line in INPUT.lines() {
            let mut split = line.split(" ").into_iter();
            let opponent = Move::from(split.next().unwrap());
            let result = Result::from(split.next().unwrap());
            points += guess_move(opponent, result);
        }

        assert_eq!(13886, points)
    }
}

const INPUT: &str = "A Z
C X
A Z
A Z
C Y
C Y
A Z
A Y
C Y
A Y
A Z
A Z
A Z
A Y
A Z
A Y
C Y
C X
A Y
C Y
C Y
C X
A Z
C Y
C X
A X
A Y
A Z
A Y
A Y
C X
C X
B Y
C X
C X
A Y
A Z
A Z
A X
A Z
A Z
C Y
A Z
A Z
A Y
C X
C Y
C X
B X
C Z
A Y
A Z
A Z
A Z
A Y
A Y
C X
A Y
A Z
C Y
A Y
A Y
A Z
A Z
C Y
A Z
C Y
A Y
A Z
A Z
C Y
B Z
A Z
A Z
A Z
A Z
C X
C X
A Y
A X
A Y
A Z
A Z
C X
A Z
C X
A Z
C Y
C X
A Z
A Z
A Z
A Z
C X
C Y
A Z
A Z
C Y
A Z
B Z
C X
A Z
A X
C X
A Z
C Z
A Z
C X
A Z
A Y
C X
C Z
A Z
C X
C Y
A Z
B Z
B Y
A X
A X
A X
A Z
A Z
A X
A Z
A X
A Z
A Z
C X
C X
B Z
A Z
A Y
A Z
A Z
A Z
A Z
C X
C X
C X
A Z
A Z
A Z
A Z
A X
A Y
A Y
C X
C X
B Z
C X
A X
A Z
A Z
C X
C Z
A Z
A Z
C Y
A X
A Z
C Y
A X
A Y
A Y
A Y
C Y
A Z
A Z
C X
C X
C X
C X
B Z
C Y
C X
C X
A Y
A Y
A Z
A Z
A X
C X
A Z
B Z
A Z
C X
A Y
A Z
A Z
A Y
A Y
A Z
C X
A Y
C X
C X
A Z
A Y
C X
A Z
A Z
A Y
A Z
A Z
C Y
C Z
A Y
A Z
C X
C X
A Z
A Z
C Y
A Y
A Z
A Y
A Z
C X
A Z
A Z
C X
A Y
A Y
C Y
A Z
C Y
A Z
A Z
C Y
A Y
B Z
C Y
C X
C X
A Z
C Y
A X
C Y
A Z
A Y
A Z
C X
C X
C Y
C Y
A Y
A Z
C X
C X
A Y
A X
A Y
B Z
A Y
C Y
C X
C X
A X
C X
B Z
A X
C Y
C X
C X
A X
A Z
B Z
A Z
A Z
A Y
A X
A Z
C X
A X
C Y
A Z
A Z
A X
A Z
A Z
C X
C X
A Z
A Z
A Z
A X
A Z
A X
B Y
A Z
A Y
C Y
A Z
C X
A Z
C X
A Y
A Z
C X
C Y
A Y
C Y
A Z
C X
A Z
A Z
C X
A Y
A Z
A X
A Z
A Z
A Z
B Y
C X
A X
A Z
A Z
C Y
C Y
C X
C X
C X
A X
A X
A Z
A X
A Y
A Z
A Y
A Z
C Y
C Z
A Y
A Z
A Z
A X
A Z
A Z
C X
C Y
A Y
C X
C X
A Z
C X
C X
C X
A X
A Z
A Z
A Z
A Z
B Y
A Y
A Y
A Z
C X
A Y
A Z
C X
A Z
C Z
A Y
C X
A Z
B Z
A Y
A X
A Z
C X
A Z
A Z
A Z
A Z
A Z
B Z
A Z
C X
A Y
C X
A Z
A Z
A Y
A Z
B Z
C X
A Y
C Y
A Z
A Z
C X
C X
A Y
C X
C Y
B Z
A Y
C X
A Y
C X
A X
A Y
A Z
A Z
A Y
C X
A X
C X
B Y
A Z
A Y
B Y
A Y
C X
A Z
A Z
C X
C Y
A Y
C X
C Y
A Y
A Z
A X
B Z
C X
A Z
A Y
A Z
B Z
A Z
A X
C Y
A X
A Z
A Y
C Y
A Z
C Y
A Z
C X
C X
A Y
C X
C X
A Y
A Z
A Z
A Y
A X
C Y
A Z
A Z
C X
A X
A Z
C Z
A Z
C Y
A Z
C X
A Z
A Z
A X
C X
C X
C Y
B Z
B Y
C Y
A Y
A X
A Z
C X
A Y
A Y
A Z
A Z
C Z
C X
C X
C X
A Z
C X
A Z
A Z
A Y
C Y
C X
C X
C X
A Y
C X
B Z
C X
A Z
C Y
A Y
C Y
A Z
A Z
C X
A X
A Z
A Y
A Z
B X
C Z
A Z
C Z
A X
C X
C X
C X
C X
A Z
A Y
A Z
A Z
A Z
A Z
A Z
C Z
C Y
C Y
C X
C Y
A Z
C X
C X
A Z
A Z
C X
C Y
C Y
A Z
A X
C Y
C Y
C X
A Y
C X
A Z
A Z
A Y
C Y
A Y
C X
C X
A Z
A Z
C Y
A Z
C X
A Y
A Y
A Z
C Y
A Z
C X
A Y
A Z
A Z
A Z
A Z
C Y
A Y
C Y
A Z
A Z
A X
A Y
A Y
A Z
C Y
A Z
A X
A Z
B Z
C X
C X
C Y
A Z
A Z
C X
C Z
A Z
C X
C Y
A Z
B Z
A Z
B Z
A X
A Y
A Z
A Z
A Z
A Z
C Y
A Z
A Z
A Z
B Z
A Z
C X
C X
A Z
C X
A X
A Z
A Y
A Y
A X
A Z
A Z
A Z
A Z
B Z
A Z
C Y
C Y
C X
C X
B Z
C Y
A Z
C Y
A Z
A Y
C Y
A X
A Y
C X
A Y
C X
A Z
A Z
B Z
A Y
C Y
C X
A Z
A Z
C X
A Y
A Z
A Z
A X
A Y
A Y
A Y
A Z
A Y
A Z
C X
A X
A Z
A Z
C Y
A Z
C X
A Z
C Y
A Y
A Z
A Z
A Z
A Z
C Y
A X
A Z
A Z
A Y
A Z
B Z
A X
A Y
C X
C X
A Y
A Z
C Y
A Z
A Y
A Y
A Z
A Y
A Y
A Z
A Z
A Z
C X
A Z
A Y
A Z
A Z
C Y
A Z
C X
A Y
C X
C Y
A Z
C Y
A Z
A Y
C Y
C Y
A Z
C X
C X
C X
A Z
A Z
A Y
C X
A Z
A X
A Z
A Z
C X
A Z
C Y
A Y
A Z
A Y
A Z
A Z
C Y
A Z
C X
A Z
A X
A Z
A Z
C X
A Z
A Z
C Y
C X
A Z
C Y
C X
C X
A Z
A Z
A Z
A X
C X
A Z
A Z
C Y
A Y
C X
A Y
C X
C Y
A Y
A Z
C X
A Z
B Y
A X
B Z
A Y
A X
C Y
A Z
A X
A Z
A Z
C Y
A Z
B Z
C X
C X
B Y
A Z
A Z
A Z
C Y
C X
A Z
A Y
A Y
A Y
C Y
C X
A Z
A X
A X
A X
A Z
A Z
A Z
A Z
C X
C X
A Z
C X
A Y
C X
B Z
A Z
A Z
C X
A Z
C X
C X
C X
A Y
C Y
C X
A Y
C X
C Y
B Z
A Z
C X
A Z
A X
B Z
A Y
B Y
A Z
A Z
A X
A Z
A X
A Z
A Z
C X
C Y
A X
C Y
C X
A Z
A Z
A Z
C Y
A X
A Y
A Z
A Y
C X
B Z
A Z
A Y
C Z
C X
A Z
A Z
A Z
B Z
A X
C X
A Z
A Z
B Z
A Z
A Z
B Z
A Z
C X
A Z
C X
A Z
C X
A Z
C Y
A Z
A X
A Y
A Y
C X
A Y
C X
B Z
A Z
A Z
A Z
A Z
C X
A Z
C X
A Z
A Y
A Z
B Z
A Y
C Z
A Y
C X
A Z
A Z
A Y
B Z
A X
C Y
A Z
A Y
A Z
A Y
A Y
A Z
A Z
A Z
A X
A Z
C Y
A Y
A X
A Y
C X
A Y
C X
A X
C X
A Z
C Y
A Y
A Z
A Z
A X
A Y
C X
C X
A Y
A X
A Z
B Z
A Y
A Z
A X
A Z
B Y
A Y
A Y
A Z
C X
A Z
A Z
A Y
A Z
C Z
A Z
A Z
A Z
A Z
C Y
C Y
A Y
B Z
C Y
A Y
C Y
A Z
A Z
A Z
A Z
C Y
A Z
A Z
C Y
C X
A Y
A Y
A Z
C X
C Z
C X
C X
A Z
A Z
A Y
A Z
A X
C Y
A Z
A Z
C Y
C X
A X
A Z
A Z
A X
C X
C X
C X
A Z
A X
C X
C Y
A X
A Z
C X
A Z
A Z
C X
A Y
A Z
A Z
A Y
A Z
C X
A Z
A Z
C X
A Y
A Z
A Z
C X
A Z
A Y
A Z
C Y
A X
A Z
A Z
C X
A Z
A Y
C Y
B Z
A Z
A Y
C X
A Z
B Z
A Z
C Z
A X
A Z
A Z
C Y
A Z
A Y
C X
C Y
A Z
A Z
A Y
A X
C Y
A Y
C X
C Y
A Z
C Y
A Z
C X
A Z
A Z
A X
A Z
A Z
B X
A X
A Z
C Y
A Z
A X
C X
A Z
A Z
C X
C Y
C Y
A X
A Y
C Y
A Y
A Z
A Z
A Z
A Z
A Y
C X
C X
C Y
C X
A Z
A X
B Z
B Y
C X
C Y
A Y
A Z
A Y
C X
C Z
A Z
A Y
C Y
C X
A Z
A Z
A Y
C X
C Z
C Y
A Z
C X
C Y
A X
A X
A Y
A Z
B Z
A X
A Y
A Y
C X
C Y
A Z
A X
A Z
A X
A Y
A Z
A Z
A Z
C X
A Z
A Z
A Z
C Z
C Y
C Y
A Z
C Y
C Y
C Y
C X
A Z
C X
C X
A Z
A Y
A Z
A Z
A X
A Y
A Y
C X
C X
A Z
A Z
A Z
A Z
A Y
A Z
A Z
A Z
A Z
A X
A X
A Y
A X
C Y
A Y
A Z
C X
A Y
A Y
A Z
A Z
A Z
C X
A Z
C X
C X
C Y
A Y
A Z
A Y
A Z
C X
C X
A Z
A Z
A Z
C X
A X
A Z
A Z
C Y
C Y
A Y
A Y
A Z
A Z
C Y
C X
C Y
A X
C Y
C X
C Y
A Z
A Z
A X
C Y
C Y
A Z
A Y
C X
A X
B Z
A Z
C X
A Y
A X
A Z
A Z
A Z
C Y
C X
A Z
C X
A Z
A Z
A Y
A Z
A Y
A X
A Z
C Z
A Z
A Z
A Z
A X
A Z
A X
C X
A Z
A Z
C Y
A X
A Z
C X
C X
A Y
A Z
A Z
C X
B X
A Z
C Y
A Z
C X
A Y
A Z
C X
A X
A Z
A X
A Z
C X
A Z
A X
A Z
C X
C Y
A Y
A X
A X
A Z
C Y
C Y
A Y
A Y
A Y
A X
A Z
A Z
A Z
C Y
C Y
A Y
B X
B X
A Z
C X
C X
A Z
C Y
C X
A Z
A Z
A Z
A Z
C X
A Z
A Z
C Y
A Y
C Y
A Z
C Y
C Y
A Z
C X
A Z
A X
A Z
C X
C X
A Y
B Z
A Y
C Y
A Z
C Y
A X
C X
A Y
C X
A Z
C Z
C Y
A Z
C X
C Y
A Z
A X
A Z
A Z
A Z
C X
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Y
A Z
A X
A Y
C Y
B Y
C X
B Z
A Z
A Z
A Y
B Z
A Z
A Z
C X
C Y
C X
A Y
A X
C X
C Y
A Y
C X
C Y
A Z
A Z
A Z
A Y
C X
A Y
C X
B Z
A Z
A Y
A Z
A Z
A Y
C X
A Z
C X
C Y
A Y
A Z
B Z
C X
A Y
C X
A Y
A Z
C X
A X
C X
B Y
C X
A Z
A Y
A Z
A Y
A X
C X
C X
A Y
C X
A Y
A Y
A X
B Y
A Y
C X
C X
A Y
B Z
B X
B Z
A Y
A Z
C Y
A Y
B Y
A Z
C X
A Z
A Z
A Z
A Z
B Z
C X
C Y
A Z
C Y
C Y
A X
C X
A Z
A Z
A Z
C X
C X
C X
A Y
C Z
C Z
A Z
C X
A Y
A Z
A Z
A Z
C X
A Z
A X
A Z
A Z
A Z
A Z
A Y
C Y
C X
A X
A Y
C X
A X
A Z
A Z
C X
A Z
A X
A Z
A Z
A X
A Z
A Z
A X
A Z
A X
B Y
A Y
A Y
C Y
A Z
A Y
C X
A Z
A Y
A X
C Y
B X
C Y
A Z
C X
A Y
A Z
A Y
A X
C Y
A Z
A Z
C Y
C X
A Z
C X
A Y
C X
A Z
A Y
A Z
A Z
A Z
A Y
A Z
C X
C X
A X
C X
C X
A Z
C X
A Z
C Y
C X
A Z
A Z
A Z
C X
A X
C Y
A Z
C Y
A X
A Z
C X
A Z
A Z
A X
A Z
C X
B X
A Z
A Z
A Z
C X
A Y
A Y
A X
C Y
C Y
A Z
A Y
A Z
A Z
C X
A X
A Y
A Z
A Z
A Z
A Z
B Z
C X
C X
C X
A Z
C Z
A X
C X
A Z
C Y
A Z
A Z
A Y
A Y
C X
A Z
A X
A Z
A Z
A Z
A Z
C Y
A Z
A Y
A X
A X
A Z
C X
A X
A X
A Z
A Y
C X
A Z
A Z
A Y
A Z
B Z
C X
C X
C Z
C Y
C X
A Z
C Y
A Z
C Z
A Z
A Y
A Y
A X
A X
A Z
A Y
A Y
A Y
A Y
A Z
C Y
A Z
A Z
C X
A Z
A Z
C Y
A Y
C X
A Y
C X
A Z
B Z
A X
B X
A Y
A X
A Y
B Z
A Y
A Z
C Y
C Y
A Z
A X
A Z
A Z
C Z
A Z
A Y
C X
A Y
C X
A X
A Y
C Y
A Y
A Z
A Z
C X
C X
B Z
A Z
A Z
A X
C X
C Y
A Z
A Z
A X
C X
C Z
A Z
C Y
A Y
B Z
C Y
A Z
C X
A X
A Z
A Z
A Z
A Y
C Y
A Z
C Y
A Z
A Z
A Z
A Z
A Y
C X
A Y
C Y
B Z
A Z
C X
C Y
A Z
C X
A Z
C X
C Z
A Z
C X
C X
A Z
A Z
A Y
A Y
A Y
C X
A Y
A Z
A Z
A Z
A Z
A Z
A Z
A Y
A Z
A X
C Y
A Z
A Z
A Z
A Y
A Z
A Z
A Z
A Z
C X
B Z
A Z
A Y
A Y
A X
A Z
A Z
C Z
A Z
C X
A Y
A X
B Z
A Z
A Z
A Z
C Y
C Z
C X
A Z
C Y
C Y
C Y
C X
B Z
A Z
A Z
C Z
A X
A Z
A Z
A Z
C X
A Z
A Z
C Y
C Y
A Z
A Z
C X
A Y
C Y
C Y
A Z
A Z
A X
A Z
A Z
A X
A X
C X
A Z
A X
C X
C X
A Z
A X
A Z
C X
C X
C X
C Y
A Z
A Z
A X
A Z
A Y
A Z
A Z
C Y
A Z
A Z
A Z
A X
A Z
C X
A X
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Y
A X
A Y
A X
C Y
A Z
A Z
C X
A Z
A Z
A Z
A X
A Z
A Z
A Y
C X
A Y
A Z
C X
A X
A Y
A Z
A Z
A X
A Z
A Y
C X
A Y
A Z
A Z
A Z
A Z
C X
C Y
A Z
B Z
C X
A Z
A Z
C Y
C Y
C X
A X
C Y
B Y
A Z
A Z
A Z
C Y
A Z
A X
A Y
A Z
A Z
A Z
C Y
A Z
C Y
C X
A Z
A Z
A Z
A Z
A Z
A X
A Z
A Z
C X
A Z
A Y
C Z
A Z
A Z
A Z
B X
C X
A Z
A Z
A Z
A Z
C X
A Z
A Z
A Y
A X
C X
C Y
A X
A Y
C X
A Z
A Z
C X
C X
A Z
C X
A Z
A Y
C X
A Z
B X
B Y
A X
C Y
A X
A Y
C Y
A Z
A Z
A Z
C X
A Z
A X
A Z
C Y
A Z
A Z
C X
A Z
A Z
A Y
A Z
A X
A Y
A Z
C X
C Y
B X
C Y
A Y
A Z
C X
A Z
C X
C Y
A X
A X
A Z
A Z
C Y
A Y
A Y
A Z
A Z
C X
A X
C X
A X
A Y
C X
A Z
A Z
B Y
A Z
A Z
A Z
A Z
A Z
A X
A Y
A X
A Z
A Z
C X
A Y
A Z
C Y
C X
C X
C X
C X
C Y
A X
A X
C Y
A X
A Y
A Y
B Z
A Z
B Y
C Y
A X
A Y
A Z
A Z
A Z
A X
C X
A Z
A Z
A Z
A Z
A Z
A Y
A Z
B Z
A Z
A Z
A Y
C Y
C Y
C X
A Z
A X
C X
A Y
B Z
C X
A Z
C X
C Y
C Y
A X
A X
C X
A Z
A Z
A Y
A X
A X
A Z
C Y
B X
A Z
A Z
A Z
A Z
A Y
A Y
A Z
C Y
C Y
A X
A Z
A Z
C Y
A Y
A Y
A X
A Y
A X
A X
A X
A Z
A Z
A Y
A Z
C X
A Z
A Y
A Z
A Z
A Y
A Z
A Z
C X
A Z
A Y
A Z
A Z
A Z
A Z
A Z
A Z
A Y
C X
B Z
C Z
A Z
A Y
A X
A X
C Y
C Y
A X
C X
B Z
A Z
C X
A Z
A Z
A Z
A Z
A Z
A Z
A Z
A Y
A Z
A Z
A Y
A Z
A Y
A Y
A Z
A Z
A Y
A Z
C Y
A Z
A Z
A Z
C X
C X
A Z
C Y
A Z
A Z
C X
A Z
C X
A Y
A Z
C Y
A Z
A Z
C X
C Y
A Z
C X
C X
A X
A Y
C Y
A Y
A Z
C X
A Z
A Z
A X
A Z
C Y
A Z
C X
A Z
C X
A Z
C X
A Y
C Y
A X
A Z
A Z
C Y
A X
A Z
A Y
B Z
A Z
A X
A Z
A Z
A X
C X
A Z
A Z
C Y
A Z
A Y
A Z
C Y
A Z
A Y
A Z
C Z
A Y
A Z
A Y
C Y
A Z
C X
A X
B Z
C X
C X
A Z
A Z
A Y
A X
A Z
A Z
B Z
A Z
C Z
A X
A Z
A Z
A Z
A Z
A X
A X
A Z
A X
A Z
A Z
A Z
A X
C X
C Y
A X
C X
C X
A Z
A X
C Y
B Z
A Z
A Z
C X
B X
A Z
A Z
C X
C X
B Z
C Y
A Z
A Z
C X
A X
A Y
C Y
C Y
B Z
A Y
C X
A Z
A Z
A Y
C X
A Z
A Z
C X
C Y
A Z
C X
C Y
C Y
C X
C Y
A Z
C Y
A Z
C X
A Z
C Y
C Z
A Z
B Z
A Z
C X
C X
B Y
B Z
C Y
C X
C X
A Y
C X
C X
A Z
A Y
A Z
C X
A X
A Z
A Z
A Z
A Z
A Z
A Z
A Z
C X
C X
A Z
C Y
C X
A Z
B Z
A Z
A Z
A X
C X
A Y
A Z
A Z
A Y
A X
C Y
B X
A Z
A X
C Y
C X
C X
C Y
C Y
A Z
A Z
C Y
A Z
A Y
C Y
A Y
C X
A Z
C X
C Y
C Y
A Z
A Z
A X
A Y
A Z
A X
A Z
A Z
A Z
C Y
A Z
A Z
C X
C Y
A Z
A Z
A Z
C Y
C X
A Z
C X
A Z
A Z
A Z
A X
A Z
A Z
A Y
B Z
A Z
A Z
A Z
C Y
A Z
B Z
A Z
C Z
A Z
A Y
C X
C Y
C X
C X
A Z
A Z
A Y
A Y
A Z
A Y
B Z
C Y
A Y
A Z
C X
A Z
A Z
C Y
A Y
A Z
A Y
C Y
A Z
A Z
A X
B Z
A Z
A X
C X
A Z
C Y
C Y
A Z
B Y
A Y
A Z
A Z
A Z
A Z
C X
C X
A Z
C X
A Y
A Z
A Z
C Y
A Z
A Z
C Y
C X
B Z
A Y
A Y
C X
C X
A Z
A X
B Z
A Z
C Y
A Z
A Y
A Z
A Z
A Y
C Y
C X
A Z
C X
A Z
C Y
A Z
C Y
A Y
A Z
A Y
A Z
C X
A Z
A Z
C X
A Z
B Y
A Z
A Z
C Y
C X
C X
A Z
A Z
C X
B Z
A Y
A Z
A Y
A Z
A Z
A Y
A X
C X
C X
A X
A Z
A Y
A Y
A Z
A Y
A Z
A Z
C Y
A X
A Z
A Z
C X
A Z
A X
B X
C X
A Z
A Y
B Z
C X
C Y
A Z
B Z
C Y
A Z
A Z
A X
A Z
A Z
A Z
A Z
A Z
A Z
A Z";