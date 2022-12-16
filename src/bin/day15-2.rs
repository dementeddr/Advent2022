use std::env;
use std::fs;
use std::str;
use std::cmp;
use std::collections::HashSet;
use regex::Regex;


fn manhatten(first: (i32, i32), second: (i32, i32)) -> u32 {
    let dx = i32::abs(first.0 - second.0) as u32;
    let dy = i32::abs(first.1 - second.1) as u32;

    dx + dy
}


fn find_sensor(coords: (i32, i32), sensors: &Vec<Vec<i32>>) -> bool {
    
    for s in sensors {
        let dist = manhatten(coords, (s[0], s[1])) as i32;

        if dist <= s[4] {
            return true;
        }
    }

    false
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    let mut sensors: Vec<Vec<i32>> = Vec::new();
    let mut unseen: HashSet<(i32, i32)> = HashSet::new();
    
    let size = 4000000;
    let re = Regex::new(r"(\d+)").unwrap();

    //let mut xmin = i32::MAX;
    //let mut xmax = i32::MIN;
    //let mut ymin = i32::MAX;
    //let mut ymax = i32::MIN;

    for line in input {

        if line.len() == 0 { break; }

        let mut nums: Vec<i32> = re.find_iter(line).map(|n| n.as_str().parse().unwrap()).collect();
        nums.push(manhatten((nums[0], nums[1]), (nums[2], nums[3])) as i32);

        sensors.push(nums);
    }

    let mut beacon = (-1,-1);

    for x in 0..(size+1) {
        
        let mut beacon_found = false;

        if x % 10 == 0 {
            print!("\n row: {x}");
        //} else if x % 1000 == 0 {
        } else {
            print!(".");
        }
        
        for y in 0..(size+1) {

            if !find_sensor((x, y), &sensors) {
                beacon_found = true;
                beacon = (x,y);
                break;
            }
        }

        if beacon_found { break; }
    }

    println!("Beacon in distress is at {:?}", beacon);
}
