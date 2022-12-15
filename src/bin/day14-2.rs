use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;
use regex::Regex;

struct Cave {
    map: HashMap<(u32, u32), char>,
    xmax: u32,
    xmin: u32,
    ymax: u32,
    ymin: u32,
    sand_count: u32,
}


impl Cave {

    fn new() -> Self {
        Self {
            map: HashMap::new(),
            xmax: 0,
            ymax: 0,
            xmin: u32::MAX,
            ymin: u32::MAX,
            sand_count: 0,
        }
    }


    fn build_wall(&mut self, start: (u32, u32), end: (u32, u32)) {
        self.expand_cave(start);
        self.expand_cave(end);

        //println!("Start: {:?}  End: {:?}", start, end);

        let horiz = if start.0 < end.0 {start.0..(end.0 + 1)} else {end.0..(start.0 + 1)};

        for x in horiz {
            
            let vert = if start.1 < end.1 {start.1..(end.1 + 1)} else {end.1..(start.1 + 1)};
            for y in vert { 
                self.map.insert((x,y), '#');
            }
        }
    }


    fn expand_cave(&mut self, coords: (u32, u32)) {
        if coords.0 > self.xmax {
            self.xmax = coords.0;
        }

        if coords.0 < self.xmin { self.xmin = coords.0;
        }

        if coords.1 > self.ymax {
            self.ymax = coords.1;
        }

        if coords.1 < self.ymin {
            self.ymin = coords.1;
        }
    }


    fn print_cave(&self) {

        println!("Cave size: {}", self.map.len());
        println!("{} - {}", self.xmin, self.xmax);

        for y in self.ymin..(self.ymax+1) {

            print!("{y:>4} ");

            for x in self.xmin..(self.xmax+1) {
                let coords = (x, y);
                let val = self.get(coords);
                
                print!("{val}");
            }

            println!();
        }
    }

    // True: Sand came to rest
    // False: Sand fell out of the cave
    fn sandfall(&mut self) -> bool {
        
        let mut x = 500;
        let mut y = 0;

        loop {

            if x > self.xmax {
                self.map.insert((x+1, self.ymax), '#');
                self.xmax += 1;
            } else if x < self.xmin {
                self.map.insert((x-1, self.ymax), '#');
                self.xmin -= 1;
            }
        
            if self.get((x, y+1)) == '.' {
                y += 1;
            } else if self.get((x-1, y+1)) == '.' {
                x -= 1;
                y += 1;
            } else if self.get((x+1, y+1)) == '.' {
                x += 1;
                y += 1;
            } else {
                self.map.insert((x, y), '~');
                self.sand_count += 1;
                
                if (x, y) == (500, 0) {
                    return false;
                } else {
                    return true;
                }
            }
        }
    }


    fn get(&self, coords: (u32, u32)) -> char{
        match self.map.get(&coords) {
            Some(p) => *p,
            None => '.',
        }
    }


    fn floor_cave(&mut self) {
        
        self.ymax += 2;

        for x in (self.xmin-1)..(self.xmax+2) {
            self.map.insert((x, self.ymax), '#');
        }
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("Path to input file required");
    let input = fs::read_to_string(input_path).expect("Could't read input file");
    let input: str::Split<&str> = input.split("\n");

    let re = Regex::new(r"(\d*,\d*)").unwrap();
    let mut cave = Cave::new();

    cave.expand_cave((500, 0));
    cave.map.insert((500, 0), 'X');

    for line in input {
        let endpoints = re.find_iter(line);
        let mut prev = (0,0);

        for p in endpoints {
            
            let mut cur = p.as_str().split(',').map(|n| n.parse().unwrap());
            let cur = (cur.next().unwrap(), cur.next().unwrap());
            
            if prev != (0,0) {
                cave.build_wall(prev, cur);
            }

            prev = cur;
        }
    }

    println!("Walls built");
    cave.floor_cave();
    println!("Cave floored");
    cave.print_cave();
    
    println!("\n");

    while cave.sandfall() {
        //println!("{}", cave.sand_count);
    }

    cave.print_cave();

    println!("Amount of sand contained in cave is: {}", cave.sand_count);
}
