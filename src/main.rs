
mod path;
mod raw_to_tiff;

use std::env;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();
 
    for i in 1..args.len() {
        let in_file = &args[i];
        if path::file_exists(in_file) {
            println!("Processing File: {}", in_file);
            raw_to_tiff::process(&in_file);
        } else {
            eprintln!("File not found: {}", in_file);
            process::exit(1);
        }
    }
}
