use advent::util::Coordinate;

fn char_at(s: &str, index: usize) -> Option<char> {
    s.char_indices()
        .find(|(i, _)| *i == index)
        .map(|(_, c)| c)
}

fn replace_char_at(s: &mut String, index: usize, new_char: char) {
    if let Some(c) = s.chars().nth(index) {
    if c != new_char {
        s.replace_range(index..index+c.len_utf8(), &new_char.to_string());
    }
}
}

fn move_single_rock(lines: Vec<String>) -> Option<(Coordinate, Coordinate)>{
    // for each line
    for (y, line) in lines.iter().enumerate() {
        // for each character
        for (x, ch) in line.chars().enumerate(){
            // if it's not a period we don't care as the space is occupied and we move on
            if ch != '.' { continue }

            // for each line underneath the current
            for sub_y in y..lines.len() {
                let sub_line = lines.get(sub_y).unwrap();

                // find the character at the same X
                match char_at(&sub_line, x).unwrap() {
                    // If subline.x is a # we quit and don't move anything else in this column
                    '#' => { break } 
                    // If subline.x an O, we roll it up to the current line
                    'O' => {
                        let prev_o_pos = Coordinate {x: x as i32, y: sub_y as i32};
                        let new_o_pos = Coordinate {x: x as i32, y: y as i32};
                        return Some((prev_o_pos, new_o_pos));
                    }
                    _ => { continue }
                }
            }
        }
    }
    return None;
}

pub fn run(lines: Vec<String>, _: bool) -> i64 {
    let mut total = 0;

    let mut mutable_lines = lines.clone();

    while let Some((prev_o_pos, next_o_pos)) = move_single_rock(mutable_lines.clone()) {
        replace_char_at(mutable_lines.get_mut(prev_o_pos.y as usize).unwrap(), prev_o_pos.x as usize, '.');
        replace_char_at(mutable_lines.get_mut(next_o_pos.y as usize).unwrap(), next_o_pos.x as usize, 'O');
    }
    for (y, line) in mutable_lines.iter().enumerate() {
        for c in line.chars() {
            if c == 'O' { total += mutable_lines.len() - y }
        }
    }
    return total as i64
}

#[cfg(test)]
mod tests {

    const TEST_DATA: &str = 
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
    use super::*;

    #[test]
    fn test_part_one() {
        let test_data: Vec<String> = TEST_DATA.lines().map(|line| line.to_string()).collect();
        assert_eq!(run(test_data, true), 136);
    }
}