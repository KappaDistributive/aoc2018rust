#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate image;

use image::{ImageBuffer, Rgb};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::usize;
use std::cmp;

type Point = (usize, usize);

const INPUT: &str = include_str!("../input.txt");

struct Scan {
    scan: HashMap<Point,char>,
    active: HashSet<Point>,
    min_y: usize,
    max_y: usize,
}

impl Scan {
    fn from_data(input: &HashMap<Point,char>) -> Self {
        let mut min_y: usize = usize::MAX;
        let mut max_y: usize = 0;
        for ((_,y),_) in input.iter() {
            min_y = cmp::min(min_y, *y);
            max_y = cmp::max(max_y, *y);
        }
        let mut scan: HashMap<Point,char> = input.clone();
        scan.insert((500,0),'+');
        let mut active: HashSet<Point> = HashSet::new();
        active.insert((500,0));
        for ((x,y),c) in input.iter() {
            if *c == '|' {
                active.insert((*x,*y));
            }
        }
        
        Self { scan, active, min_y, max_y }
    }
    

    fn water_count(&self) -> usize {
        let mut count: usize = 0;
        for ((_,y),c) in self.scan.clone() {
            if (self.min_y <= y && y <= self.max_y) && (c == '~' || c == '|' || c == '+') {
                count += 1;
            }
        }
        count
    }

    fn step(&mut self) {
        
    }

    fn print(&self) {
        let mut min_x: usize = usize::MAX;
        let mut max_x: usize = 0;
        for ((x,_),_) in self.scan.clone() {
            min_x = cmp::min(min_x, x);
            max_x = cmp::max(max_x, x);
        }

        for y in 0..=self.max_y {
            for x in min_x..=max_x {
                match self.scan.get(&(x,y)) {
                    Some(c) => {
                        print!("{}",c);
                    }
                    None => {
                        print!(".");
                    }
                }
            }
            println!();
        }
    }

    fn print_png(&self, path: &str) {
        let mut min_x: usize = usize::MAX;
        let mut max_x: usize = 0;
        for ((x,_),_) in self.scan.clone() {
            min_x = cmp::min(min_x, x);
            max_x = cmp::max(max_x, x);
        }
        let width: u32 = (max_x - min_x + 1) as u32;
        let height: u32 = (self.max_y + 1) as u32;
        
        let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
        
        let flowing_water: [u8;3] = [0,200,200];
        let still_water: [u8;3] = [0,50,200];
        let clay: [u8;3] = [200,200,200];
        let background: [u8;3] = [50,50,50];
        let source: [u8;3] = [200,0,0];
        
        for y in 0..=self.max_y {
            for x in min_x..=max_x {
                match self.scan.get(&(x,y)) {
                    Some(c) => {
                        if *c == '#' {
                            image.get_pixel_mut((x - min_x) as u32,y as u32).data = clay;
                        }
                        if *c == '|' {
                            image.get_pixel_mut((x - min_x) as u32,y as u32).data = flowing_water;
                        }
                        if *c == '~' {
                            image.get_pixel_mut((x - min_x) as u32,y as u32).data = still_water;
                        }
                        if *c == '+' {
                            image.get_pixel_mut((x - min_x) as u32,y as u32).data = source;
                        }
                    }
                    None => {
                        image.get_pixel_mut((x - min_x) as u32,y as u32).data = background;
                    }
                }
            }
            
        }
        image.save(Path::new(path));
    }
}

fn format_input(input: &str) -> Scan {
    lazy_static! {
        static ref RE_X: Regex = Regex::new(r"x=(?P<x>[0-9]+),\sy=(?P<y_start>[0-9]+)..(?P<y_end>[0-9]+)").unwrap();
        static ref RE_Y: Regex = Regex::new(r"y=(?P<y>[0-9]+),\sx=(?P<x_start>[0-9]+)..(?P<x_end>[0-9]+)").unwrap();
    }
    let mut scan: HashMap<Point,char> = HashMap::new();
    
    for cap in RE_X.captures_iter(input) {
        let x: usize = cap["x"].parse::<usize>().unwrap();
        let y_start: usize = cap["y_start"].parse::<usize>().unwrap();
        let y_end: usize = cap["y_end"].parse::<usize>().unwrap();

        for y in y_start..=y_end {
            scan.insert((x,y),'#');
        }
    }

    for cap in RE_Y.captures_iter(input) {
        let y: usize = cap["y"].parse::<usize>().unwrap();
        let x_start: usize = cap["x_start"].parse::<usize>().unwrap();
        let x_end: usize = cap["x_end"].parse::<usize>().unwrap();

        for x in x_start..=x_end {
            scan.insert((x,y),'#');
        }
    }
    Scan::from_data(&scan)
}

fn main() {
    let mut scan: Scan = format_input(INPUT);
    for i in 0..17 {
        scan.step();
    }
    for i in 0..5 {
        scan.step();
        scan.print_png(&format!("image{}.png",i));
    }
    println!("Answer part 1: {}",scan.water_count());
}
