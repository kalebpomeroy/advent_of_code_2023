use advent::util::load_file;
use std::collections::HashSet;

fn get_numbers(ln: &str) -> HashSet<i32> {
    ln.split(" ").filter_map(|s| s.parse().ok()).collect()
}

fn main() {
    let mut total = 0;
    for line in load_file() {
        
        let l: Vec<&str> = line[10..line.len()].split("|").collect();
        
        let winning_numbers = get_numbers(l.first().unwrap_or(&""));
        let playing_numbers = get_numbers(l.last().unwrap_or(&""));
        let wins = winning_numbers.intersection(&playing_numbers).collect::<Vec<_>>().len();
        if wins == 0 { continue }
        
        let mut points = 1;
        for _ in 1..wins { points *= 2 }

        total += points;
    }
    println!("{}", total);
}