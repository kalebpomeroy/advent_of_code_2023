use advent::util::load_file;
use std::collections::HashMap;

struct Converter {
    source: i64,
    destination: i64,
    range: i64,
}

const MAP_ORDER: [&str; 8] = [
    "seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"
];

fn convert(order: usize, value: i64, sources: &HashMap<String, Vec<Converter>>) -> i64{
    
    if order == 7 { return value }

    let thing = MAP_ORDER[order];
    
    for con in &sources[thing] {
        if value < con.source || value >= con.source + con.range { continue }
        let new_value = con.destination + (value - con.source);
        return convert(order +1, new_value, sources);
    }

    return convert(order+1, value, sources);
}

fn main() {
    let mut seeds: Vec<i64> = Vec::new();
    let mut sources: HashMap<String, Vec<Converter>> = HashMap::new();
    let mut current_name: String = String::new();

    for line in load_file() {
        if line.len() == 0 { continue }

        if seeds.len() == 0 {
            let seed_result: Result<Vec<i64>, _> = line[7..].split(" ").map(|s| s.parse()).collect();
            match seed_result {
                Ok(s) => { seeds = s; }
                Err(e) => { eprintln!("Failed to parse seeds: {:?}", e) }
            }
        } else if let Some(index) = line.find('-') {
            current_name = String::from(line[..index].to_string());
            sources.insert(current_name.clone(), Vec::new());
        } else {
            let conversions: Result<Vec<i64>, _> = line.split(" ").map(|s| s.parse()).collect();
            match conversions {
                Ok(conversions) => { 
                
                    sources.get_mut(&current_name.clone()).unwrap().push(Converter {
                        destination: conversions[0],
                        source: conversions[1],
                        range: conversions[2]
                    })
                }
                Err(e) => { eprintln!("Failed to parse seeds: {:?}", e) }
            }
        }         
    }

    let mut lowest_loc: i64 = 1000000000000;

    for seed in &seeds {
        let loc = convert(0, *seed, &sources); 
        if loc < lowest_loc { lowest_loc = loc.clone()}
    }

    println!("Lowest: {}", lowest_loc);
}
  