use std::io;

fn process_empty_line(current_list: &mut Vec<u64>, max_calories: &mut u64, top_three: &mut Vec<u64>) {
    let current_calories = current_list.iter().sum();
    if current_calories > *max_calories {
        *max_calories = current_calories;
    }
    if top_three.len() < 3 {
        top_three.push(current_calories);
        top_three.sort();
    } else {
        let mut larger = false;
        for el in top_three.iter() {
            if current_calories > *el {
                larger = true;
            }
        }
        if larger {
            top_three.push(current_calories);
            top_three.sort();
            top_three.remove(0);
        }
    }
}

fn main() -> io::Result<()> {
    let mut max_calories: u64 = 0;
    let mut current_list = Vec::new();
    let mut top_three = Vec::new();

    for line in io::stdin().lines() {
        let input = line.unwrap();
        if input.is_empty() {
            // End of elf
            process_empty_line(&mut current_list, &mut max_calories, &mut top_three);
            current_list.clear();
        } else {
            let current_entry = input.trim().parse().unwrap();
            current_list.push(current_entry);
        }
    }

    process_empty_line(&mut current_list, &mut max_calories, &mut top_three);

    println!("Maximum calories is {}", max_calories);

    println!("Top three is {:?}", &top_three);
    let top_three_sum: u64 = top_three.iter().sum();
    println!("Top three sum is {}", top_three_sum);

    Ok(())
}
