use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;
use std::collections::HashSet;
use core::ops::Range;

fn main() {

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    let mut forest: HashMap<(i32, i32), i32> = HashMap::new();
    let mut visible: HashSet<(i32, i32)> = HashSet::new();
    let mut row = 0;

    let mut width = 0;
    let mut height = 0;

    // initial read
    for line in input {
       
        let mut col = 0;

        for tree in line.chars() {
            
            let coord = (row, col);
            forest.insert(coord, tree.to_digit(10).unwrap() as i32);

            col += 1;
        }

        if line.len() > 0 {
            width = line.len();
            row += 1;
        }
    }

    height = row as usize;

    println!("Number of trees: {}", forest.len());
    println!("Forest Dimensions: {width} x {height}");

    // top-down pass
    for x in 0..width {

        let mut last_seen = -1;

        for y in 0..height {
            let coords = (x as i32, y as i32);
            let tree = forest[&coords];

            if tree > last_seen {
                visible.insert(coords);
                last_seen = tree;
            }
        }
    }

    println!("top-down visibility: {}", visible.len());

    // bottom-up pass
    for x in 0..width {

        let mut last_seen = -1;

        for y in (0..height).rev() {
            let coords = (x as i32, y as i32);
            let tree = forest[&coords];

            if tree > last_seen {
                visible.insert(coords);
                last_seen = tree;
            }
        }
    }

    println!("bottom-up visibility: {}", visible.len());

    // left-right pass
    for y in 0..height {

        let mut last_seen = -1;

        for x in 0..width {
            let coords = (x as i32, y as i32);
            let tree = forest[&coords];

            if tree > last_seen {
                visible.insert(coords);
                last_seen = tree;
            }
        }
    }

    println!("left-right visibility: {}", visible.len());

    // right-left pass
    for y in 0..height {

        let mut last_seen = -1;

        for x in (0..width).rev() {
            let coords = (x as i32, y as i32);
            let tree = forest[&coords];

            if tree > last_seen {
                visible.insert(coords);
                last_seen = tree;
            }
        }
    }

    println!("right-left visibility: {}", visible.len());

}

