use std::collections::HashSet;

use advent::util::Coordinate;
use advent::util::load_file;

fn insert_char(s: &mut String, index: usize, ch: char) {
    let mut char_vec: Vec<char> = s.chars().collect();
    if index <= char_vec.len() {
        char_vec.insert(index, ch);
        *s = char_vec.into_iter().collect();
    }
}

fn main() {

    let lines = load_file();
    let mut expanded_lines: Vec<String> = Vec::new();
    let mut cols: Vec<usize> = Vec::new();
    let mut rows: Vec<usize> = Vec::new();

    for (index, line) in lines.clone().iter().enumerate() {
        expanded_lines.push(line.to_string());

        if ! line.contains("#") {
            rows.push(index);
            expanded_lines.push(line.to_string());   
        }
    }

    for (index, c) in lines.first().unwrap().chars().enumerate() {
        if c != '.' { continue }

        let mut empty_space = true;
        for line in lines.clone() {
            if line.chars().nth(index).unwrap_or_default() == '#' {
                empty_space = false;
                break;
            }
        }

        if empty_space {
            cols.push(index);
        }
    }

    for (i, c) in cols.iter().enumerate() {
        for line in expanded_lines.iter_mut() {
            
            insert_char(line, c+i , 'o');
        }
    }
    

    let mut galaxies: Vec<Coordinate> = Vec::new();
    for (y, line) in expanded_lines.clone().iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Coordinate {x: x as i32, y: y as i32});
            }
        }
    }

    let mut pairs: HashSet<(Coordinate, Coordinate)> = HashSet::new();
    for g in galaxies.clone() {
        for f in galaxies.clone() {
            if f != g && f < g {
                pairs.insert((g, f));
            }
        }
    }

    let mut total: i64 = 0;
    for (a, b) in pairs {

        let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
        let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };

        let matches = cols.iter().filter(|&&x | x >= min_x as usize && x <= max_x as usize ).collect::<Vec<_>>();
        for x in 0..matches.len() {
            println!("Adding a million for {}, {} (col is {:?}) {}-{}", a.x, b.x, matches.clone(), min_x, max_x);
            total+=10
        }
        for _ in 0..rows.iter().filter(|&&y | y >= min_y as usize && y <= max_y as usize ).collect::<Vec<_>>().len() {
            total+=10
        }
        total += a.get_distance(b) as i64;
    }

}
