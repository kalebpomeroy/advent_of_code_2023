use advent::util::load_file;

mod daily;
type ScriptRunner = fn(Vec<String>, bool) -> i64;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [script_name]", args[0]);
        std::process::exit(1);
    }
    
    let runner: Option<ScriptRunner> = match args[1].as_str() {
        "dec1" => Some(daily::dec1::run),
        "dec2" => Some(daily::dec2::run),
        "dec3" => Some(daily::dec3::run),
        "dec4" => Some(daily::dec4::run),
        "dec5" => Some(daily::dec5::run),
        "dec6" => Some(daily::dec6::run),
        "dec7" => Some(daily::dec7::run),
        // "dec8" => Some(daily::dec8::run),
        "dec9" => Some(daily::dec9::run),
        // "dec10" => Some(daily::dec10::run),
        "dec11" => Some(daily::dec11::run),
        // "dec12" => Some(daily::dec12::run),
        "dec13" => Some(daily::dec13::run),
        "dec14" => Some(daily::dec14::run),
        "dec15" => Some(daily::dec15::run),
        "dec16" => Some(daily::dec16::run),
        "dec17" => Some(daily::dec17::run),
        _ => None,
    };

    let lines = load_file();
    match runner {
        Some(r) => {
            println!("Part One: {}", r(lines.clone(), true));
            println!("Part Two: {}", r(lines.clone(), false));
        },
        _ => {
            println!("Couldn't find the runner (check main.rs)");
        }
    }


}
