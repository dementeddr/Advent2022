use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {

    const UNIQUE: usize = 14;

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");

    for i in 0..input.len()-UNIQUE+1 {
        
        let mut window_chars = HashSet::new();
        let slice = input.get(i..i+UNIQUE).unwrap();
        
        for c in slice.chars() {
            window_chars.insert(c);
        }
        
        if window_chars.len() == UNIQUE {
            println!("Number of processed chars is {}", i+UNIQUE);
            break;
        }

    }
}
