use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    let mut fs: HashMap<String, u32> = HashMap::new();
    let mut path: Vec<String> = Vec::new();

    path.push("/".to_string());
    fs.insert(path_to_string(&path), 0);

    for line in input {
    
        let text: Vec<&str> = line.split(" ").collect();

        if text.len() <= 1 {
            break;
        }

        let field0 = text[0];
        let field1 = text[1];
        let mut field2 = "\0";

        if text.len() == 3 {
            field2 = text[2];
        }

        if field0 == "$" {

            print!("{line} -> ");

            if field1 == "cd" {
                if field2 == "/" {
                    path.clear();
                    path.push("/".to_string());
                } else if field2 == ".." {
                    path.pop();
                } else {
                    path.push(field2.to_string());
                }
                print_path(&path);
            } else if field1 == "ls" {
                print_path(&path);
                continue;

            } else {
                println!("aoc: command not found: {}", field1);
            }

        } else if field0 == "dir" {
            let fullpath = path_to_string(&path);
            let fullpath = format!("{fullpath}{field1}/");
            println!("dir {fullpath}");
            
            if !fs.contains_key(&fullpath) {
                fs.insert(fullpath, 0);
            }
            //print_keys(&fs);

        } else {
           
            println!("{field1}:  {field0}");
           
            let size: u32 = field0.parse().unwrap();
            let mut part_path: Vec<String> = Vec::new(); 

            for dir in &path {
                
                part_path.push(String::from(dir));
                let substring = path_to_string(&part_path);
                let cur_size = fs.get_mut(&substring).unwrap();
                
                *cur_size = size + *cur_size;
            }
        }
    }

    println!();

    let mut total = 0;

    for (dir, size) in fs.iter() {

        if *size <= 100000 {
            total += *size;
            println!("{dir}={size} total={total}");
        }
    }

    println!("\nFinal Total : {total}");
}


fn path_to_string(path: &Vec<String>) -> String {
   
    let mut fullpath = String::from("");
   
    for dir in path {
        fullpath = format!("{fullpath}{dir}/");
    }

    fullpath
}

fn print_path(path: &Vec<String>) {

    let fullpath = path_to_string(path);
    println!("{}", fullpath);
}

fn print_keys(map: &HashMap<String, u32>) {
    
    println!("- PRINTING MAP -");

    for (key, val) in map.iter() {
        println!("- {key} -> {val}");
    }
}
