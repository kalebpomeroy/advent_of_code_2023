use advent::util::load_file;

fn main() {
    for line in load_file() {
        println!("{}", line);
    }
    println!("Hello, world!");
}
