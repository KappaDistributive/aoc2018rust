extern crate regex;
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::cmp;
use std::i32;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

type Point = (i32,i32);

fn format_input(input_str: &str) -> HashMap<Point,Point> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^0-9-]*(-?[0-9]+)[^0-9-]*(-?[0-9]+)[^0-9-]*(-?[0-9]+)[^0-9-]*(-?[0-9])[^0-9-]*").unwrap();
    }
    let mut result: HashMap<Point,Point> = HashMap::new();
    for line in input_str.trim().lines() {
        let mut entry: (Point,Point) = ((0,0),(0,0));
        match RE.captures(line) {
            Some(caps) => {
                entry.0 = (caps.get(1).map_or(0i32, |m| m.as_str().parse::<i32>().unwrap()),
                            caps.get(2).map_or(0i32, |m| m.as_str().parse::<i32>().unwrap()));
                entry.1 = (caps.get(3).map_or(0i32, |m| m.as_str().parse::<i32>().unwrap()),
                           caps.get(4).map_or(0i32, |m| m.as_str().parse::<i32>().unwrap()));
            }
            None => {
                panic!("Couldn't parse: {}", "bla");
            }
        }
        result.insert(entry.0, entry.1);
    }
    result
}

fn print_points(input: &HashMap<Point,Point>) {
    let mut min: Point = (-10,-10);
    let mut max: Point = (20,15);

    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            match input.get(&(x,y)) {
                Some(_) => {
                    print!("x");
                }
                None => {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

fn solve_part_1(input_str: &str) {
    let mut input: HashMap<Point,Point> = format_input(input_str);
    print_points(&input);
}

fn main() {
    solve_part_1(INPUT);
}
