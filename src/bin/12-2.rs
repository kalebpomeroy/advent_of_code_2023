use std::cmp;
use std::collections::HashSet;
use std::fmt;
use advent::util::load_file;

struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Round(R: {}, G: {}, B: {})", self.red, self.green, self.blue)
    }
}

fn main() {
   
    const RED_COUNT: i32 = 12;
    const GREEN_COUNT: i32 = 13;
    const BLUE_COUNT: i32 = 14;

    let mut total = 0;

    let mut valid_games: HashSet<i32> = HashSet::new();
    for game in load_file() {
        let parts: Vec<&str> = game.split(":").collect();
        if parts.len() != 2 {
            println!("Poorly formatted data")
        }
        
        let round_string = parts.last().unwrap_or(&"");
        let rounds: Vec<&str>= round_string.split(";").collect();
        let mut max_round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        for round in &rounds {
            let colors: Vec<&str>= round.split(",").collect();
            for color in &colors {
                let c: Vec<&str> = color.trim().split(" ").collect();
                let color = c.last().unwrap_or(&"").to_string();
                let value: i32 = c.first().unwrap_or(&"").parse().expect("Failed to parse count");
                match color.as_str() {
                    "red" => max_round.red =  cmp::max(value, max_round.red),
                    "green" => max_round.green = cmp::max(value, max_round.green),
                    "blue" => max_round.blue = cmp::max(value, max_round.blue),
                    _ => println!("Invalid color"),
                }
            }
        }
        total += max_round.blue * max_round.red * max_round.green;
        if max_round.blue <= BLUE_COUNT && max_round.red <= RED_COUNT && max_round.green <= GREEN_COUNT { 
            let game_string = parts.first().unwrap_or(&"");
            let game_split: Vec<&str>= game_string.split(" ").collect();
            let game_number: i32 = game_split.last().unwrap_or(&"").parse().expect("Failed to parse game number");
            valid_games.insert(game_number);
        }
    }
    
    let sum: i32 = valid_games.iter().sum();
    println!("Game Additions: {:?}", sum);
    println!("Total Power: {:?}", total);
}
