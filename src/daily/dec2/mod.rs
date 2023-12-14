use std::cmp;

struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_game_data(line: &str) -> (i32, Vec<Round>) {
    // Split the string into two parts: the game number and the game descriptions
    let parts: Vec<&str> = line.splitn(2, ':').collect();

    // Parse the game number
    let game_number = parts[0].trim().split_whitespace().last().unwrap().parse().unwrap();

    // Split the game descriptions into a vector
    let round_strings: Vec<String> = parts[1].trim().split(';')
                               .map(|s| s.trim().to_string())
                               .collect();

    // A list of all the rounds we'll be returning
    let mut rounds: Vec<Round> = Vec::new();                               
    for round in &round_strings {

        // A list of all the colors as a string like "8 green"
        let colors: Vec<&str>= round.split(",").collect();      
        
        // By default, rounds have 0 of each color
        let mut round = Round { red: 0, green: 0, blue: 0 };      

        // For each color in the list
        for color in &colors {

            // Get the color name (as a string) and the value (as an int)
            let c: Vec<&str> = color.trim().split(" ").collect();
            let color = c.last().unwrap_or(&"").to_string();
            let value: i32 = c.first().unwrap_or(&"").parse().expect("Failed to parse count");

            // Once we have the color name and value, set it in the round we have
            match color.as_str() {
                "red" => round.red = value,
                "green" => round.green = value,
                "blue" => round.blue = value,
                _ => {}
            }
        }
        // Push the round to our list we'll be returning
        rounds.push(round);
    }      

    (game_number, rounds)
}

pub fn run(lines: Vec<String>, part_one: bool) -> i64 {
   
    // For part one, this is the "correct" limits
    let baseline = Round {
        red: 12,
        green: 13, 
        blue: 14
    };
    
    let mut total = 0;

    for line in lines {

        // Turn the string of text into something sane to work with
        let (game_id, rounds) = parse_game_data(&line);
        
        // We'll keep track of the highest running totals here
        let mut max_round = Round { red: 0, green: 0, blue: 0 };

        // For each round, set the max value of each color
        for round in &rounds { 
            max_round.red = cmp::max(round.red, max_round.red);
            max_round.blue = cmp::max(round.blue, max_round.blue);
            max_round.green = cmp::max(round.green, max_round.green);
        }

        match part_one {
            // For part one, we want to count the valid games. A valid game is any where the max round 
            // is less than or equal to the 
            true => {
                if max_round.blue <= baseline.blue && max_round.red <= baseline.red && max_round.green <= baseline.green { 
                    total += game_id;
                }
            }, 

            // For part two, we want to multiply the highest totals from each game
            false => {
                total += max_round.blue * max_round.red * max_round.green;
            }
        }
        
    }
    
    return total as i64;
}
