use advent::util::load_file;

use std::collections::HashSet;

pub fn run(part_one: bool) -> i64 {

    let mut total = 0;
    for line in load_file() {

        let numbers: Vec<i64> = line.split_whitespace()
                                    .filter_map(|s| s.parse().ok())
                                    .collect();

        let pattern = if part_one { get_pattern } else { get_pattern_backwards };
        total += pattern(numbers.clone());

    }

    return total
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
        return last_number + previous_next_number
    }

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
        return first_number - previous_first_number
    }

    return *first_number;
}
