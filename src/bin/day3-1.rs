use std::fs;
use std::str;
use std::collections::HashSet;

fn main() {

    let input_path = "../input.txt";
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");
    let mut priorities = 0;

    for line in input {
        let sub_len = line.len() / 2;

        let mut bag_half: HashSet<char> = HashSet::new();

        for item in line[..sub_len].chars() {
            bag_half.insert(item);
        }

        for item in line[sub_len..].chars() {
            if bag_half.contains(&item) {
                let value = prioritize(item);
                priorities += value;
                println!("{item} = {value} -> {priorities}");
                break;
            }
        }
    }

    println!("Priority total = {priorities}");
}

fn prioritize(item: char) -> i32 {
    if item.is_ascii_lowercase() {
        return item as i32 - 96;
    } else if item.is_ascii_uppercase() {
        return item as i32 - 38;
    } else {
        return 0;
    }
}
