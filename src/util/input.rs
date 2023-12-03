use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn get_file_path() -> String {    
    // By default, if an argument is passed, we should use that as the data file
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return args[1].to_string();
    }
     
    // For simplicity, if the file name isn't passed as an arg, let's try to guess 
    // it based on the standard structure; it should be in `data/*.txt` where `*` is 
    // matches the name of the executable (12-1, 12-2...)
    match env::current_exe() {
        Ok(exe_path) => {
            let exe_name = exe_path.file_name().unwrap_or_default().to_string_lossy();
            return format!("data/{}.txt", exe_name);
        }
        Err(e) => {
            println!("Failed to get the executable name: {}", e);
            process::exit(1);
        }
    }
}

fn read_file_to_list(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn load_file() -> Vec<String> {
    match read_file_to_list(&get_file_path()) {
        Ok(lines) => lines, 
        Err(e) => {
            println!("Error reading file: {}", e);
            process::exit(1);
        }
    }
}
 
  