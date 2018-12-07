use std::fs::File;
use std::io::prelude::*;
use std::cmp;
//use std::error::Error;
//use std::collections::HashSet;
type Point = (i32,i32,i32);

fn process(filename: &str) -> Vec<Point> {
    let mut f = File::open(filename).expect("Error reading file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");
    let mut coords: Vec<Point> = Vec::new();
    
    for line in contents.lines() {
        let temp: Vec<&str> = line.split(", ").collect();
        coords.push((temp[0].parse().unwrap(),temp[1].parse().unwrap(),0));
    }
    coords
}

fn has_infinite_area(p : &Point, min : &Point, max : &Point) -> bool {
    p.0 <= min.0 || p.1 <= min.1 || p.0 >= max.0 || p.1 >= max.1
}

fn main() {
    let mut coords: Vec<Point> = process("../inputs/day6.txt");

    // find borders
    let mut min :<Point> = (0,0);
    let mut max :<Point> = (0,0);
    for c in coords.clone() {
        min = (cmp::min(min.0,c.0),cmp::min(min.1,c.1));
        max = (cmp::max(max.0,c.0),cmp::max(max.1,c.1));
    }
    for x in 0..(&max.0 - &min.0) {
        for y in 0..(&max.1 - &min.1) {
            let mut closest = NULL;
            for c in coords.clone() {
                if !closest {
                    closets = c;
                }
                else {
                    if cmp::abs(&c.0-&x) + cmp::abs(&c.1-&y) < cmp::abs(&c.0-&x) + cmp::abs(&c.1-&y) {
                        closets = c;
                    }
                }
            }
            // check if minumum is strict
            for c in coords.clone() {
                if (&c.0 != &closest.0 || &c.1 != &closest.1) && cmp::abs(&c.0-&x) + cmp::abs(&c.1-&y) == cmp::abs(&c.0-&x) + cmp::abs(&c.1-&y) {
                    
                    }
                }
            }
        }
    }
        
    }
}
