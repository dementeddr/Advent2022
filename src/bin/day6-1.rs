use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");

    for i in 0..input.len()-3 {
        
        let mut window_chars = HashSet::new();
        let slice = input.get(i..i+4).unwrap();
        
        for c in slice.chars() {
            window_chars.insert(c);
            //print!("{c}");
        }
        
        //println!("  {}", window_chars.len());

        if window_chars.len() == 4 {
            println!("Number of processed chars is {}", i+4);
            break;
        }

    }
}
