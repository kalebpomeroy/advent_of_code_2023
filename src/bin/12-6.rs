fn calculate_time(ms: i32, mm: i32) -> i32 {
    let mut ways_to_win = 0;
    for press in 0..ms {
        if (ms-press)*press > mm {
            ways_to_win += 1;
        }
    }
    return ways_to_win
}

fn main() {
    // My data set (This was easier than parsing the file. Cheating? Maybe.)
    let records = [(38, 234), (67, 1027), (76, 1156), (73, 1236)];
    let mut result = 1;

    for (ms, mm) in records {
        result *= calculate_time(ms, mm);
    }
    println!("{}", result)
}
