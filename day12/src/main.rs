extern crate regex;
#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::cmp;
use std::i32;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone)]
struct GameOfPots {
    state: HashMap<i32,bool>,
    range: (i32,i32),
    rules: HashMap<(bool,bool,bool,bool,bool),bool>,
}

impl GameOfPots {
    fn new() -> Self {
        let state: HashMap<i32,bool> = HashMap::new();
        let range: (i32,i32) = (0,0);
        let rules: HashMap<(bool,bool,bool,bool,bool),bool> = HashMap::new();        
        GameOfPots { state,range , rules }
    }

    fn from_data(input_str: &str) -> Self {
        let temp: (HashMap<i32,bool>,(i32,i32)) = get_state(input_str);
        let (state,range): (HashMap<i32,bool>,(i32,i32)) = temp;
        let rules: HashMap<(bool,bool,bool,bool,bool),bool> = get_rules(input_str);
        
        GameOfPots { state,range , rules }
    }

    fn print_state(&self) {
        for i in self.range.0..self.range.1 {
            if *self.state.get(&i).unwrap() {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();        
    }

    fn print_rules(&self) {
        for rule in self.rules.clone() {
            let (condition, result): ((bool,bool,bool,bool,bool),bool) = rule;
            let mut temp: Vec<char> = Vec::new();
            temp.push(if condition.0 {'#'} else {'.'});
            temp.push(if condition.1 {'#'} else {'.'});
            temp.push(if condition.2 {'#'} else {'.'});
            temp.push(if condition.3 {'#'} else {'.'});
            temp.push(if condition.4 {'#'} else {'.'});
            temp.push(if result {'#'} else {'.'});
            println!("{}{}{}{}{} => {}", temp[0],temp[1],temp[2],temp[3],temp[4],temp[5]);
        }
    }

    fn step(& mut self) {
        let mut t_state: HashMap<i32,bool> = HashMap::new();
        for i in (self.range.0 - 1)..(self.range.1 + 1) {
            let mut condition: Vec<bool> = Vec::new();
            for j in (i-2)..=(i+2) {
                condition.push( match self.state.get(&j) {
                    Some(b) => {
                        *b
                    }
                    None => {
                        false
                    }
                }); 
            }
            match self.rules.get(&(condition[0],condition[1],condition[2],condition[3],condition[4])) {
                Some(b) => {
                    t_state.insert(i,*b);
                    self.range.0 = cmp::min(self.range.0, i);
                    self.range.1 = cmp::max(self.range.1, i+1);
                }
                None => {
                    t_state.insert(i,false);
                    self.range.0 = cmp::min(self.range.0, i);
                    self.range.1 = cmp::max(self.range.1, i+1);
                }
            }
        }
        self.state = t_state;
    }

    fn score(&self) -> i32 {
        let mut result: i32 = 0;
        for (id,plant) in self.state.iter() {
            if *plant {
                result += id;
            }
        }
        result
    }
    
}

fn get_rules(input_str: &str) -> HashMap<(bool,bool,bool,bool,bool),bool> {
    lazy_static!{
        static ref RE_rules: Regex = Regex::new(r"(?P<con>[#.]{5}) => (?P<res>[#.])").unwrap();
    }
    let mut rules: HashMap<(bool,bool,bool,bool,bool),bool> = HashMap::new();
    for line in input_str.trim().lines() {
        match RE_rules.captures(line) {
            Some(caps) => {
                let result_str = caps.name("res").map_or("", |p| p.as_str());
                let condition_str = caps.name("con").map_or("",|p| p.as_str());
                let result: bool = result_str == "#";
                let condition: Vec<bool> = condition_str.chars().map(|c| c == '#').collect();
                rules.insert((condition[0], condition[1], condition[2], condition[3], condition[4]), result);
            }
            None => {
                continue;
            }
            
        }
    }
    rules
}

fn get_state(input_str: &str) -> (HashMap<i32,bool>,(i32,i32)) {
    lazy_static!{
        static ref RE_initial: Regex = Regex::new(r"initial state: ([#.]+)").unwrap();
    }
    let mut state: HashMap<i32,bool> = HashMap::new();
    let mut range: (i32,i32) = (0,0);
    let mut pos: i32 = 0;
    
    for  line in input_str.trim().lines() {
        for cap in RE_initial.captures(line) {
            for c in cap.get(1).map_or("",|p| p.as_str()).chars() {
                if c == '#' {
                    state.insert(pos,true);
                }
                else if c == '.' {
                    state.insert(pos,false);
                }
                else {
                    panic!("Couldn't parse {}", line);
                }
                pos += 1;
                range.1 +=1;
            }
        }
    }
    (state, range)
}

fn solve_part_1(input_str: &str) -> i32 {
    let mut game = GameOfPots::from_data(input_str);
    for _ in 0..20 {
        game.step();
    }
    game.score()
}

fn solve_part_2(input_str: &str) -> u64 {
    let mut game = GameOfPots::from_data(input_str);
    // iterate far enough out so that the plant pattern, up to
    // shifting, becomes constant
    for _ in 0..100 {
        game.step();
    }
    // inc is the value by which the score increases in each iteration
    // from here on
    let inc: u64 = game.state.clone().iter().map(|(_,plant)| if *plant { 1 } else { 0 }).sum();
    let mut result: u64 = game.score() as u64;
    result += inc * (50000000000-100);
    result
}


fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
