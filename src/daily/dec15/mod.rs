use std::collections::HashMap;


fn hash(c: char, current_value: i32) -> i32 {
    
    let mut new_value = current_value.clone();
    
    new_value += c as i32;
    new_value = new_value * 17;
    new_value = new_value % 256;
    new_value
}

pub fn run(lines: Vec<String>, part_one: bool) -> i64 {
    let mut total = 0;

    let line = lines.first().unwrap();
    let steps = line.split(",");

    // let boxes:HashMap<i32, HashMap<&str, i32>> = HashMap::new();

    for step in steps {
        let mut step_total = 0;
        for c in step.chars(){
            step_total = hash(c, step_total);
        }

        // if step includes a -
            // find the box
            // if the box exists
                // for each (index, (label, power)) in box.iter().enumerate()
                    // if label == step.label
                        // to remove = index

        // if step includes =
            // find the box
            // if the box exists
                // for each label in the box
                    // if matches
                        // replace with new value
                // add to existing list
            // create list

        total += step_total;
    }

    if part_one { return total as i64 }

    return 0
}

#[cfg(test)]
mod tests {

    const TEST_DATA: &str = "HASH";
    const STEP_DATA: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    use super::*;

    #[test]
    fn test_part_one() {
        let test_data: Vec<String> = TEST_DATA.lines().map(|line| line.to_string()).collect();
        assert_eq!(run(test_data, true), 52);
    }
    #[test]
    fn test_part_one_with_steps() {
        let test_data: Vec<String> = STEP_DATA.lines().map(|line| line.to_string()).collect();
        assert_eq!(run(test_data, true), 1320);
    }
}