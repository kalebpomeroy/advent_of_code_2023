use advent::util::load_file;
use advent::util::prepend_to_vector;
use std::fmt;


#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinate {
    x: i32, 
    y: i32
}

impl Coordinate {
    fn add(&self, c: Coordinate) -> Coordinate{
        return Coordinate { 
            x: c.x + self.x, 
            y: c.y + self.y
        };
    }

    fn get_space(&self, rows: &Vec<String>) -> Option<char>{
        if self.y < 0 || self.y > rows.len() as i32{ return None }
        else if self.x < 0 || self.x > rows.first().unwrap().len() as i32 { return None }
    
        return rows.get(self.y as usize).unwrap().chars().nth(self.x as usize);
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

const NORTH: (char, Coordinate) = ('^', Coordinate { x: 0, y: -1 });
const SOUTH: (char, Coordinate) = ('v', Coordinate { x: 0, y: 1});
const EAST: (char, Coordinate) = ('>', Coordinate { x: 1, y: 0});
const WEST: (char, Coordinate) = ('<', Coordinate { x: -1, y: 0});

const ORIGIN: Coordinate = Coordinate {x: 0, y: 0};
fn main() {

    let lines = load_file();
    let mut path: Vec<Coordinate> = Vec::new();
    let mut starting_space: Coordinate;
    // Store each line in the appropriate vector for type of hand it is
    for (index, line) in lines.clone().iter().enumerate() {

        let position = line.char_indices()
                .find(|&(_, c)| c == 'S')
                .map(|(i, _)| i);

        match position {
            Some(pos) => {
                starting_space = Coordinate { x: pos as i32, y: index as i32 };

                let c = starting_space.get_space(&lines);
                println!("{:?} found in position {:?}", c, starting_space);
                path = venture_down_the_pipe(&lines, starting_space, ORIGIN, 0).unwrap();
            },
            None => {},
        }
    }


    println!("There are {} steps ", path.len());

    let mut formatted_lines: Vec<String> = Vec::new();

    for (y, line) in lines.clone().iter().enumerate() {
        let mut formatted_line = String::from("");
        for (x, _) in line.char_indices() {

            if path.contains(&Coordinate {x: x as i32, y: y as i32 }){
                formatted_line.push_str("-");
              } else {

                let mut has_n_pipe = false;
                let mut has_s_pipe = false;
                let mut has_e_pipe = false;
                let mut has_w_pipe = false;

                for p in path.clone() {
                    if !has_n_pipe && p.x == x as i32 && p.y < y as i32 { has_n_pipe = true }
                    if !has_s_pipe && p.x == x as i32 && p.y > y as i32 { has_s_pipe = true }
                    if !has_e_pipe && p.y == y as i32 && p.x < x as i32 { has_e_pipe = true }
                    if !has_w_pipe && p.y == y as i32 && p.x > x as i32 { has_w_pipe = true }
                }

                if has_n_pipe && has_s_pipe && has_e_pipe && has_w_pipe {
                    formatted_line.push_str("I");
                } else {
                    formatted_line.push_str(".");
                }

            }
        }
        formatted_lines.push(formatted_line);
    }

    // I'm getting frustrated and brute forcing it
    let mut formatted_lines = clean_up_the_edges(&formatted_lines);
    for _ in 1..5 {
        formatted_lines = clean_up_the_edges(&formatted_lines);
    }

    let whole_block = formatted_lines.join("\n");
   
    let count = whole_block.chars().filter(|&c| c == 'I').count();

    println!("count: {}", count);
    println!("{}", whole_block);
}

fn clean_up_the_edges(blob: &Vec<String>) -> Vec<String> {
    let mut cleaned_lines: Vec<String> = Vec::new();
    let directions_to_check = [NORTH, SOUTH, EAST, WEST].to_vec();
    for (y, line) in blob.clone().iter().enumerate() {
        let mut formatted_line = String::from("");
        for (x, c) in line.char_indices() {
            let mut new_c = c.clone();
            if c == 'I' {
                let pos = Coordinate {x: x as i32, y: y as i32};
                for (c, step) in &directions_to_check {
                    let new_coordinate = pos.add(*step);
                    if new_coordinate.get_space(blob).unwrap_or_default() == '.' {
                        new_c = '.';
                    }
                }
            }
            formatted_line.push_str(&new_c.to_string());
     
        }
        cleaned_lines.push(formatted_line);
    }
    return cleaned_lines
}

fn venture_down_the_pipe(rows: &Vec<String>,
                         position: Coordinate,
                         previous_position: Coordinate, 
                         total_steps: i64) -> Option<Vec<Coordinate>>{
    if total_steps > 100000 { return Some([position].to_vec()); }

    let valid_directions: Vec<(char, Coordinate)> = match position.get_space(&rows) {
        Some('|') => {[SOUTH, NORTH].to_vec()},
        Some('-') => {[EAST, WEST].to_vec()},
        Some('L') => {[NORTH, EAST].to_vec()},
        Some('7') => {[WEST, SOUTH].to_vec()},
        Some('J') => {[NORTH, WEST].to_vec()},
        Some('F') => {[EAST, SOUTH].to_vec()},
        Some('S') => {[NORTH, SOUTH, EAST, WEST].to_vec()},
        _ => {Vec::new()}
    };

    if valid_directions.len() == 4 && previous_position != ORIGIN {
        return Some([position].to_vec());
    } else if valid_directions.len() == 0 {
        // println!("We hit a dead end at {:?}. Walking back", position.get_space(&rows));
        return None
    } else {
        // println!("We have some choices {}", valid_directions.len());
        for (_, step) in &valid_directions {
            let new_coordinate = position.add(*step);
            if new_coordinate == previous_position {
                // println!("No sense travelling the way we came...");
                continue;
            } else {
                // println!("We're going to {} {} ({} total steps)", c, new_coordinate, total_steps);
                let prev = venture_down_the_pipe(rows, new_coordinate, position, total_steps + 1);
                match prev {
                    Some(p) => {
                        return Some(prepend_to_vector(position, p))
                    },
                    _ => { continue }
                }
            }
        }
    }

    return None;

}