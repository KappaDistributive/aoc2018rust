extern crate regex;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;


type Point = (i32,i32);
type Entry = (i32,Point,Point);



fn process(filename:&str){
    let result:Vec<Entry> = Vec::new();
    let mut f = File::open(filename).expect("Error reading file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");
    for line in contents.lines() {
        retrieve_id(&line);
    }    
}

fn retrieve_id(line:&str) -> &str {
    let id_regex = Regex::new(r"\#([0-9]*)").unwrap();
    match id_regex.captures(line) {
        Some(caps) => return caps.get(0).unwrap().as_str(),
        None => return ""
    }

}

fn formatting(line:&str) {
    
    println!("{}", line);
}

fn main() {
    //process("../inputs/day3.txt");
    println!("{}",retrieve_id("#1350 @ 574,133: 20x12"));
}
