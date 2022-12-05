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

#[derive(Eq, Debug, PartialEq)]
enum ExpectedResult {
    Lose,
    Draw,
    Win,
}

impl From<ExpectedResult> for u64 {
    fn from(value: ExpectedResult) -> Self {
        match value {
            ExpectedResult::Lose => 0,
            ExpectedResult::Draw => 3,
            ExpectedResult::Win => 6,
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

fn calculate_player_outcome(input: &str) -> ExpectedResult {
    match input {
        "X" => ExpectedResult::Lose,
        "Y" => ExpectedResult::Draw,
        _   => ExpectedResult::Win,
    }
}

fn calculate_opponent_value(input: &str) -> RPS {
    match input {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        _   => RPS::Scissors,
    }
}

fn what_to_play(opponent: &RPS, expected_result: &ExpectedResult) -> RPS {
    match opponent {
        RPS::Rock => {
            match expected_result {
                ExpectedResult::Lose => RPS::Scissors,
                ExpectedResult::Draw => RPS::Rock,
                ExpectedResult::Win => RPS::Paper,
            }
        }
        RPS::Paper => {
            match expected_result {
                ExpectedResult::Lose => RPS::Rock,
                ExpectedResult::Draw => RPS::Paper,
                ExpectedResult::Win => RPS::Scissors,
            }
        }
        RPS::Scissors => {
            match expected_result {
                ExpectedResult::Lose => RPS::Paper,
                ExpectedResult::Draw => RPS::Scissors,
                ExpectedResult::Win => RPS::Rock,
            }
        }
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

fn calculate_score(input: &String) -> (u64, u64) {
    let mut opponent: RPS = RPS::Rock;
    let mut player: RPS = RPS::Rock;
    let mut expected_result: ExpectedResult = ExpectedResult::Win;

    for (idx, val) in input.trim().split_whitespace().enumerate() {
        if idx == 0 {
            opponent = calculate_opponent_value(val);
        } else {
            player = calculate_player_value(val);
            expected_result = calculate_player_outcome(val);
        }
    }

    let result = calculate_result(&opponent, &player);
    let player_score: u64 = player.into();

    let player_score_p2: u64 = what_to_play(&opponent, &expected_result).into();
    let result_p2: u64 = expected_result.into();

    (player_score + result, player_score_p2 + result_p2)
}

fn main() -> io::Result<()> {
    let mut score: u64 = 0;
    let mut score_p2: u64 = 0;

    for line in io::stdin().lines() {
        let input = line.unwrap();
        let (sc_p1, sc_p2) = calculate_score(&input);
        score += sc_p1;
        score_p2 += sc_p2;
    }

    println!("Total score (part 01) is: {}", score);
    println!("Total score (part 02) is: {}", score_p2);

    Ok(())
}
