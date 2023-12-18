use advent::util::Coordinate;
use std::collections::HashSet;
use std::cmp;

pub const NORTH: Coordinate = Coordinate { x: 0, y: -1 };
pub const SOUTH: Coordinate = Coordinate { x: 0, y: 1};
pub const EAST: Coordinate = Coordinate { x: 1, y: 0};
pub const WEST: Coordinate = Coordinate { x: -1, y: 0};


#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Beam {
    pos: Coordinate,
    direction: Coordinate
}

impl Beam {
    fn next(&self, direction: Coordinate) -> Beam {
        return Beam {pos: self.pos.add(direction), direction: direction}
    }
}

fn move_light(beam: &Beam, lines: Vec<String>) -> Vec<Beam>{   
    // We know where we are, and what direction we came from (both on the beam struct)

    // This method returns a list of beams that tell us where to go next
    let mut beams: Vec<Beam> = Vec::new();

    // We need to get the current character in the position 
    let current_line = lines.get(beam.pos.y as usize).unwrap();     
    let current_ch = current_line.chars().nth(beam.pos.x as usize).unwrap();
    match current_ch {
        // Turn 90 degrees (the match is where we were coming from, 
        // the beam.next is where we're going to)
        '/' => {
            match beam.direction {
                NORTH => { beams.push(beam.next(EAST)) }
                SOUTH => { beams.push(beam.next(WEST)) }
                EAST => { beams.push(beam.next(NORTH)) }
                WEST => { beams.push(beam.next(SOUTH)) }
                _ => {}
            }
        }
        // Another 90 degress turn
        // Note, this is escaped, but should be considered a single backslash
        '\\' => {  
            match beam.direction {
                NORTH => { beams.push(beam.next(WEST)) }
                SOUTH => { beams.push(beam.next(EAST)) }
                EAST => { beams.push(beam.next(SOUTH)) }
                WEST => { beams.push(beam.next(NORTH)) }
                _ => {}
            }
            
        }

        // This is a splitter. If we come from east or west, just keep going
        '-' => {
            match beam.direction {
                EAST | WEST  => { beams.push(beam.next(beam.direction)) }
                NORTH | SOUTH => { 
                    beams.push(beam.next(EAST));
                    beams.push(beam.next(WEST));
                }
                _ => {}
            }
        }
        
        // This is a splitter. If we come from north or south, just keep going
        '|' => {

            match beam.direction {
                NORTH | SOUTH => { beams.push(beam.next(beam.direction)) }
                EAST | WEST  => { 
                    beams.push(beam.next(NORTH));
                    beams.push(beam.next(SOUTH));
                }
                _ => {}
            }
        }

        // If it's not a direction change, just keep going the way we're going
        _ => {
           beams.push(beam.next(beam.direction))
        }
    }

    // At this point, we should always have the next beam location. It's ok if 
    // it is out of bounds or otherwise invalid. We'll check that later
    return beams;
}

fn route_beams(beams: Vec<Beam>, lines: Vec<String>) -> Option<Vec<Beam>>{
    // Given a list of beams, execute all of them

    // Create a list for all beams that are still active
    let mut all_beams: Vec<Beam> = Vec::new();
    for beam in beams.clone() {
        // Execute each beam in the list, and all them to the active list
        all_beams.extend(move_light(&beam, lines.clone()));      
    }

    // If we don't have any beams, then we're done
    if all_beams.len() == 0 { return None }

    // We need to de-dup the beams so we don't get into loops
    let mut unique_beams: HashSet<Beam> = HashSet::new();
    
    // We also need to check to make sure the new beams are valid
    for beam in all_beams{
        if beam.pos.x < 0 || beam.pos.y < 0 || beam.pos.y >= lines.len() as i32 { 
            continue; 
        }
        if beam.pos.x >= lines.first().unwrap().len() as i32 { 
            continue;
        }
        // If it's in bounds, insert the beam into the list
        unique_beams.insert(beam);
    }
    // Return the list of beams we'll want to step through next
    return Some(unique_beams.into_iter().collect());
}

// Given a unique list of beam paths, which coordinates are energized
fn get_energized_spaces(beam_paths: HashSet<Beam>) -> HashSet<Coordinate> {
    let mut energized_spaces: HashSet<Coordinate> = HashSet::new();
    for beam_path in beam_paths {
        energized_spaces.insert(beam_path.pos);
    }
    energized_spaces
}

fn print_map(lines: Vec<String>, beam_paths: HashSet<Beam>){
    let energized_spaces = get_energized_spaces(beam_paths);
    for (y, line) in lines.iter().enumerate() {
        if y == 0 { println!("  0 1 2 3 4 5 6 7 8 9")}
        print!("{y} ");
        for (x, ch) in line.chars().enumerate() {

            if energized_spaces.contains(&Coordinate::new(x, y)) {
                print!("# ")
            } else { print!("{ch} ")}
        }
        println!(" {y}");
    }
}

fn beam_me_up(beam: Beam, lines: Vec<String>) -> i64 {
    let mut beams: Vec<Beam> = [beam.clone()].to_vec();
    let mut beam_paths: HashSet<Beam> = beams.clone().into_iter().collect();

    // We're going to keep routing beams until their are no (new) beams left
    while let Some(current_beams) = route_beams(beams.clone(), lines.clone()) {
        let mut new_beams:Vec<Beam> = Vec::new();
        for beam in current_beams.clone() {
            if beam_paths.insert(beam.clone()){
                new_beams.push(beam);
            }
        } 
        beams = new_beams;
    }

    println!("Generating map for {beam:?}");
    print_map(lines, beam_paths.clone());
    return get_energized_spaces(beam_paths.clone()).len() as i64;
}

pub fn run(lines: Vec<String>, part_one: bool) -> i64 {
    
    if part_one {
        // Create the first beam and put it in the lists. Starts at 0, 0 going east
        return beam_me_up(Beam{
            pos: Coordinate::new(0,0),
            direction: EAST
        }, lines);
    }

    let mut max = 0;
    let line_len = lines.first().unwrap().len();
    for y in 0..lines.len() {
        max = cmp::max(max, beam_me_up(Beam {pos: Coordinate::new(0, y), direction: EAST}, lines.clone()));
        max = cmp::max(max, beam_me_up(Beam {pos: Coordinate::new(line_len-1, y), direction: WEST}, lines.clone()));
    }

    for x in 0..line_len {
        max = cmp::max(max, beam_me_up(Beam {pos: Coordinate::new(x, 0), direction: SOUTH}, lines.clone()));
        max = cmp::max(max, beam_me_up(Beam {pos: Coordinate::new(x, lines.len()-1), direction: NORTH}, lines.clone()));
    }
    
    return max

}

#[cfg(test)]
mod tests {

    const TEST_DATA: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    use super::*;

    #[test]
    fn test_part_one() {
        let test_data: Vec<String> = TEST_DATA.lines().map(|line| line.to_string()).collect();
        assert_eq!(run(test_data, true), 46);
    }
    #[test]
    fn test_part_two() {
        let test_data: Vec<String> = TEST_DATA.lines().map(|line| line.to_string()).collect();
        assert_eq!(run(test_data, false), 51);
    }
}