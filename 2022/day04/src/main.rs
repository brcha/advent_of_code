use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlap(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once('-').unwrap();
        let start = start_str.parse()?;
        let end = end_str.parse()?;
        Ok(Range{ start, end })
    }
}

fn fully_contained(first: &Range, second: &Range) -> bool {
    first.contains(second) || second.contains(first)
}

fn parse_line(line: &str) -> (Range, Range) {
    let (first, second) = line.split_once(',').unwrap();
    (first.parse().unwrap(), second.parse().unwrap())
}

fn process_line(line: &str) -> (bool, bool) {
    let (first, second) = parse_line(line);
    (fully_contained(&first, &second), first.overlap(&second))
}

fn main() {
    let mut count :u64 = 0;
    let mut count_p2 :u64 = 0;

    for line in io::stdin().lines() {
        let input = line.unwrap();
        let (p1, p2) = process_line(&input);

        if p1 {
            count += 1;
        }

        if p2 {
            count_p2 += 1;
        }
    }

    println!("Number of contained ranges is {}", count);
    println!("Number of overlaping ranges is {}", count_p2);
}
