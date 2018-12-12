#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
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
    let input = format_input(input_str);
    let min_x: i32 = input.iter().map(|claim| claim.pos.0).min().unwrap();
    let max_x: i32 = input.iter().map(|claim| claim.pos.0 + claim.dim.0 as i32).max().unwrap();
    let min_y: i32 = input.iter().map(|claim| claim.pos.1).min().unwrap();
    let max_y: i32 = input.iter().map(|claim| claim.pos.1 + claim.dim.1 as i32).max().unwrap();
    let mut fabric: Vec<Vec<u32>> = vec![vec![0u32;(max_x-min_x + 1) as usize]; (max_y-min_y + 1) as usize];
    let mut result: u32 = 0;
    for claim in input {
        for y in 0..claim.dim.1 {
            for x in 0..claim.dim.0 {
                if fabric[(claim.pos.1 - min_y + y as i32) as usize][(claim.pos.0 - min_x + x as i32) as usize] == 1 {
                    result += 1;
                }
                fabric[(claim.pos.1 - min_y + y as i32) as usize ][(claim.pos.0 - min_x + x as i32) as usize] += 1;
            }
        }
    }
    result
}

fn solve_part_2(input_str: &str) -> u32 {
    let input = format_input(input_str);
    let len: usize = input.len();
    let min_x: i32 = input.iter().map(|claim| claim.pos.0).min().unwrap();
    let max_x: i32 = input.iter().map(|claim| claim.pos.0 + claim.dim.0 as i32).max().unwrap();
    let min_y: i32 = input.iter().map(|claim| claim.pos.1).min().unwrap();
    let max_y: i32 = input.iter().map(|claim| claim.pos.1 + claim.dim.1 as i32).max().unwrap();
    let mut fabric: Vec<Vec<u32>> = vec![vec![0u32;(max_x-min_x + 1) as usize]; (max_y-min_y + 1) as usize];
    let mut overlapped: HashSet<u32> = HashSet::new();
    let mut result: u32 = 0;
    for claim in input {
        for y in 0..claim.dim.1 {
            for x in 0..claim.dim.0 {
                if fabric[(claim.pos.1 - min_y + y as i32) as usize][(claim.pos.0 - min_x + x as i32) as usize] > 0 {
                    overlapped.insert(fabric[(claim.pos.1 - min_y + y as i32) as usize][(claim.pos.0 - min_x + x as i32) as usize]);
                    overlapped.insert(claim.id);
                }
                fabric[(claim.pos.1 - min_y + y as i32) as usize ][(claim.pos.0 - min_x + x as i32) as usize] = claim.id;
            }
        }
    }
    for i in 1..=len {
        if !overlapped.contains(&(i as u32)) {
            return i as u32;
        }
    }
    unreachable!();
}


fn main() {
    println!("Answert part 1: {}", solve_part_1(INPUT));
    println!("Answert part 2: {}", solve_part_2(INPUT));
}
