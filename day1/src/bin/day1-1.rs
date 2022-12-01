//use std::env;
use std::fs;
use std::str;

fn main () {
    let path = "../input.txt";

    let input = fs::read_to_string(path).expect("Couldn't read file");

    let input: str::Split<&str> = input.split("\n");
    let mut prev_max = 0;
    let mut cur_total = 0;

    for line in input {

        if line.len() == 0 {
            println!("total: {cur_total}\n--");
            if cur_total > prev_max {
                prev_max = cur_total;
            }
            cur_total = 0;
            continue;
        }

        let num: u32 = line.parse().expect("NaN");
        println!("{}", num);

        cur_total += num;
    }

    println!("The highest concentration of calories is {prev_max}");
}
