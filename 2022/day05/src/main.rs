use std::io;
use regex::Regex;

fn parse_stacks() -> Vec<Vec<char>>{
    let mut stacks = Vec::new();

    for line in io::stdin().lines() {
        let input = line.unwrap();
        let bytes = input.as_bytes();
        if input.is_empty() {
            break;
        }
        if stacks.is_empty() {
            let stacks_num = (bytes.len() + 1) / 4;
            for _ in 0..stacks_num {
                stacks.push(Vec::new());
            }
        }
        for idx in 0..stacks.len() {
            if bytes[1+4*idx] != b' ' {
                stacks[idx].push(char::from_u32(bytes[1+4*idx].into()).unwrap());
            }
        }
    }

    for s in &mut stacks {
        s.pop();
        s.reverse();
    }

    stacks
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Command {
    count: u64,
    from: usize,
    to: usize,
}

fn parse_commands() -> Vec<Command>{
    let mut commands = Vec::new();
    let rx = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in io::stdin().lines() {
        let input = line.unwrap();
        let caps = rx.captures(&input).unwrap();
        let count = caps.get(1).unwrap().as_str().parse().unwrap();
        let from = caps.get(2).unwrap().as_str().parse().unwrap();
        let to = caps.get(3).unwrap().as_str().parse().unwrap();
        commands.push(Command { count, from, to });

    }

    commands
}

fn replay_commands(stacks: &mut Vec<Vec<char>>, commands: &Vec<Command>) {
    for c in commands {
        for _ in 0..c.count {
            let val = stacks[c.from-1].pop().unwrap();
            stacks[c.to-1].push(val);
        }
    }
}

fn replay_commands_part2(stacks: &mut Vec<Vec<char>>, commands: &Vec<Command>) {
    for c in commands {
        let mut temp_stack = Vec::new();
        for _ in 0..c.count {
            let val = stacks[c.from-1].pop().unwrap();
            temp_stack.push(val);
        }
        temp_stack.reverse();
        stacks[c.to-1].append(&mut temp_stack);
    }
}

fn get_solution(stacks: &mut Vec<Vec<char>>) -> String {
    let mut res = String::new();

    for s in stacks {
        res.push(s[s.len()-1]);
    }

    res
}

fn main() {
    let stacks = parse_stacks();
    let commands = parse_commands();

    let mut stacks_p1 = stacks.clone();
    replay_commands(&mut stacks_p1, &commands);
    let solution = get_solution(&mut stacks_p1);

    println!("Solution (part 1) is: {}", solution);

    let mut stacks_p2 = stacks.clone();
    replay_commands_part2(&mut stacks_p2, &commands);
    let solution = get_solution(&mut stacks_p2);

    println!("Solution (part 2) is: {}", solution);
}
