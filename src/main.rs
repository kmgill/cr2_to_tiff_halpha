
mod raw_to_tiff;
use std::env;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();
 
    for i in 1..args.len() {
        let in_file = &args[i];
        if Path::new(&in_file).exists() {
            println!("Processing File: {}", in_file);
            raw_to_tiff::process(&in_file);
        } else {
            println!("File not found: {}", in_file);
        }
    }
}
