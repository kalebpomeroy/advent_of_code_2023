use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn get_file_path() -> String {    
    // By default, if an argument is passed, we should use that as the data file
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        return args[2].to_string();
    }
    return format!("src/daily/{}/data.txt", args[1]);
}

fn read_file_to_list(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn load_file() -> Vec<String> {
    let file_path = get_file_path();
    match read_file_to_list(&file_path) {
        Ok(lines) => lines, 
        Err(e) => {
            println!("Error reading file ({}): {}", file_path, e);
            process::exit(1);
        }
    }
}
 
  