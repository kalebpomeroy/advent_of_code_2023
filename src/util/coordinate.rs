use std::fmt;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i32, 
    pub y: i32
}

pub const NORTH: Coordinate = Coordinate { x: 0, y: -1 };
pub const SOUTH: Coordinate = Coordinate { x: 0, y: 1};
pub const EAST: Coordinate = Coordinate { x: 1, y: 0};
pub const WEST: Coordinate = Coordinate { x: -1, y: 0};
pub const NORTHEAST: Coordinate = Coordinate { x: 1, y: -1 };
pub const SOUTHEAST: Coordinate = Coordinate { x: 1, y: 1};
pub const NORTHWEST: Coordinate = Coordinate { x: -1, y: -1};
pub const SOUTHWEST: Coordinate = Coordinate { x: -1, y: 1};

impl Coordinate {
    pub fn add(&self, c: Coordinate) -> Coordinate{
        return Coordinate { 
            x: c.x + self.x, 
            y: c.y + self.y
        };
    }

    pub fn new(x: usize, y: usize) -> Coordinate {
        return Coordinate {
            x: x as i32,
            y: y as i32
        }
    }

    pub fn new_i(x: i32, y: i32) -> Coordinate {
        return Coordinate { x: x, y: y}
    }

    pub fn get_distance(&self, c: Coordinate) -> i32 {
        return (c.x - self.x).abs() + (c.y - self.y).abs();
    }

    pub fn get_space(&self, rows: &Vec<String>) -> Option<char>{
        if self.y < 0 || self.y > rows.len() as i32{ return None }
        else if self.x < 0 || self.x > rows.first().unwrap().len() as i32 { return None }
    
        return rows.get(self.y as usize).unwrap().chars().nth(self.x as usize);
    }

    pub fn is_blank_of(&self, c: Coordinate) -> Coordinate{

        let mut x: Coordinate = Coordinate::new(0,0);
        let mut y: Coordinate = Coordinate::new(0,0);

        if self.x < c.x {
            x = WEST;
        } else if self.x > c.x {
            x = EAST;
        } 
        
        if self.y < c.y {
            y = NORTH;
        } else if self.y > c.y {
            y = SOUTH;
        }

        return x.add(y);
        

    }

    pub fn between(&self, c: Coordinate) -> Option<Vec<Coordinate>> {
        

        let mut in_between: Vec<Coordinate> = Vec::new();

        // If the y's are the same, we're getting the horizontal between
        if self.y == c.y {
            let start = std::cmp::min(self.x, c.x) + 1;
            let end = std::cmp::max(self.x, c.x);
            
            for x in start..end {
                in_between.push(Coordinate::new_i(x, c.y))
            }
            
        // If the x's are the same, we're getting the vertical between
        } else if self.x == c.x {
            let start = std::cmp::min(self.y, c.y) + 1;
            let end = std::cmp::max(self.y, c.y);
            
            for y in start..end {
                in_between.push(Coordinate::new_i(c.x, y))
            }
        } else {
            // We can only do this for straight lines.
            return None;
        }
        return Some(in_between);
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {

        if self.x == other.x {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

#[cfg(test)]
mod tests_c {
    use super::*;
    #[test]
    fn test_part_one() {
        let nw = Coordinate::new(1,1);
        let se = Coordinate::new(5,5);
        let ne = Coordinate::new(5,1);
        let sw = Coordinate::new(1,5);

        assert_eq!(nw.is_blank_of(ne), WEST);
        assert_eq!(ne.is_blank_of(nw), EAST);
        assert_eq!(se.is_blank_of(ne), SOUTH);
        assert_eq!(ne.is_blank_of(se), NORTH);

        assert_eq!(sw.is_blank_of(ne), SOUTHWEST);
        assert_eq!(ne.is_blank_of(sw), NORTHEAST);
        assert_eq!(se.is_blank_of(nw), SOUTHEAST);
        assert_eq!(nw.is_blank_of(se), NORTHWEST);
    }
}