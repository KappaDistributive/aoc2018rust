use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashSet;

fn distance(l: &str,r: &str) -> i32 {
    let l: Vec<char> = l.chars().collect();
    let r: Vec<char> = r.chars().collect();
    let mut dst = 0i32;
    if l.len() != r.len() {
        -1
    }
    else {
        for i in 0..r.len() {
            if l[i] != r[i] {
                dst += 1;
            }
        }
        dst
    }
}

fn part_one(input: &String) -> i32 {
    let mut factors = [0usize;4];
    for line in input.lines() {
        let mut count = [0u8;26];
        for c in line.bytes() {
            count[(c as usize)-('a' as usize)] += 1;
        }
        let count_set: HashSet<&u8> = count.iter().filter(|&n| *n > 0).collect();
        for i in count_set {
            if (*i as usize) < factors.len() {
                factors[(*i as usize)] += 1;
            }
        }
    }
    (factors[2] as i32)*(factors[3] as i32)
}

fn part_two(input: &String) -> String {
    let mut result = String::new();
    for w1 in input.lines() {
        for w2 in input.lines() {
            if distance(&w1,&w2) == 1 {
                let w1_array : Vec<char> = w1.chars().collect();
                let w2_array: Vec<char> = w2.chars().collect();
                for i in 0..w1_array.len() {
                    if w1_array[i] == w2_array[i] {
                        result.push(w1_array[i]);
                    }
                }
                return result;
            }
        }
    }
    String::from("Panic! This should never be reached.")
}

fn main() -> Result<(), Box<Error>> {
    let filename = "../inputs/day2.txt";

    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    
    println!("Answer for part 1: {}", part_one(&contents));

    println!("Answer for part 2: {}", part_two(&contents));

    Ok(())
}
