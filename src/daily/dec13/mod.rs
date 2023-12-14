use advent::util::load_file;

fn find_reflection(pattern: &Vec<String>, clean_smudges: bool) -> Option<usize> {
    for (index, _) in pattern.iter().enumerate() {
        
        // If this is the first or last line, we don't need to check 
        if index >= pattern.len() || index == 0 { continue }

        let mut smudge_cleaned = !clean_smudges;
        // Starting from index, loop over every line in the pattern
        for (offset, i) in (index..pattern.len()).enumerate() {

            // The next comparison we would make would be negative. Since we reached
            // the end of the list without breaking, `index` is the reflection line.
            if offset >= index {
                if smudge_cleaned {
                    return Some(index);
                }
                break
            }

            let pairing_line = index - offset - 1;

            // Get the two corresponding lines
            match (pattern.get(pairing_line), pattern.get(i)) {
                // If there are two lines...
                (Some(a), Some(b)) => {
                    // ...and they don't match, we can bail completely
                    if a != b {
                        if !smudge_cleaned && a.chars()
                                            .zip(b.chars())
                                            .filter(|(a, b)| a != b)
                                            .count() == 1 {
                            smudge_cleaned = true;
                        } else {
                            break 
                        }
                    }
                }
                // There shouldn't be a situation where this occurs due to careful checks above
                _ => { return None }
            }

            // If this is the last line in the list, `index` is the reflection line
            if smudge_cleaned && i >= pattern.len()-1 {
                return Some(index)
            }
        }
    }
    None
}

fn rotate_pattern(pattern: &Vec<String>) -> Vec<String>{

    let mut new_pattern: Vec<String> = Vec::new();

    for line in pattern {
        for (x, ch) in line.chars().enumerate() {
            if x >= new_pattern.len() {
                new_pattern.push(String::new());
            }
            let _ = &mut new_pattern[x].push_str(&ch.to_string());
        }
    }

    return new_pattern
}

pub fn run(part_one: bool) -> i64 {

    let mut total = 0;
    let mut patterns: Vec<Vec<String>> = Vec::new();

    for line in load_file() {
        if ! line.contains('.') {
            patterns.push(Vec::new());
            continue;
        } 

        match patterns.last_mut() {
            Some(p) => { p.push(line)}
            _ => { patterns.push([line].to_vec()) }
        }
    }

    for pattern in patterns {

        let mut working_pattern = pattern.clone();
        
        total += match find_reflection(&working_pattern, !part_one) {
            Some(r) => r * 100,
            _ => {
                working_pattern = rotate_pattern(&working_pattern);
                find_reflection(&working_pattern, !part_one).unwrap()
            }
        };        
    }

    return total as i64
}