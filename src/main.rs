use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: scheduler <input-file>");
        std::process::exit(1);
    }
    let _contents = fs::read_to_string(&args[1]).expect("Failed to read input file");
    // TODO: implement scheduler
}
