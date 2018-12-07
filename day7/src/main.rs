#![feature(test)]
extern crate test;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use regex::Regex;
use std::collections::HashSet;


type Pair = (char,char);

fn format_input(input_str: &str) -> HashSet<Pair> {
    let re_before: Regex = Regex::new(r"Step \w").unwrap();
    let re_after: Regex = Regex::new(r"step \w").unwrap();
    let mut result: HashSet<Pair> = HashSet::new();

    for l in input_str.lines() {
        result.insert((re_before.captures(l).unwrap()[0].chars().last().unwrap(),
                     re_after.captures(l).unwrap()[0].chars().last().unwrap()));
        }
    result
}

fn get_domain(input: &HashSet<Pair>) -> Vec<char> {
    let mut temp: HashSet<char> = HashSet::new();
    for i in input {
        temp.insert(i.0);
        temp.insert(i.1);
    }
    let mut domain: Vec<char> = Vec::new();
    for t in temp {
        domain.push(t);
    }
    domain.sort_by_key(|k| (*k as u8));
    domain
}

// fn compute_transitive_closure(input: &HashSet<Pair>) -> HashSet<Pair> {
//     let mut wo: HashSet<Pair> = input.clone();
//     let mut finished = false;
//     while !finished {
//         finished = true;
//         for a in wo.clone() {
//             for b in wo.clone() {
//                 if a.1 == b.0 && !wo.contains(&(a.0,b.1)) {
//                     wo.insert((a.0,b.1));
//                     finished = false;
//                 }
//             }
//         }
//     }
//     wo
// }

// fn is_less_than(x: char, y: char, trcl: &HashSet<Pair>) -> bool{
//     x != y && trcl.contains(&(x,y))
//}

fn is_available(d: &char, domain: &Vec<char>, ignore_conflict: &Vec<char>, input: &HashSet<Pair>) -> bool {
    if ignore_conflict.contains(d) {
        return false;
    }
    for c in domain {
        if input.contains(&(*c,*d)) && !ignore_conflict.contains(c) {
            return false;
        }
    }
    true
}

fn order(domain: &Vec<char>, input: &HashSet<Pair>)  -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    let mut n = domain.len();
    while n > 0 {
        for d in domain {
            if is_available(d, domain, &result, input) {
                result.push(*d);
                n -=1;
                break;
            }
        }
    }
    result
}

fn solve_part_1(input: &HashSet<Pair>) -> String {
    let domain = get_domain(&input);
    let mut answer = String:: new();
    
    for a in order(&domain,&input) {
        answer.push(a);
    }
    answer
}

fn main() {
    let filename = "../inputs/day7.txt";
    let mut input_str = String::new();
    File::open(filename).expect("Error reading file")
        .read_to_string(&mut input_str).expect("Error reading file");
    let input = format_input(&input_str);

    println!("Answer part 1: {}", solve_part_1(&input));
    
}

#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;

    
}
