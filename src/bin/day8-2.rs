use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;
use num_integer::Roots;

fn main() {

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    let mut forest: HashMap<(i32, i32), i32> = HashMap::new();
    let mut visibility: HashMap<(i32, i32), i32> = HashMap::new();

    let mut row = 0;

    // initial read
    for line in input {
       
        let mut col = 0;

        for tree in line.chars() {
            
            let coord = (row, col);
            forest.insert(coord, tree.to_digit(10).unwrap() as i32);

            col += 1;
        }

        if line.len() > 0 {
            row += 1;
        }
    }

    let size = row;

    for x in 0..size {
        let x = x as i32;
        for y in 0..size {
            let y = y as i32;
            visibility.insert((x,y), calculate_visibility(&forest, x, y));
            println!("visibility at {},{} = {}", x, y, visibility[&(x, y)]);
        }
    }

    let mut max = 0;

    for t in visibility.values() {
        if t > &max {
            max = *t;
        }
    }

    println!("Best Visiblity is {max}");
}


fn calculate_visibility(forest: &HashMap<(i32, i32), i32>, tree_x: i32, tree_y: i32) -> i32 {

    let tree = forest[&(tree_x, tree_y)];
    let size = forest.len().sqrt(); //yes we're assuming the forest is square. No I don't care.

    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    //look up
    for y in (0..tree_y).rev() {
        let look_tree = forest[&(tree_x, y)];

        up += 1;
        if look_tree >= tree {
            break;
        }
    }

    //look down
    for y in tree_y+1..size as i32 {
        let look_tree = forest[&(tree_x, y)];

        down += 1;
        if look_tree >= tree {
            break;
        }
    }

    //look left
    for x in (0..tree_x).rev() {
        let look_tree = forest[&(x, tree_y)];

        left += 1;
        if look_tree >= tree {
            break;
        }
    }

    //look right
    for x in tree_x+1..size as i32{
        let look_tree = forest[&(x, tree_y)];

        right += 1;
        if look_tree >= tree {
            break;
        }
    }

    //println!("{up} * {down} * {left} * {right}");
    up * down * left * right
}
