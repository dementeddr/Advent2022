use std::fs;
use std::str;
use regex::Regex;

fn main() {

    let input_path = "../input.txt";
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();

    let mut count = 0;

    for line in input {
        let nums = match re.captures(line) {
            Some(x)=>x,
            None=>break,
        };

        let mut r1: [i32; 2] = [0; 2];
        let mut r2: [i32; 2] = [0; 2];

        r1[0] = nums.get(1).unwrap().as_str().parse().expect("parse problem");
        r1[1] = nums.get(2).unwrap().as_str().parse().expect("parse problem");
        
        r2[0] = nums.get(3).unwrap().as_str().parse().expect("parse problem");
        r2[1] = nums.get(4).unwrap().as_str().parse().expect("parse problem");

        if (r1[0] <= r2[0] && r1[1] >= r2[0]) ||
            (r1[0] <= r2[1] && r1[1] >= r2[1]) ||
            (r1[0] > r2[0] && r1[1] < r2[1]) {
            count += 1;
            println!("overlap: {}-{},{}-{} count={}", r1[0], r1[1], r2[0], r2[1], count);
        } else {
            println!("noverlap: {}-{},{}-{}", r1[0], r1[1], r2[0], r2[1]);
        }
    }

    println!("Final Overlap Count = {count}");
}
