#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::string::String;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Sample {
    register_before: Vec<usize>,
    instruction: Vec<usize>,
    register_after: Vec<usize>,
}

impl PartialEq for Sample {
    fn eq(&self, other: &Sample) -> bool {
        self.register_before == other.register_before
            && self.instruction == other.instruction
            && self.register_after == other.register_after
    }
}

fn addr(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = register[instruction[1]] + register[instruction[2]];
}

fn addi(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = register[instruction[1]] + instruction[2];
}

fn mulr(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = register[instruction[1]] * register[instruction[2]];
}

fn muli(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = register[instruction[1]] * instruction[2];
}

fn banr(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = register[instruction[1]] & register[instruction[2]];
}

fn bani(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = register[instruction[1]] & instruction[2];
}

fn borr(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = register[instruction[1]] | register[instruction[2]];
}

fn bori(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = register[instruction[1]]  |instruction[2];
}

fn setr(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = register[instruction[1]];
}

fn seti(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = instruction[1];
}

fn gtir(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = if instruction[1] > register[instruction[2]] { 1 } else { 0 };
}

fn gtri(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = if register[instruction[1]] > instruction[2] { 1 } else { 0 };
}

fn gtrr(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = if register[instruction[1]] > register[instruction[2]] { 1 } else { 0 };
}

fn eqir(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = if instruction[1] == register[instruction[2]] { 1 } else { 0 };
}

fn eqri(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = if register[instruction[1]] == instruction[2] { 1 } else { 0 };
}

fn eqrr(instruction: &Vec<usize>, register: &mut Vec<usize>) {
    if instruction.len() != 4 || register.len() != 4 {
        panic!("Wrong number of arguments");
    }
    register[instruction[3]] = if register[instruction[1]] == register[instruction[2]] { 1 } else { 0 };
}

fn get_named_functions() -> Vec<(String, fn(&Vec<usize>, &mut Vec<usize>))> {
    let mut named_functions: Vec<(String, fn(&Vec<usize>, &mut Vec<usize>))> = Vec::new();

    named_functions.push(("addr".to_string(), addr));
    named_functions.push(("addi".to_string(), addi));
    named_functions.push(("mulr".to_string(), mulr));
    named_functions.push(("muli".to_string(), muli));
    named_functions.push(("banr".to_string(), banr));
    named_functions.push(("bani".to_string(), bani));
    named_functions.push(("borr".to_string(), borr));
    named_functions.push(("bori".to_string(), bori));
    named_functions.push(("setr".to_string(), setr));
    named_functions.push(("seti".to_string(), seti));
    named_functions.push(("gtir".to_string(), gtir));
    named_functions.push(("gtri".to_string(), gtri));
    named_functions.push(("gtrr".to_string(), gtrr));
    named_functions.push(("eqir".to_string(), eqir));
    named_functions.push(("eqri".to_string(), eqri));
    named_functions.push(("eqrr".to_string(), eqrr));
    
    named_functions
}

fn samples_from_str(input: &str) -> Vec<Sample> {
    lazy_static!{
        static ref RE_SAMPLE: Regex = Regex::new(r"Before: (?P<register_before>\[[-,\s0-9]+\])\n(?P<instruction>[\s0-9]+)\nAfter:  (?P<register_after>\[[-,\s0-9]+\])\n").unwrap();
    }
    let mut samples: Vec<Sample> = Vec::new();
    
    for cap in RE_SAMPLE.captures_iter(input) {
        let register_before: Vec<usize> = register_from_str(&cap["register_before"]);
        let instruction: Vec<usize> = cap["instruction"].split(" ").map(|m| m.parse::<usize>().unwrap()).collect();
        let register_after: Vec<usize> = register_from_str(&cap["register_after"]);
        samples.push( Sample { register_before, instruction, register_after });
    }
    samples
}

// expects a string of the form "[0, 1, 2, 3, 4]"
fn register_from_str (input: &str) -> Vec<usize> {
    lazy_static!{
        static ref RE_REGISTER: Regex = Regex::new(r"(?P<number>[0-9]+)").unwrap();
    }
    let mut result: Vec<usize> = Vec::new();
    for cap in RE_REGISTER.captures_iter(input) {
        result.push(cap["number"].parse::<usize>().unwrap());
    }
    result
}

fn solve_part_1(input: &str) -> usize {
    0
}

fn main() {
    println!("Answer part 1: {}",solve_part_1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_from_str () {
        let mut register = register_from_str(&"[0, 1, 2, 3]");
        assert_eq!(register, vec![0,1,2,3]);

        register = register_from_str(&"[5,3,2,7,0,1]");
        assert_eq!(register, vec![5,3,2,7,0,1]);
    }

    #[test]
    fn test_samples_from_str () {
        let mut test_samples: Vec<Sample> = Vec::new();
        let register_before: Vec<usize> = vec![2,1,1,0];
        let instruction: Vec<usize> = vec![5,1,0,1];
        let register_after: Vec<usize> = vec![2,0,1,0];
        test_samples.push( Sample { register_before, instruction, register_after  });
        
        let samples: Vec<Sample> = samples_from_str("Before: [2, 1, 1, 0]\n5 1 0 1\nAfter:  [2, 0, 1, 0]\n");
        assert_eq!(samples, test_samples);
    }
}
