#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]

const INPUT: &str = include_str!("inputs/02.txt");

mod rps {
    pub type Score = i32;
    #[derive(PartialEq, Eq)]
    pub enum Rps {
        Rock,
        Paper,
        Scissors,
    }

    pub enum MatchResult {
        Win,
        Draw,
        Lose,
    }

    impl MatchResult {
        pub fn from_char(c: char) -> Self {
            let c_lower = c.to_ascii_lowercase();
            match c_lower {
                'a' | 'x' => Self::Lose,
                'b' | 'y' => Self::Draw,
                'c' | 'z' => Self::Win,
                _ => panic!("Not a valic character"),
            }
        }

        pub const fn from_moves(own: &Rps, other: &Rps) -> Self {
            match own {
                Rps::Rock => match other {
                    Rps::Rock => Self::Draw,
                    Rps::Paper => Self::Lose,
                    Rps::Scissors => Self::Win,
                },
                Rps::Paper => match other {
                    Rps::Rock => Self::Win,
                    Rps::Paper => Self::Draw,
                    Rps::Scissors => Self::Lose,
                },
                Rps::Scissors => match other {
                    Rps::Rock => Self::Lose,
                    Rps::Paper => Self::Win,
                    Rps::Scissors => Self::Draw,
                },
            }
        }

        pub const fn get_score(&self) -> Score {
            match self {
                Self::Lose => 0,
                Self::Draw => 3,
                Self::Win => 6,
            }
        }
    }

    impl Rps {
        pub fn from_char(c: char) -> Self {
            let c_lower = c.to_ascii_lowercase();
            match c_lower {
                'a' | 'x' => Self::Rock,
                'b' | 'y' => Self::Paper,
                'c' | 'z' => Self::Scissors,
                _ => panic!("Not a valic character"),
            }
        }
        pub const fn from_other_outcome(other: &Self, outcome: &MatchResult) -> Self {
            match other {
                Self::Rock => match outcome {
                    MatchResult::Win => Self::Paper,
                    MatchResult::Draw => Self::Rock,
                    MatchResult::Lose => Self::Scissors,
                },
                Self::Paper => match outcome {
                    MatchResult::Win => Self::Scissors,
                    MatchResult::Draw => Self::Paper,
                    MatchResult::Lose => Self::Rock,
                },
                Self::Scissors => match outcome {
                    MatchResult::Win => Self::Rock,
                    MatchResult::Draw => Self::Scissors,
                    MatchResult::Lose => Self::Paper,
                },
            }
        }
        pub const fn get_score(&self) -> Score {
            match self {
                Self::Rock => 1,
                Self::Paper => 2,
                Self::Scissors => 3,
            }
        }
    }
}

fn run_pt_1() {
    let result = INPUT
        .trim()
        .lines()
        .map(|round| {
            let mut play = round
                .split_whitespace()
                .map(|s| s.chars().next().unwrap());
            let opponent = rps::Rps::from_char(play.next().unwrap());
            let own = rps::Rps::from_char(play.next().unwrap());
            let match_result = rps::MatchResult::from_moves(&own, &opponent);
            match_result.get_score() + own.get_score()
        })
        .sum::<rps::Score>();
    println!("Day 2: Rock Paper Scissors part 1:");
    println!("{result:?}");
}

fn run_pt_2() {
    let result = INPUT
        .trim()
        .lines()
        .map(|round| {
            let mut play = round
                .split_whitespace()
                .map(|s| s.chars().next().unwrap());
            let opponent = rps::Rps::from_char(play.next().unwrap());
            let match_result = rps::MatchResult::from_char(play.next().unwrap());
            let own = rps::Rps::from_other_outcome(&opponent, &match_result);
            match_result.get_score() + own.get_score()
        })
        .sum::<rps::Score>();
    println!("Day 2: Rock Paper Scissors part 1:");
    println!("{result:?}");
}

fn main(){
    let start = std::time::Instant::now();
    run_pt_1();
    run_pt_2();
    let duration = start.elapsed();
    println!("Finished in {duration:#?}");
}