use advent::util::load_file;
use std::collections::HashSet;
use std::collections::HashMap;


fn get_numbers(ln: &str) -> HashSet<i32> {
    ln.split(" ").filter_map(|s| s.parse().ok()).collect()
}

fn main() {
    let mut total = 0;
    let mut cards: HashMap<usize, i32> = HashMap::new();
    for (index, line) in load_file().iter().enumerate() {
        
        let l: Vec<&str> = line[10..line.len()].split("|").collect();
        *cards.entry(index).or_insert(0) += 1;
        let winning_numbers = get_numbers(l.first().unwrap_or(&""));
        let playing_numbers = get_numbers(l.last().unwrap_or(&""));
        let wins = winning_numbers.intersection(&playing_numbers).collect::<Vec<_>>().len();
        if wins > 0 { 
            let mut points = 1;
            for _ in 1..wins { points *= 2 }
            total += points;
        }
        for prize_card in index+1..(index+wins+1) {
            *cards.entry(prize_card).or_insert(0) += *cards.entry(index).or_default();
        }
    }
    let total_sum: i32 = cards.values().cloned().sum();
    println!("The big number is {:?}", total_sum);
    println!("{}", total);
}