use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;

struct Node {
    name: String,
    isDir: bool,
    parent: String,
    size: u32,
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    let mut fs: HashMap<&str, Vec<Node>> = HashMap::new();
    let mut path: Vec<&str> = Vec::new();

    for line in input {
    
        let text: Vec<&str> = line.split(" ").collect();

        if text.len() <= 1 {
            return;
        }

        let field0 = text[0];
        let field1 = text[1];
        let mut field2 = "/";

        if text.len() == 3 {
            field2 = text[2];
        }

        if field0 == "$" {

            print_path(&path);
            println!("{line}");

            if field1 == "cd" {
                if field2 == "/" {
                    path.clear();
                    path.push("/");
                } else if field2 == ".." {
                    path.pop();
                } else {
                    path.push(field2);
                }
            } else if field1 == "ls" {
                continue;
            } else {
                println!("aoc: command not found: {}", field1);
            }

        } else if field0 == "dir" {
            
            println!("{line}");

            let folder = Node {
                name: field1.to_string(),
                isDir: true,
                parent: path.last().unwrap().to_string(),
                size: 0,
            };

            add_node(&mut fs, folder);

        } else {

            println!("{line}");

            let file = Node {
                name: field1.to_string(),
                isDir: false,
                parent: path.last().unwrap().to_string(),
                size: field0.parse().unwrap(),
            };

            add_node(&mut fs, file);
        }
    }
}


fn print_path(path: &Vec<&str>) {
    for dir in path {
        print!("{}/", dir);
    }

    print!(" ");
}


fn add_node<'a>(fs: &'a mut HashMap<&'a str, Vec<Node>>, new_node: a' Node) {

    let contents: Vec::<Node> = Vec::new();

    fs.entry(&new_node.parent).or_insert(contents);
    fs.entry(&new_node.parent).and_modify(|d| d.push(new_node));
}
