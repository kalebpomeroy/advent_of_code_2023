use advent::util::load_file;
use std::cmp;
use regex::Regex;

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
        
        let gears: Vec<usize> = line.char_indices()
                                    .filter(|&(_, ch)| ch == '*')
                                    .map(|(index, _)| index)
                                    .collect(); 

        for gear in gears {
            let mut top = String::new();
            let mut bot = String::new();

            let size = (gear-1)..(gear+2);
            if y > 0 {
                top = lines[y-1][size.clone()].to_string();
            }
            let mid = lines[y][size.clone()].to_string();
            if y+1 < lines.len() {
                bot = lines[y+1][size.clone()].to_string();
            }
            let area = [top, mid, bot];
            let re = Regex::new(r"[0-9][\.\*][0-9]").unwrap();

            // Filter the list to the lines that have numbers in them
            let num_lines: Vec<&String> = area.iter().filter(|&s| contains_number(&s)).collect();

            // Filter the list to the lines that have two separate numbers
            let dual_num_lines: Vec<&String> = area.iter().filter(|&s| re.is_match(s)).collect();
                        
            // If there are two lines with numbers, that's a match
            if num_lines.len() == 2 {
                // println!("{:?}", num_lines);
                continue;
            // If there are any lines with two separate numbers, that's a match
            } else if dual_num_lines.len() == 1 {

                let line_offset = match area.iter().position(|s| &s == &dual_num_lines[0]) {
                    Some(index) => index,
                    None => 0,
                };

                
                println!("{:?}", lines[y.saturating_sub(line_offset).saturating_sub(1)]);
                println!("{:?}", lines[y.saturating_sub(line_offset)]);
                println!("{:?}", dual_num_lines);
                println!("{:?} {:?}", y, line_offset);

            // If we have less than two lines and none of the above patterns
            // we can safely continue
            } else {
                continue;
            }

        }
    }
    println!("Part number total: {}", total);
}

fn contains_number(s: &str) -> bool {
    s.chars().any(|ch| ch.is_numeric())
}