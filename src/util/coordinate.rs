use std::fmt;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i32, 
    pub y: i32
}

impl Coordinate {
    pub fn add(&self, c: Coordinate) -> Coordinate{
        return Coordinate { 
            x: c.x + self.x, 
            y: c.y + self.y
        };
    }

    pub fn get_distance(&self, c: Coordinate) -> i32 {
        return (c.x - self.x).abs() + (c.y - self.y).abs();
    }

    pub fn get_space(&self, rows: &Vec<String>) -> Option<char>{
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