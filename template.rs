use std::env;
use std::fs;
use std::str;
use std::collections;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    for line in input {
    
    }
}
