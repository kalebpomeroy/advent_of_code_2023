use std::collections::HashSet;
use std::collections::HashMap;


fn get_numbers(ln: &str) -> HashSet<i64> {
    ln.split(" ").filter_map(|s| s.parse().ok()).collect()
}

pub fn run(lines: Vec<String>, part_one: bool) -> i64 {
    let mut total = 0;
    let mut cards: HashMap<usize, i64> = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        
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

    return match part_one { 
        true => total, 
        false => { cards.values().cloned().sum() }
    }
}