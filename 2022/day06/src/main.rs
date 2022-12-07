#![feature(slice_partition_dedup)]

use std::collections::VecDeque;
use std::io;

fn all_unique_chars(v: &VecDeque<char>) -> bool {
    let mut v1= VecDeque::new();
    v.clone_into(&mut v1);

    let v1_c = v1.make_contiguous();
    v1_c.sort();
    let (v1_dedup, _) = v1_c.partition_dedup();

    v1_dedup.len() == v.len()
}

fn main() -> io::Result<()>{
    let mut input = String::new();
    let mut start_of_packet_marker = VecDeque::with_capacity(4);
    let mut start_of_message_marker = VecDeque::with_capacity(14);

    io::stdin().read_line(&mut input)?;

    for (idx, ch) in input.chars().enumerate() {
        if idx < 4 {
            start_of_packet_marker.push_back(ch);
            continue;
        }

        start_of_packet_marker.pop_front();
        start_of_packet_marker.push_back(ch);

        if all_unique_chars(&start_of_packet_marker) {
            println!("Start of packet marker appears at {}", idx+1);
            break;
        }
    }

    for (idx, ch) in input.chars().enumerate() {
        if idx < 14 {
            start_of_message_marker.push_back(ch);
            continue;
        }

        start_of_message_marker.pop_front();
        start_of_message_marker.push_back(ch);

        if all_unique_chars(&start_of_message_marker) {
            println!("Start of message marker appears at {}", idx+1);
            break;
        }
    }

    Ok(())
}
