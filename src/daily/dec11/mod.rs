use std::collections::HashSet;

use advent::util::Coordinate;

pub fn run(lines: Vec<String>, part_one: bool) -> i64 {

    let mut cols: Vec<usize> = Vec::new();
    let mut rows: Vec<usize> = Vec::new();

    // Go through and see if any of the lines are an empty space
    for (index, line) in lines.clone().iter().enumerate() {

        // If there's no galaxies, add an expanding line
        if ! line.contains("#") {
            rows.push(index);
        }
    }

    // Go through each character in the first line
    for (index, c) in lines.first().unwrap().chars().enumerate() {
        // If it's not empty space then the column is not just empty space
        if c != '.' { continue }

        // See if any of the characters below have a galaxy in that pos
        let mut empty_space = true;
        for line in lines.clone() {
            if line.chars().nth(index).unwrap_or_default() == '#' {
                empty_space = false;
                break;
            }
        }

        // If we never found a galaxy, add it as a column
        if empty_space {
            cols.push(index);
        }
    }
    
    // Find all galaxies and create them as coordinates
    let mut galaxies: Vec<Coordinate> = Vec::new();
    for (y, line) in lines.clone().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Coordinate {x: x as i32, y: y as i32});
            }
        }
    }

    // Create a list of every possible pair of galaxies
    let mut pairs: HashSet<(Coordinate, Coordinate)> = HashSet::new();
    for g in galaxies.clone() {
        for f in galaxies.clone() {
            // Prevent a galaxy from linking to itself, or adding (3, 4) and (4, 3) as two pair
            if f != g && f < g {
                pairs.insert((g, f));
            }
        }
    }

    let mut total: i64 = 0;
    for (a, b) in pairs {

        // Find the coordinates of the left/right most and high/lowest for comparison
        let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
        let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };

        // How many of our rows/columns are within the bounds of the min/max x/y respectively
        let matching_cols = cols.iter().filter(|&&x | x >= min_x as usize && x <= max_x as usize ).collect::<Vec<_>>().len() as i64;
        let matching_rows = rows.iter().filter(|&&y | y >= min_y as usize && y <= max_y as usize ).collect::<Vec<_>>().len() as i64;

        // In part one, the rate of expansion is 1. In part two, it's a million
        let expansion_rate = if part_one { 1 } else { 999999 };
        total += a.get_distance(b) as i64;

        total += matching_cols * expansion_rate;
        total += matching_rows * expansion_rate;

    }
    return total;
}
