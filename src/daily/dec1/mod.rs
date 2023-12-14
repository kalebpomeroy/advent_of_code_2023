use advent::util::load_file;

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

pub fn run(part_one: bool) -> i64 {
    let mut total: i64 = 0;
    for line in load_file() {
        let mut working_line = line.to_string();

        
        if !part_one {
            for (w, n) in WORD_MAP.iter() {
                // In the case of the string "oneight" we can't just replace
                // the word with the number. Instead, this turns into one1oneight8eight and
                // then I can just look at the numerals without messing up the order.
                working_line = working_line.replace(w, &format!("{}{}{}", w, n, w).to_string());
            }
        }

        // Get a list of numeric characters
        let numbers: Vec<_> = working_line.chars().filter(|ch| ch.is_numeric()).collect();

        // Get the first and last characters from the numbers
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();

        // Make a two digit string based on the first and last
        let calibration_str = format!("{}{}", first, last);

        // Cast that two digit number to an int and add it to the total
        let calibration_value: i64 = calibration_str.parse().expect("Failed to parse calibration_value");
        total += calibration_value;
    }

    return total;
}
  