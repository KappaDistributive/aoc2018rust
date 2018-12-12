#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp;
use std::collections::HashMap;
use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

type Point = (i32,i32);
type Dimension = (usize,usize);

struct Claim {
    id: u32,
    pos: Point,
    dim: Dimension,
}

impl Claim {
    
}

impl Claim {
    fn from_data(id: u32, pos: (i32,i32), dim: (usize,usize)) -> Self {
        Claim { id, pos, dim }
    }
}

fn format_input(input_str: &str) -> Vec<Claim> {
    let mut result: Vec<Claim> = Vec::new();
    lazy_static!{
        static ref RE: Regex = Regex::new(r"#(-?[0-9]+)\D+(-?[0-9]+),(-?[0-9]+)\D+(-?[0-9]+)x(-?[0-9]+)").unwrap();
    }
    for line in input_str.trim().lines() {
        let caps = RE.captures(line).unwrap();
        let id: u32 = caps.get(1).map_or(0u32, |c| c.as_str().parse::<u32>().unwrap());
        let x: i32 = caps.get(2).map_or(0i32, |c| c.as_str().parse::<i32>().unwrap());
        let y: i32 = caps.get(3).map_or(0i32, |c| c.as_str().parse::<i32>().unwrap());
        let w: usize = caps.get(4).map_or(0usize, |c| c.as_str().parse::<usize>().unwrap());
        let h: usize = caps.get(5).map_or(0usize, |c| c.as_str().parse::<usize>().unwrap());

        result.push(Claim::from_data(id, (x,y), (w,h)));
    }
    result
}

fn solve_part_1(input_str: &str) -> u32 {
    
}


fn main() {
    println!("{}", solve_part_1(INPUT));
}
