use std::fs;
use std::str;

fn main() {

    let input_path = "../input.txt";
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    for line in input {

    }
}
