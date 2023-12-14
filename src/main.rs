// src/main.rs

mod daily;
use advent::util::Part;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [script_name]", args[0]);
        std::process::exit(1);
    }
    
    let runner: Option<fn(Part) -> i64> = match args[1].as_str() {
        "dec1" => Some(daily::dec1::run),
        _ => None,
    };

    match runner {
        Some(r) => {
            println!("Part One: {}", r(Part::One));
            println!("Part Two: {}", r(Part::Two));
        },
        _ => {
            println!("Couldn't find the runner (check main.rs)");
        }
    }


}
