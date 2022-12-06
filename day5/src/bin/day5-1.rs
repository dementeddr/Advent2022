use std::fs;
use std::str;
use regex::Regex;
use std::collections::HashMap;

fn main() {

    let input_path = "../input.txt";
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    let re = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();

    let mut stacks: HashMap<i32, Vec<char>> = HashMap::new();
    let mut stack_input: Vec<&str> = Vec::new();
    let mut found = false;

    for line in input {

        if !found {
            if line == "" {
                stacks = stack_setup(&stack_input);
                found = true;
                print_stacks(&stacks);
            } else {
                stack_input.push(line);
            }
            continue;
        }

        let match_nums = match re.captures(line) {
            Some(x)=>x,
            None=>break,
        };

        let source = match_nums.get(2).unwrap().as_str().parse().unwrap();
        let target = match_nums.get(3).unwrap().as_str().parse().unwrap();
        let number = match_nums.get(1).unwrap().as_str().parse().unwrap();

        execute_move(source, target, number, &mut stacks);
        print_stacks(&stacks);
    }

    println!("Stack Toppers:");

    for i in 1..10 {
        // This is what good Rust code looks like, right?
        print!("{}", stacks.get(&i).unwrap().last().unwrap());
    }
    println!();
}


fn stack_setup(stack_input: &Vec<&str>) -> HashMap<i32, Vec<char>> {
    let mut stack_nums = Vec::new();
    let mut stacks: HashMap<i32, Vec<char>> = HashMap::new();

    for num in stack_input.last().expect("input read fail").chars() {
        if num != ' ' {
            stack_nums.push(num.to_digit(10).unwrap() as i32);
        }
    }

    for i in 0..9 {

        let key = *stack_nums.get(i).unwrap();
        let mut stack = Vec::new();

        for line in (&stack_input)[..stack_input.len()-1].iter().rev() {
            let label = line.chars().nth((i*4)+1).unwrap();
            if label != ' ' {
                stack.push(label);
            }
        }

        stacks.insert(key, stack);
    }

    stacks
}


fn execute_move(source: i32, target: i32, number: i32, stacks: &mut HashMap<i32, Vec<char>>) {

    println!("{source} -> {target} x {number}");

    for _ in 0..number {
        let mut load = '\0';
        stacks.entry(source).and_modify(|s| {load = s.pop().unwrap();});
        stacks.entry(target).and_modify(|s| s.push(load));
    }
}


fn print_stacks(stacks: &HashMap<i32, Vec<char>>) {

    println!("Printing Stacks:");

    for i in 1..10 {
        print!("{i}: ");
        for item in stacks.get(&i).unwrap().iter().rev() {
            print!("{item}");
        }
        println!();
    }
    println!();
}
