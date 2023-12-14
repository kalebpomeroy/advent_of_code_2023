use advent::util::load_file;
use std::cmp;

fn get_number_pos(line: &str) -> Vec<(usize, usize)> {
    // This returns a list of tuples such that (starting_pos, ending_pos) for each number
    let mut positions = Vec::new();

    // If this is set, we're "counting" meaning we're in the middle of a number sequence
    let mut start_index = None; 

    for (i, ch) in line.char_indices() {
        match (ch.is_numeric(), start_index) {
            // If it's a numeric character, and we're not counting:
            (true, None) => start_index = Some(i), // set the start_index to current position

            // If it's not a number and we're counting, we've reached the end
            (false, Some(start)) => {
                positions.push((start, i)); // Push this position to the vector
                start_index = None; // Reset the start_index for the next number   
            },
            // Other scenarios, all caught by the default:
            // (true, Some(start)) means it's a numeric character, but we're counting 
            // (false, None) means is a non numeric character, and we're not counting
            _ => {}
        }
    }

    // Handle case where a number sequence is at the end of the string
    if let Some(start) = start_index {
        positions.push((start, line.len()));
    }
    positions
}


fn main() { 
    let mut total = 0;
    let mut gear_total = 0;
    let lines = load_file();
    for (y, line) in lines.iter().enumerate() {
        for (x_left, x_right) in &get_number_pos(&line) {

            // Rust is weird and usize can't be negative. This is apparently
            // so common that they created a method to "subtract until 0" which 
            // feels bizarre but actually exactly what I needed in this case. 
            // note: I tried using cmp::max and learned about subtracting with overflow
            let x_start = x_left.saturating_sub(1);

            // We still use cmp::min for the right most value
            let x_end = cmp::min(*x_right + 1, line.len());

            let mut area = String::new();

            // Get the previous line's substring value (only if we're past the first line)
            if y > 0 {
                area.push_str(&lines[y-1][x_start..x_end])
            }
            
            // Get the current line's substring value
            area.push_str(&lines[y][x_start..x_end]);

            // Get the next line's substring value (only if we're not on the last line)
            if y+1 < lines.len() {
                area.push_str(&lines[y+1][x_start..x_end])
            }

            // If the area string contains ANY nonnumeric, non period characters, add it
            if area.chars().any(|ch| !ch.is_numeric() && ch != '.') {
                let part_string = &lines[y][*x_left..*x_right];
                let part_number: i32 = part_string.parse().expect("Failed to parse part number");
                total += part_number;
            } 
        }
        
        // For part two, let's look at every gear (the '*' character)
        let gears: Vec<usize> = line.char_indices()
                                    .filter(|&(_, ch)| ch == '*')
                                    .map(|(index, _)| index)
                                    .collect(); 
        for gear in gears {
            let mut part_one = None;

            // Build the list of relevant lines, which is usually 3 but 2 on the first/last lines
            let mut relevant_lines: Vec<String> = Vec::new();
            if y > 0 { relevant_lines.push(lines[y-1].clone()) }
            relevant_lines.push(lines[y].clone());
            if y+1 < lines.len() { relevant_lines.push(lines[y+1].clone()) }

            for line in &relevant_lines {
                
                // For each line, let's reuse the method from the first part of the exercise to get 
                // the list of numbers
                for (x_left, x_right) in &get_number_pos(&line) {

                    // Let's get the total range of possible spaces
                    let min_x_left = x_left.saturating_sub(1);
                    let max_x_right = cmp::min(x_right + 1, line.len()); 

                    // If the possible range includes the space the gear is in
                    if (min_x_left..max_x_right).contains(&&gear) {
                        let gear_no_str = &line[*x_left..*x_right];
                        let gear_no: i32 = gear_no_str.parse().expect("Failed to parse gear number");

                        // If there is a previous part, we have our second and can add to the total
                        if let Some(p1) = part_one {
                            gear_total += p1 * gear_no;
                        // Otherwise save it as part one
                        } else {
                            part_one = Some(gear_no);
                        }
                    } 
                }
            }
        }
    }
    println!("Part gear total: {}", gear_total);
    println!("Part number total: {}", total);
}