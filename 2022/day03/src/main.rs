use std::io;

fn char_to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        u32::from(c) - u32::from('a') + 1
    } else {
        u32::from(c) - u32::from('A') + 27
    }
}

fn process_line(line: String) -> String {
    let mut result= String::new();

    let length = line.len();
    let (first, second) = line.split_at(length / 2);

    for ch in first.chars() {
        if second.contains(ch) {
            if !result.contains(ch) {
                result.push(ch);
            }
        }
    }

    result
}

fn main() {
    let mut total_score = 0;
    let mut group_vec = Vec::new();
    let mut group_prio_sum = 0;

    for line in io::stdin().lines() {
        let input = line.unwrap();
        let processed_line = process_line(input.clone());
        let mut current_score = 0;
        for c in processed_line.chars() {
            current_score += char_to_priority(c);
        }
        total_score += current_score;

        group_vec.push(input);
        if group_vec.len() == 3 {
            for ch in group_vec[0].chars() {
                if group_vec[1].contains(ch) && group_vec[2].contains(ch) {
                    group_prio_sum += char_to_priority(ch);
                    break;
                }
            }
            group_vec.clear();
        }
    }

    println!("Total score is {}", total_score);
    println!("Group priority sum is {}", group_prio_sum);
}
