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


fn main() {

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    let mut sensors: Vec<Vec<i32>> = Vec::new();
    let mut row_beacons: HashSet<i32> = HashSet::new();
    
    let re = Regex::new(r"(\d+)").unwrap();
    let row = 2000000;

    let mut seen = 0;
    let mut xmin = i32::MAX;
    let mut xmax = i32::MIN;
    //let mut ymin = i32::MAX;
    //let mut ymax = i32::MIN;

    for line in input {

        if line.len() == 0 { break; }

        let mut nums: Vec<i32> = re.find_iter(line).map(|n| n.as_str().parse().unwrap()).collect();
        nums.push(manhatten((nums[0], nums[1]), (nums[2], nums[3])) as i32);

        if nums[0]-nums[4] < xmin || nums[2]-nums[4] < xmin {
            xmin = cmp::min(nums[0], nums[2]) - nums[4];
        }

        if nums[0]+nums[4] > xmax || nums[2]+nums[4] > xmax {
            xmax = cmp::max(nums[0], nums[2]) + nums[4];
        }

        if nums[3] == row && !row_beacons.contains(&nums[3]) {
            seen -= 1;
            row_beacons.insert(nums[3]);
        }

        if i32::abs(nums[3] - row) <= nums[4] {
            sensors.push(nums);
        }
    }

    for spot in xmin..(xmax+1) {
        
        let mut sensor_found = false;
        
        for s in &sensors {
            
            let dist = manhatten((spot, row), (s[0], s[1])) as i32;
            
            if dist == 0 { break; }

            if dist <= s[4] {
                sensor_found = true;
                break;
            }
        }

        if sensor_found {
            seen += 1;
        }
    }

    println!("Beacon-free locations: {seen}");
}
