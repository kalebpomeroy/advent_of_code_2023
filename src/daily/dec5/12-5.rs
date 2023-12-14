use advent::util::load_file;
use std::ops::Range;

#[derive(Debug)]
struct Converter {
    destination: i64,
    range: Range<i64>,
}


fn calculate_overlap(a: &Range<i64>, b: &Range<i64>) -> Option<Range<i64>> {
    return if a.end <= b.start || a.start >= b.end {
        // No overlap
        None
    } else {
        // Overlap exists
        Some(a.start.max(b.start)..a.end.min(b.end))
    };
}

fn convert(conversion_level: usize, seed_range: Range<i64>, sources: &Vec<Vec<Converter>>) -> i64{
    
    // If it's the last conversion level, just return the range
    if conversion_level >= sources.len() { 
        return seed_range.start;
    }

    for _ in (0..conversion_level) { print!("\t" )}
    println!("Converting {:?}", seed_range);

    let converters = sources.get(conversion_level).unwrap();
    
    let mut locations: Vec<i64> = Vec::new();
    for con in converters {
        let split_range = calculate_overlap(&con.range, &seed_range);
        if split_range.is_some() {
            let overlap_range = split_range.unwrap();
            let length = overlap_range.end - overlap_range.start + 1;
            let new_range = con.destination..(con.destination + length);

            
            for _ in (0..conversion_level) { print!("\t" )}
            println!("Found a convertor for {:?} (starting with {:?})", overlap_range, new_range);

            locations.push(convert(conversion_level + 1, new_range, &sources));
        }
    }
    let lowest_value = locations.iter().min().unwrap_or(&10000000000);
    return *lowest_value;
}

fn main() {
    let mut seeds: Vec<i64> = Vec::new();
    let mut sources: Vec<Vec<Converter>> = Vec::new();

    for line in load_file() {
        if line.len() == 0 { continue }

        if seeds.len() == 0 {
            let seed_result: Result<Vec<i64>, _> = line[7..].split(" ").map(|s| s.parse()).collect();
            match seed_result {
                Ok(s) => { seeds = s; }
                Err(e) => { eprintln!("Failed to parse seeds: {:?}", e) }
            }
        } else if let Some(_) = line.find('-') {
            sources.push(Vec::new());
        } else {
            let conversions: Result<Vec<i64>, _> = line.split(" ").map(|s| s.parse()).collect();
            match conversions {
                Ok(conversions) => { 
                    sources.last_mut().unwrap().push(Converter {
                        destination: conversions[0],
                        range: conversions[1]..(conversions[1] + conversions[2])
                    })
                }
                Err(e) => { eprintln!("Failed to parse seeds: {:?}", e) }
            }
        }         
    }

    let mut locations: Vec<i64> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let seed_start = seeds[i];
        let seed_range = seed_start..(seed_start + seeds[i+1]);

        locations.push(convert(0, seed_range, &mut sources));
        // if loc < lowest_loc { lowest_loc = loc.clone()}
    }
    println!("{:?}", locations.iter().min());

}
  