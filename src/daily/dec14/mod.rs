pub fn run(lines: Vec<String>, _: bool) -> i64 {

    let mut total = 0;

    for line in lines {
       println!("{line}");
       total += 1;
    }

    return total as i64
}
