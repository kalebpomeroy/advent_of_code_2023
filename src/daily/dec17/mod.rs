use advent::util::{ Coordinate, Grid, Space };
use advent::util::{ NORTH, SOUTH, EAST, WEST };

use std::collections::{HashSet, HashMap};

pub const VALID_DIRECTIONS: [Coordinate; 4] = [NORTH, SOUTH, EAST, WEST];

pub fn get_neighbors(current: &Space, grid: &Grid) -> Vec<Space>{

    let mut neighbors: Vec<Space> = Vec::new();
    
    // We can travel in all four directions
    for dir in VALID_DIRECTIONS {
        let new_x = current.c.x + dir.x;
        let new_y = current.c.y + dir.y;

        // See if that space is a valid space in the grid
        if let Some(option) = grid.getu(new_x, new_y) {
            // Add it to the list
            neighbors.push(option.clone());
        }
    }
    return neighbors;
}

pub fn get_g_scores(grid: &Grid, start: &Space, end: &Space) -> HashMap<Space, i32>{
    // This function builds a map of the "costs" to get to any space from the starting space

    // First we're gonna keep two sets of Spaces
    let mut open_set: HashSet<Space> = HashSet::new();
    let mut closed_set: HashSet<Space> = HashSet::new();

    open_set.insert(start.clone());

    // Two lists of scores. g_score is the "real" score that maps the lowest 
    // value from start to the current square. f_score is a "guess" based on 
    // hueristics. My specific hueristic is the number of steps. This would 
    // be the lowest possible score (ie, all remaining steps are value 1)
    let mut g_score: HashMap<Space, i32> = HashMap::new();
    let mut f_score: HashMap<Space, i32> = HashMap::new();

    // For each space in our grid, we're going to fill it with the max value. 
    for row in &grid.spaces {
        for space in row {
            g_score.insert(space.clone(), i32::MAX);
            f_score.insert(space.clone(), i32::MAX);
        }
    }

    // We want the real score of the starting space to be 0
    g_score.insert(start.clone(), 0);

    // The score of the end space is, at minimum, the distance
    f_score.insert(start.clone(), start.c.get_distance(end.c));

    // We're gonna loop through all the open set 
    // (first loop would just be the starting space)
    while !open_set.is_empty() {

        // Get the space that is the lowest scoring space in the open set
        let current = open_set.iter().min_by_key(|node| f_score.get(node).unwrap()).unwrap().clone();

        // If the lowest scoring item in the set IS the end space, we're done
        if current == *end { break; }

        // Remove the current from the open set and add it to the closed set
        open_set.remove(&current);
        closed_set.insert(current.clone());

        // For each neighbor of the current space, let's see if it's worth calculating
        for neighbor in get_neighbors(&current.clone(), &grid) {
            // If we've already closed it, no need to do anything
            if closed_set.contains(&neighbor) { continue }
    
            // g_score is the real score. Add the current space's score + relevant spaces value
            let current_g = g_score.get(&current).unwrap();
            let mut new_g_score = current_g + neighbor.value;

            // The neighbor is not in the closed set, so we should add it to the open set
            open_set.insert(neighbor.clone());

            if new_g_score >= *g_score.get(&neighbor).unwrap() {
                // The minimum possible g_score is creater than the current value. This means
                // that whatever path we chose to get to this space is basically useless
                continue;
            }
     
            // This path to neighbor is better than any previous one. Record it!
            g_score.insert(neighbor.clone(), new_g_score);

            // Re-evaluate the f_score based on the new_g_score plus the minimum to the end
            f_score.insert(neighbor.clone(), new_g_score + neighbor.c.get_distance(end.c));
        }
    } 
    return g_score;
}

pub fn run(lines: Vec<String>, _: bool) -> i64 {

    // Define the grid and start and end. For simplicity in the path building, I'm 
    // doing it backwards and doing the top left as the end, and the bottom right 
    // as the starting space. 
    let grid = Grid::new(lines);
    let end = grid.get(0, 0).unwrap();
    let start = grid.get(grid.width()-1, grid.height()-1).unwrap();

    // Get the list of g_scores (cost to move) to each position
    let g_score = get_g_scores(&grid, start, end);

    let mut path = Vec::new();
    let mut current = end.clone();

    // Each time we are going to take a step
    while current != start.clone() {
        path.push(current.clone());
        let neighbors = get_neighbors(&current.clone(), &grid);
        current = neighbors.iter().min_by_key(|node| g_score.get(node).unwrap()).unwrap().clone()
    }
    
    // Nasty print statement
    grid.print_with(|space| {
        let v = g_score.get(space).unwrap();
        let str_v = if path.contains(space) {
            "#".to_string()
        } else if v < &100000 { 
            ".".to_string()
        } else { 
            " ".to_string()
        };
        print!("{str_v}");
    });

    return 0
}

#[cfg(test)]
mod tests {

    const TEST_DATA: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    use super::*;

    #[test]
    fn test_part_one() {
        let test_data: Vec<String> = TEST_DATA.lines().map(|line| line.to_string()).collect();
        assert_eq!(run(test_data, true), 0);
    }
}