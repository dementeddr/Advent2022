use std::fs;
use std::str;

fn main () {
    let path = "../input.txt";
    let input = fs::read_to_string(path).expect("Couldn't read file");
    let input: str::Split<&str> = input.split("\n");

    let mut elves: Vec<u32> = Vec::new();
    let mut cur_total = 0;

    for line in input {

        if line.len() == 0 {
            elves.push(cur_total);
            cur_total = 0;
            continue;
        }

        let num: u32 = line.parse().expect("NaN");
        cur_total += num;
    }

    elves.sort();

    for elf in &elves {
        println!("{elf}");
    }

    let mut final_total = 0;

    for _ in 0..3 {
        final_total += elves.pop().expect("no number here");
    }

    println!("Total calories between the 3 swolest elves is {final_total}");
}
