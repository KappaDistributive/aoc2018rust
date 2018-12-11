extern crate regex;
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::cmp;
use std::ops;
use std::i32;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

}

#[derive(Clone, Copy, Debug)]
struct Vector {
    dx: i32,
    dy: i32,
}

impl Vector {
    fn new(dx: i32, dy: i32) -> Self {
        Vector { dx, dy}
    }
}

#[derive(Clone, Copy, Debug)]
struct Light {
    pos: Point,
    vel: Vector,
}

impl Light {
    fn step(&mut self) {
        self.pos += self.vel;
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;
    
    fn add(self, v: Vector) -> Self::Output {
        Point {
            x: self.x + v.dx,
            y: self.y + v.dy,
        }
    }

}

impl ops::AddAssign<Vector> for Point {
    fn add_assign(&mut self, v: Vector) {
        *self = *self + v;
    }
}

const INPUT: &str = include_str!("../input.txt");

/// parse input_str into a suitable data structure
fn format_input(input_str: &str) -> Vec<Light> {
    let mut lights: Vec<Light> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?[0-9]+)").unwrap();
    }

    for line in input_str.trim().lines() {
        let mut nums: Vec<i32> = Vec::new();
        for c in RE.captures_iter(line) {
            nums.push(c[0].parse::<i32>().unwrap());
        }
        if nums.len() != 4 {
            panic!("Couldn't parse: {}", line);
        }
        let pos = Point::new(nums[0], nums[1]);
        let vel = Vector::new(nums[2], nums[3]);
        lights.push(Light{ pos, vel });
    }
    lights
}

/// returns the maximal x and y values in lights
fn border_lights(lights: &Vec<Light>) -> (i32,i32,i32,i32) {
    let upper = lights.iter().map(|p| p.pos.y).min().unwrap();
    let lower = lights.iter().map(|p| p.pos.y).max().unwrap();
    let left = lights.iter().map(|p| p.pos.x).min().unwrap();
    let right = lights.iter().map(|p| p.pos.x).max().unwrap();

    (upper, lower, left, right)
}

/// prints lights as in the problem description
fn print_lights(lights: &Vec<Light>) {
    let (upper,lower,left,right) = border_lights(lights);
    let height = (lower - upper) as usize + 1;
    let width = (right - left) as usize + 1;
    let mut state = vec![vec![false;width];height];

    for (x,y) in lights.iter()
        .map(|l| ((l.pos.x - left) as usize, (l.pos.y - upper) as usize)) {
        state[y][x] = true;
    }

    for line in state.iter().map(|row| {
        row.iter().map(|p| if *p { "#"} else { "." }).collect::<String>()
    }) {
        println!("{}", line);
    }
}

fn solve(input_str: &str) {
    let mut lights: Vec<Light> = format_input(INPUT);
    let limit = 20000;
    let mut i = 1;
    let b: (i32,i32,i32,i32) = border_lights(&lights);
    let (upper,lower,left,right) = b;
    let mut benchmark = (lower - upper) + (right - left);
    for _ in 0..limit {
        i += 1;
        lights.iter_mut().for_each(Light::step);
        let (t_upper, t_lower, t_left, t_right) = border_lights(&lights);
        if (t_lower - t_upper) + (t_right - t_left) < benchmark {
            benchmark = (t_lower - t_upper) + (t_right - t_left) + 100;
        }
        else {
            break;
        }
        // I've obtained the correct bound for benchmark by trial and
        // error. This part of the code could certainly be improved by
        // adding some metric to calculate the upper bound instead of
        // guessing it.
        if benchmark < 200 {
                print_lights(&lights);
            println!("i = {}", i);
        }
    }
}


fn main() {
    solve(INPUT);
}
