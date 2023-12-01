use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;


fn read_file_to_list(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

const WORD_MAP: &[(&str, &str)] = &[
    ("one", "1"),
    ("two", "2"),
    ("two", "2"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <=1 {
        println!("Please provide the path to the calibration file.");
        process::exit(1);
    }

    let file_path = &args[1];
    let mut total = 0;
    match read_file_to_list(file_path) {
        Ok(lines) => {
            for line in &lines {
                let mut working_line = line.to_string();
                for (w, n) in WORD_MAP.iter() {
                    // What a dirty hack. In the case of the string "oneight" I can't just replace
                    // the word with the number. Instead, this turns into one1oneight8eight and
                    // then I can just look at the numbers without messing up the order.
                    working_line = working_line.replace(w, &format!("{}{}{}", w, n, w).to_string());
                }
                if working_line != line.to_string() {
                    println!("replaced {} with {}", line, working_line)
                }
                let numbers: Vec<_> = working_line.chars().filter(|ch| ch.is_numeric()).collect();

                if numbers.len() < 1 {
                    println!("Line had no numeric characters: {}", line);
                    continue
                }
                let first = numbers.first().unwrap_or(&'0');
                let last = numbers.last().unwrap_or(&'0');
                let calibration_str = format!("{}{}", first, last);
                let calibration_value: i32 = calibration_str.parse().expect("Failed to parse calibration_value");
                total += calibration_value;
            }
        },
        Err(e) => println!("Error reading file: {}", e),
    }
    println!("Calibration total: {}", total)
}
