#![feature(slice_partition_dedup)]

use std::collections::VecDeque;
use std::io;

fn all_unique_chars(v: &VecDeque<char>) -> bool {
    let mut v1= VecDeque::new();
    v.clone_into(&mut v1);

    let v1_c = v1.make_contiguous();
    v1_c.sort();
    let (v1_dedup, _) = v1_c.partition_dedup();

    v1_dedup.len() == 4
}

fn main() -> io::Result<()>{
    let mut input = String::new();
    let mut marker = VecDeque::with_capacity(4);

    io::stdin().read_line(&mut input)?;

    for (idx, ch) in input.chars().enumerate() {
        if idx < 4 {
            marker.push_back(ch);
            continue;
        }

        marker.pop_front();
        marker.push_back(ch);

        if all_unique_chars(&marker) {
            println!("Marker appears at {}", idx+1);
            break;
        }
    }

    Ok(())
}
