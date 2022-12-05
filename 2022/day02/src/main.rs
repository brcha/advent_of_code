use std::cmp::Ordering;
use std::io;

#[derive(Eq, Debug, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        match &self {
            RPS::Rock => {
                match &other {
                    RPS::Rock => Ordering::Equal,
                    RPS::Paper => Ordering::Less,
                    RPS::Scissors => Ordering::Greater,
                }
            }
            RPS::Paper => {
                match &other {
                    RPS::Rock => Ordering::Greater,
                    RPS::Paper => Ordering::Equal,
                    RPS::Scissors => Ordering::Less,
                }
            }
            RPS::Scissors => {
                match &other {
                    RPS::Rock => Ordering::Less,
                    RPS::Paper => Ordering::Greater,
                    RPS::Scissors => Ordering::Equal,
                }
            }
        }
    }
}

impl From<RPS> for u64 {
    fn from(value: RPS) -> Self {
        match value {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

fn calculate_player_value(input: &str) -> RPS {
    match input {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        _   => RPS::Scissors,
    }
}

fn calculate_opponent_value(input: &str) -> RPS {
    match input {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        _   => RPS::Scissors,
    }
}

fn calculate_result(opponent: &RPS, player: &RPS) -> u64 {
    if opponent > player {
        0
    } else if opponent == player {
        3
    } else {
        6
    }
}

fn calculate_score(input: &String) -> u64 {
    let mut opponent: RPS = RPS::Rock;
    let mut player: RPS = RPS::Rock;

    for (idx, val) in input.trim().split_whitespace().enumerate() {
        if idx == 0 {
            opponent = calculate_opponent_value(val);
        } else {
            player = calculate_player_value(val);
        }
    }

    let result = calculate_result(&opponent, &player);
    let player_score: u64 = player.into();

    player_score + result
}

fn main() -> io::Result<()> {
    let mut score: u64 = 0;

    for line in io::stdin().lines() {
        let input = line.unwrap();
        score += calculate_score(&input);
    }

    println!("Total score is: {}", score);

    Ok(())
}
