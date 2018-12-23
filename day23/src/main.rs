#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
const INPUT: &str = include_str!("../input.txt");

type Point = (i32, i32, i32);

#[derive(Clone)]
struct Nanobot {
    pos: Point,
    range: u32,
}

impl Nanobot {
    fn print(&self) {
        println!(
            "pos=<{},{},{}>, r={}",
            self.pos.0, self.pos.1, self.pos.2, self.range
        );
    }

    fn dist(&self, other: &Nanobot) -> u32 {
        ((self.pos.0 - other.pos.0).abs()
            + (self.pos.1 - other.pos.1).abs()
            + (self.pos.2 - other.pos.2).abs()) as u32
    }
}

fn reachable_bots(nanobots: &Vec<Nanobot>, bot: &Nanobot) -> u32 {
    let mut in_range: u32 = 0;
    for b in nanobots {
        if bot.dist(&b) < bot.range {
            in_range += 1;
        }
    }
    in_range
}

fn format_input(input_str: &str) -> Vec<Nanobot> {
    lazy_static! {
        static ref RE_NB: Regex =
            Regex::new(r"<(?P<x>-?[0-9]+),(?P<y>-?[0-9]+),(?P<z>-?[0-9]+)>, r=(?P<range>[0-9]+)")
                .unwrap();
    }
    let mut nanobots: Vec<Nanobot> = Vec::new();
    for line in input_str.lines() {
        match RE_NB.captures(line) {
            Some(cap) => {
                nanobots.push(Nanobot {
                    pos: (
                        cap.name("x")
                            .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
                        cap.name("y")
                            .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
                        cap.name("z")
                            .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
                    ),
                    range: cap
                        .name("range")
                        .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
                });
            }
            None => {
                panic!("Couldn't parse {}", input_str);
            }
        }
    }
    nanobots
}

fn solve_part_1(input_str: &str) -> u32 {
    let nanobots: Vec<Nanobot> = format_input(input_str);
    let mut index: usize = 0;
    let mut max_range: u32 = 0;
    for i in 0..nanobots.len() {
        if nanobots[i].range > max_range {
            max_range = nanobots[i].range;
            index = i;
        }
    }
    reachable_bots(&nanobots, &nanobots[index])
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
}
