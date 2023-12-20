#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Space {
    pub c: Coordinate,
    pub value: i32
}

impl Space {
    fn new_as_i(x: usize, y: usize, i: i32) -> Space {
        return Space {
            c: Coordinate::new(x, y), value: i
        }
    }   
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Grid {
    pub spaces: Vec<Vec<Space>>
}

impl Grid {
    pub fn new(map: Vec<String>) -> Grid {
        let mut spaces: Vec<Vec<Space>> = Vec::new();
        for (y, line) in map.iter().enumerate() {
            let mut y_line: Vec<Space> = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                let digit = ch as i32 - '0' as i32;
                y_line.push(Space::new_as_i(x, y, digit));
            }
            spaces.push(y_line);
        }

        // TODO: We should verify that all lines are the same length
        return Grid { spaces: spaces }
    }

    pub fn width(&self) -> usize {
        self.spaces.first().unwrap().len()
    }

    pub fn height(&self) -> usize {
        self.spaces.len() 
    }

    pub fn getu(&self, x: i32, y:i32) -> Option<&Space>{
        self.get(x as usize, y as usize)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Space>{
        self.spaces.get(y).and_then(|row| row.get(x))
    }

    pub fn print_with<F>(&self, mut func: F)
    where
        F: FnMut(&Space), // F is a closure that takes a reference to a Space
    {
        for row in &self.spaces {
            for item in row {
                func(item); // Apply the closure to each item
                print!(""); // Print a space or any other separator
            }
            println!(); // New line after each row
        }
    }
}
