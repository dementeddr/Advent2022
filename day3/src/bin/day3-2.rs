use std::fs;
use std::str;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {

    let input_path = "../input.txt";
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");
    let mut bags = Vec::new();
    let mut priorities = 0;

    for line in input {

        bags.push(line);

        if bags.len() == 3 {
            let badge = find_badge(&bags);
            let value = prioritize(badge);
            priorities += value;
            println!("{badge} = {value} -> {priorities}");
            bags.clear();
        }
    }

    println!("Priority total = {priorities}");
}


fn find_badge(bags: &Vec<&str>) -> char {
    let mut shared: HashMap<char, u32> = HashMap::new();

    for bag in bags {

        let mut items: HashSet<char> = HashSet::new();

        for item in bag.chars() {
            items.insert(item);
        }

        for item in items {
            shared.entry(item).and_modify(|counter| *counter += 1) .or_insert(1);
        }
    }
    
    for item in shared.keys() {
        match shared.get(item) {
            Some(3) => return *item,
            Some(_) => {},
            None => println!("Invalid item? {item}"),
        }
    }

    '\0'
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
