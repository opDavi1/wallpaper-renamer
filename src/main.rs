use std::{env, process};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Not enough arguments!");
        process::exit(1);
    }

    let dir = Path::new(&args[1]);
    if let Err(e) = wprn::run(dir) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }   
}

