#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::string::String;
use regex::Regex;
const INPUT: &str = include_str!("../input.txt");

#[derive(Clone)]
struct RegisterMachine {
    register: Vec<usize>,
}

struct Sample {
    register_before: Vec<usize>,
    opcode: Vec<usize>,
    register_after: Vec<usize>,
}

impl RegisterMachine {

    fn new(register_size: usize) -> Self {
        let register: Vec<usize> = vec![0usize; register_size];

        RegisterMachine { register }
    }

    fn from_str(register: &str) -> Self {
        match register_from_str(register) {
            Some(register) => {
                RegisterMachine { register }
            }
            None => {
                let register: Vec<usize> = Vec::new();
                RegisterMachine { register }
            }
        }
    }

    fn from_register(input: &Vec<usize>) -> Self {
        let register: Vec<usize> = input.clone();
        RegisterMachine { register }
    }

    fn print_register(&self) {
        let mut output: String = String::new();
        
        for r in self.register.clone() {
            if output.len() == 0 {
                output.push('[');
            }
            else {
                output.push(',');
                output.push(' ');
            }
            output.push_str(&r.to_string());
        }
            output.push(']');
        println!("{}", output);
    }

    fn addr(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] = self.register[a] + self.register[b];
    }

    fn addi(&mut self, a: usize, b: usize, c:usize) {
        self.register[c] = self.register[a] + b;
    }

    fn mulr(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] = self.register[a] * self.register[b];
    }

    fn muli(&mut self, a: usize, b: usize, c:usize) {
        self.register[c] = self.register[a] * b;
    }

    fn banr(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] = self.register[a] & self.register[b];
    }

    fn bani(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] = self.register[a] & b;
    }

    fn borr(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] = self.register[a] | self.register[b];
    }

    fn bori(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] = self.register[a] | b;
    }

    fn setr(&mut self, a:usize, c:usize) {
        self.register[c] = self.register[a];
    }

    fn seti(&mut self, a: usize, c:usize) {
        self.register[c] = a;
    }

    fn gtir(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] =
            if a > self.register[b] {
                1
            }
        else {
            0
        };
    }

    fn gtri(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] =
            if self.register[a] > b {
                1
            }
        else {
            0
        };
    }

    fn gtrr(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] =
            if self.register[a] > self.register[b] {
                1
            }
        else {
            0
        };
    }

    fn eqir(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] =
            if a == self.register[b] {
                1
            }
        else {
            0
        };
    }

    fn eqri(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] =
            if self.register[a] == b {
                1
            }
        else {
            0
        };
    }

    fn eqrr(&mut self, a: usize, b: usize, c: usize) {
        self.register[c] =
            if self.register[a] == self.register[b] {
                1
            }
        else {
            0
        };
    }
}

impl Sample {
    fn from_str(sample: &str) -> Option<Self> {
        lazy_static!{
            static ref RE_SAMPLE: Regex =
                Regex::new(r"Before:\s(?P<register_before>\[[\s0-9,-]+\])\n(?P<opcode>[-\s0-9]+)\nAfter:\s{2}(?P<register_after>\[[\s0-9,-]+\])\n(?P<new_line>)").unwrap();
        }
        let mut register_before: Vec<usize> = Vec::new();
        let mut opcode: Vec<usize> = Vec::new();
        let mut register_after: Vec<usize> = Vec::new();
        
        match RE_SAMPLE.captures(sample) {
            Some(cap) => {
                match register_from_str(&cap.name("register_before").map_or("", |m| m.as_str())) {
                    Some(r) => {
                        register_before = r;
                    }
                    None => {
                        
                    }
                }
                opcode = opcode_from_str(&cap.name("opcode").map_or("", |m| m.as_str()));
                
                match register_from_str(&cap.name("register_after").map_or("", |m| m.as_str())) {
                    Some(r) => {
                        register_after = r;
                    }
                    None => {
                        
                    }
                }
                Some(Sample { register_before, opcode, register_after })
            }
            
            None => {
                None
            }
        }
    }

    fn print(&self) {
        print!("Before: ");
        let mut output_before: String = String::new();
        for r in self.register_before.clone() {
            if output_before.len() == 0 {
                output_before.push('[');
            }
            else {
                output_before.push(',');
                output_before.push(' ');
            }
            output_before.push_str(&r.to_string());
        }
        output_before.push(']');
        println!("{}", output_before);

        let mut output_opcode: String = String::new();
        print!("Opcode: ");
        let mut output_opcode: String = String::new();
        for r in self.opcode.clone() {
            output_opcode.push(' ');
            output_opcode.push_str(&r.to_string());
        }
        println!("{}", output_opcode);

        print!("After:  ");
        let mut output_after: String = String::new();
        for r in self.register_after.clone() {
            if output_after.len() == 0 {
                output_after.push('[');
            }
            else {
                output_after.push(',');
                output_after.push(' ');
            }
            output_after.push_str(&r.to_string());
        }
        output_after.push(']');
        println!("{}", output_after);
    }
}

fn opcode_from_str(opcode :&str) -> Vec<usize> {
    opcode.split(" ").map(|m| m.parse::<usize>().unwrap()).collect()
}

fn register_from_str(register: &str) -> Option<Vec<usize>> {
    lazy_static!{
        static ref RE_REGISTER: Regex =
            Regex::new(r"\[(?P<zero>-?[0-9]+), (?P<one>-?[0-9]+), (?P<two>-?[0-9]+), (?P<three>-?[0-9]+)\]").unwrap();
    }
        
    match RE_REGISTER.captures(register) {
        Some(cap) => {
            let zero = cap.name("zero").map_or(0usize, |m| m.as_str().parse::<usize>().unwrap());
            let one = cap.name("one").map_or(0usize, |m| m.as_str().parse::<usize>().unwrap());
            let two = cap.name("two").map_or(0usize, |m| m.as_str().parse::<usize>().unwrap());
            let three = cap.name("three").map_or(0usize, |m| m.as_str().parse::<usize>().unwrap());        
            let result: Vec<usize> = [zero, one, two, three].to_vec();
            Some(result)
        }
        None => {
            None
        }

    }
}

fn samples_from_str(input_str: &str) -> Vec<Sample> {
    let mut samples: Vec<Sample> = Vec::new();
    let mut buffer: String = String::new();
    for (i, line) in input_str.trim().lines().enumerate() {
        buffer.push_str(line);
        buffer.push('\n');
        if i % 4 == 3 {
            match Sample::from_str(&buffer) {
                Some(sample) => {
                    samples.push(sample);
                    buffer = String::new();
                }
                None => {
                    buffer = String::new();
                    continue;
                }
            }
        }
    }
    samples
}

fn solve_part_1(input_str: &str) -> u32 {
    let samples = samples_from_str(input_str);
    let mut matches: Vec<u32> = Vec::new();
    for sample in samples {
        let mut r: RegisterMachine = RegisterMachine::from_register(&sample.register_before);
        let mut matching: u32 = 0;
        r.addr(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.addi(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.mulr(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.muli(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.banr(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.bani(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.borr(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.bori(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.setr(sample.opcode[1], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.seti(sample.opcode[1], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.gtir(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.gtri(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.gtrr(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.eqir(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.eqri(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        r = RegisterMachine::from_register(&sample.register_before);
        r.eqrr(sample.opcode[1], sample.opcode[2], sample.opcode[3]);
        if r.register == sample.register_after {
            matching += 1;
        }
        matches.push(matching);
    }
    matches.iter().map(|m|
                       if *m > 2 {
                           1
                       }
                       else {
                           0
                       }).sum()
}

fn solve_part_2(input_str: &str) -> usize {

    0
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registermachine_from_str() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        assert_eq!(m.register, vec![0,1,2,3]);
        
        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        assert_eq!(m.register, vec![0,-1,-2,-3]);

        m = RegisterMachine::from_str("[0, -1, 07, -03]");
        assert_eq!(m.register, vec![0,-1,7,-3]);
    }

    #[test]
    fn test_addr() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.addr(1,2,0);
        assert_eq!(m.register, vec![3,1,2,3]);
        
        m.addr(0,0,0);
        assert_eq!(m.register, vec![6,1,2,3]);

        m.addr(0,1,3);
        assert_eq!(m.register, vec![6,1,2,7]);

        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        m.addr(1,2,0);
        assert_eq!(m.register, vec![-3,-1,-2,-3]);
    }

    #[test]
    fn test_addi() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.addi(1,1,0);
        assert_eq!(m.register, vec![2,1,2,3]);

        m.addi(3,-7,3);
        assert_eq!(m.register, vec![2,1,2,-4]);

        m.addi(0,0,0);
        assert_eq!(m.register, vec![2,1,2,-4]);

        m.addi(1,5,3);
        assert_eq!(m.register, vec![2,1,2,6]);
    }

        #[test]
    fn test_mulr() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.mulr(1,2,0);
        assert_eq!(m.register, vec![2,1,2,3]);
        
        m.mulr(2,2,1);
        assert_eq!(m.register, vec![2,4,2,3]);

        m.mulr(0,1,3);
        assert_eq!(m.register, vec![2,4,2,8]);

        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        m.mulr(1,2,0);
        assert_eq!(m.register, vec![2,-1,-2,-3]);
    }

    #[test]
    fn test_muli() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.muli(1,3,0);
        assert_eq!(m.register, vec![3,1,2,3]);

        m.muli(3,-7,3);
        assert_eq!(m.register, vec![3,1,2,-21]);

        m.muli(0,0,0);
        assert_eq!(m.register, vec![0,1,2,-21]);

        m.muli(2,5,3);
        assert_eq!(m.register, vec![0,1,2,10]);
    }

    #[test]
    fn test_banr() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.banr(1,2,0);
        assert_eq!(m.register, vec![0,1,2,3]);

        m.banr(3,2,0);
        assert_eq!(m.register, vec![2,1,2,3]);

        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        m.banr(3,2,0);
        assert_eq!(m.register, vec![-2 & -3,-1,-2,-3]);
    }

    #[test]
    fn test_bani() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.bani(1,2,0);
        assert_eq!(m.register, vec![0,1,2,3]);

        m.bani(3,2,0);
        assert_eq!(m.register, vec![2,1,2,3]);

        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        m.bani(3,2,0);
        assert_eq!(m.register, vec![2 & -3,-1,-2,-3]);
    }

    #[test]
    fn test_borr() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.borr(1,2,0);
        assert_eq!(m.register, vec![3,1,2,3]);

        m.borr(1,1,0);
        assert_eq!(m.register, vec![1,1,2,3]);

        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        m.borr(3,2,0);
        assert_eq!(m.register, vec![-2 | -3,-1,-2,-3]);
    }

    #[test]
    fn test_bori() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.bori(1,2,0);
        assert_eq!(m.register, vec![3,1,2,3]);

        m.bori(2,4,0);
        assert_eq!(m.register, vec![6,1,2,3]);

        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        m.bori(3,2,0);
        assert_eq!(m.register, vec![2 | -3,-1,-2,-3]);
    }

    #[test]
    fn test_setr() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.setr(1,0);
        assert_eq!(m.register, vec![1,1,2,3]);

        m.setr(3,1);
        assert_eq!(m.register, vec![1,3,2,3]);

        m.setr(0,2);
        assert_eq!(m.register, vec![1,3,1,3]);
    }

    #[test]
    fn test_seti() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.seti(1,0);
        assert_eq!(m.register, vec![1,1,2,3]);

        m.seti(3,1);
        assert_eq!(m.register, vec![1,3,2,3]);

        m.seti(0,2);
        assert_eq!(m.register, vec![1,3,0,3]);
    }

    #[test]
    fn test_gtir() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.gtir(2,1,0);
        assert_eq!(m.register, vec![1,1,2,3]);

        m.gtir(1,1,0);
        assert_eq!(m.register, vec![0,1,2,3]);

        m.gtir(-2,1,0);
        assert_eq!(m.register, vec![0,1,2,3]);

        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        m.gtir(2,1,0);
        assert_eq!(m.register, vec![1,-1,-2,-3]);

        m.gtir(-2,1,0);
        assert_eq!(m.register, vec![0,-1,-2,-3]);

        m.gtir(0,1,0);
        assert_eq!(m.register, vec![1,-1,-2,-3]);
    }

    #[test]
    fn test_gtri() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.gtri(1,2,0);
        assert_eq!(m.register, vec![0,1,2,3]);

        m.gtri(1,1,0);
        assert_eq!(m.register, vec![0,1,2,3]);

        m.gtri(1,-2,0);
        assert_eq!(m.register, vec![1,1,2,3]);

        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        m.gtri(1,2,0);
        assert_eq!(m.register, vec![0,-1,-2,-3]);

        m.gtri(1,-2,0);
        assert_eq!(m.register, vec![1,-1,-2,-3]);

        m.gtri(1,0,0);
        assert_eq!(m.register, vec![0,-1,-2,-3]);
    }

    #[test]
    fn test_gtrr() {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 3]");
        m.gtrr(1,2,0);
        assert_eq!(m.register, vec![0,1,2,3]);

        m.gtrr(1,1,0);
        assert_eq!(m.register, vec![0,1,2,3]);

        m.gtrr(3,1,0);
        assert_eq!(m.register, vec![1,1,2,3]);

        m = RegisterMachine::from_str("[0, -1, -2, -3]");
        m.gtrr(1,2,0);
        assert_eq!(m.register, vec![1,-1,-2,-3]);

        m.gtrr(1,3,0);
        assert_eq!(m.register, vec![1,-1,-2,-3]);

        m.gtrr(0,1,0);
        assert_eq!(m.register, vec![1,-1,-2,-3]);
    }

    #[test]
    fn test_eqir () {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 2]");
        m.eqir(1,2,0);
        assert_eq!(m.register, vec![0,1,2,2]);

        m.eqir(1,1,0);
        assert_eq!(m.register, vec![1,1,2,2]);

        m.eqir(2,2,0);
        assert_eq!(m.register, vec![1,1,2,2]);
    }

    #[test]
    fn test_eqri () {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 2]");
        m.eqri(1,2,0);
        assert_eq!(m.register, vec![0,1,2,2]);

        m.eqri(1,1,0);
        assert_eq!(m.register, vec![1,1,2,2]);

        m.eqri(2,2,0);
        assert_eq!(m.register, vec![1,1,2,2]);
    }

    #[test]
    fn test_eqrr () {
        let mut m: RegisterMachine = RegisterMachine::from_str("[0, 1, 2, 2]");
        m.eqrr(1,2,0);
        assert_eq!(m.register, vec![0,1,2,2]);

        m.eqrr(1,1,0);
        assert_eq!(m.register, vec![1,1,2,2]);

        m.eqrr(2,3,0);
        assert_eq!(m.register, vec![1,1,2,2]);
    }

    #[test]
    fn test_opcode_from_str () {
        let mut opcode: Vec<usize> = opcode_from_str("1 2 3 4");
        assert_eq!(opcode, vec![1,2,3,4]);

        opcode = opcode_from_str("-1 -2 -3 -4");
        assert_eq!(opcode, vec![-1,-2,-3,-4]);
    }

    #[test]
    fn test_register_from_str () {
        let mut register: Vec<usize> = register_from_str("[1, 2, 3, 4]").unwrap();
        assert_eq!(register, vec![1,2,3,4]);

        register = register_from_str("[-1, -2, -3, -4]").unwrap();
        assert_eq!(register, vec![-1,-2,-3,-4]);
    }

    #[test]
    fn test_sample_from_str () {
        match Sample::from_str("Before: [2, 1, 1, 0]\n5 1 0 1\nAfter:  [2, 0, 1, 0]\n") {
            Some(sample) => {
                assert_eq!(sample.register_before, vec![2,1,1,0]);
                assert_eq!(sample.opcode, vec![5,1,0,1]);
                assert_eq!(sample.register_after, vec![2,0,1,0]);
            }
            None => {
                assert!(false);
            }
        }
    }
    
}
