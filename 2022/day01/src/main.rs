use std::io;

fn main() -> io::Result<()> {
    let mut max_calories: u64 = 0;
    let mut current_list = Vec::new();

    for line in io::stdin().lines() {
        let input = line.unwrap();
        if input.is_empty() {
            // End of elf
            let current_calories = current_list.iter().sum();
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_list.clear();
        } else {
            let current_entry = input.trim().parse().unwrap();
            current_list.push(current_entry);
        }
    }

    let current_calories = current_list.iter().sum();
    if current_calories > max_calories {
        max_calories = current_calories;
    }

    println!("Maximum calories is {}", max_calories);

    Ok(())
}
