use advent::util::load_file;

use std::collections::HashSet;

fn main() {

    let mut total = 0;
    let mut prev_total = 0;
    // Store each line in the appropriate vector for type of hand it is
    for line in load_file() {

        let numbers: Vec<i64> = line.split_whitespace()
                                    .filter_map(|s| s.parse().ok())
                                    .collect();

        let next_number = get_pattern(numbers.clone());
        println!("{} << {}", line, next_number);
        total += next_number;

        let prev_number = get_pattern_backwards(numbers.clone());
        println!("[{}] >> {}", prev_number, line);        
        prev_total += prev_number;
    }
    
    println!("Total: {}", total);
    println!("Prev Total: {}", prev_total);

}


fn get_pattern(numbers: Vec<i64>) -> i64 {
    let mut next_line: Vec<i64> = Vec::new();
    let last_number = numbers.last().unwrap_or(&0);

    for i in 0..(numbers.len() - 1) {
        next_line.push(numbers[i + 1] - numbers[i]);
    }
    let unique_numbers: HashSet<i64> = numbers.clone().into_iter().collect();
    if unique_numbers.len() > 1 {

        let previous_next_number = get_pattern(next_line.clone());
        println!("{:?} << [{}]", next_line, previous_next_number);
        return last_number + previous_next_number
    }

    // At this point there is only one number in the array
    println!("{:?}", next_line);
    return *last_number;
}

fn get_pattern_backwards(numbers: Vec<i64>) -> i64 {
    let mut next_line: Vec<i64> = Vec::new();
    let first_number = numbers.first().unwrap_or(&0);

    for i in 0..(numbers.len() - 1) {
        next_line.push(numbers[i + 1] - numbers[i]);
    }
    let unique_numbers: HashSet<i64> = numbers.clone().into_iter().collect();
    if unique_numbers.len() > 1 {

        let previous_first_number = get_pattern_backwards(next_line.clone());
        println!("[{}] >> {:?}", previous_first_number, next_line);
        return first_number - previous_first_number
    }

    // At this point there is only one number in the array
    println!("{:?}", next_line);
    return *first_number;
}
