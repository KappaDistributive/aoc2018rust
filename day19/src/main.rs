#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::string::String;
use regex::Regex;

const INPUT: &str = include_str!("../input.txt");
const INPUT_SAMPLE: &str = include_str!("../input_sample.txt");

#[derive(Clone)]
struct RegisterMachine {
    register: [usize;6],
    ip_register: usize,
    instructions: Vec<[usize;4]>
}

impl RegisterMachine {
    fn from_str(input: &str) -> Self {
        let mut register: [usize;6] = [0,0,0,0,0,0];
        let mut instructions: Vec<[usize;4]> = Vec::new();
        let mut ip_register: usize = 0;
        match ip_from_str(input) {
            Some(i) => {
                ip_register = i;
            }
            None => {
                panic!("Couldn't parse {}", input);
            }
        }
        match instructions_from_str(input) {
            Some(i) => {
                instructions = i;
            }
            None => {
                panic!("Couldn't parse {}", input);
            }
        }
        RegisterMachine { register, ip_register, instructions }
    }

    fn step(&mut self) -> bool {
        if self.register[self.ip_register] < self.instructions.len() {
            let a: usize = self.instructions[self.register[self.ip_register]][1];
            let b: usize = self.instructions[self.register[self.ip_register]][2];
            let c: usize = self.instructions[self.register[self.ip_register]][3];
            match self.instructions[self.register[self.ip_register]][0] {
                // addr
                0 => {
                    self.addr(a,b,c);
                }

                // addi
                1 => {
                    self.addi(a,b,c);
                }

                // mulr
                2 => {
                    self.mulr(a,b,c);
                }

                // muli
                3 => {
                    self.muli(a,b,c);
                }

                // banr
                4 => {
                    self.banr(a,b,c);
                }

                // bani
                5 => {
                    self.bani(a,b,c);
                }

                // borr
                6 => {
                    self.borr(a,b,c);
                }
                
                // bori
                7 => {
                    self.bori(a,b,c);
                }

                // setr
                8 => {
                    self.setr(a,b,c);
                }
                
                // seti
                9 => {
                    self.seti(a,b,c);
                }

                // gtir
                10 => {
                    self.gtir(a,b,c);
                }

                // gtri
                11 => {
                    self.gtri(a,b,c);
                }

                // gtrr
                12 => {
                    self.gtrr(a,b,c);
                }

                // eqir
                13 => {
                    self.eqir(a,b,c);
                }

                // eqri
                14 => {
                    self.eqri(a,b,c);
                }

                // eqrr
                15 => {
                    self.eqrr(a,b,c);
                }

                _ => {
                    panic!("Couldn't perform step!");
                }
            }
            self.register[self.ip_register] += 1;
            true
        }
        else {
            false
        }
    }

    fn print_instructions(&self) {
        for i in 0..self.instructions.len() {
            self.print_instruction(i);
        }
    }

    fn print_instruction(&self, index: usize) {
        print!("{}", opcode_to_opname(self.instructions[index][0]));
        for i in 1..4 {
            print!(" {}", self.instructions[index][i]);
        }
        println!();
    }
    
    fn print_register(&self) {
        let mut output: String = String::new();
        output.push_str(&format!("ip={} ",self.register[self.ip_register]));
        for i in 0..6 {
            if i == 0 {
                output.push('[');
            }
            else {
                output.push(',');
                output.push(' ');
            }
            output.push_str(&format!("{}",self.register[i]));
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

    fn setr(&mut self, a:usize, b: usize, c:usize) {
        self.register[c] = self.register[a];
    }

    fn seti(&mut self, a: usize, b: usize, c:usize) {
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

fn ip_from_str(input: &str) -> Option<usize> {
    lazy_static!{
        static ref RE_IP: Regex = Regex::new(r"#ip (?P<ip>[0-9])").unwrap();     
    }
    match RE_IP.captures(input) {
        Some(cap) => {
            return Some(cap.name("ip").map_or(0usize, |m| m.as_str().parse::<usize>().unwrap()));
        }
        None => {
            return None;
        }
    }
}

fn instructions_from_str(input: &str) -> Option<Vec<[usize;4]>> {
    let mut instructions: Vec<[usize;4]> = Vec::new();
    for line in input.trim().lines() {
        match instruction_from_str(line) {
            Some(i) => {
                instructions.push(i);
            }
            None => {
                continue;
            }
        }
    }
    Some(instructions)
}

fn instruction_from_str(input: &str) -> Option<[usize;4]> {
    lazy_static!{
        static ref RE_REGISTER: Regex =
            Regex::new(r"(?P<zero>[a-z]+) (?P<one>[0-9]+) (?P<two>[0-9]+) (?P<three>[0-9]+)").unwrap();     
    }
    match RE_REGISTER.captures(input) {
        Some(cap) => {
            let zero: usize = opname_to_opcode(cap.name("zero").map_or("", |m| m.as_str()));
            let one = cap.name("one").map_or(0usize, |m| m.as_str().parse::<usize>().unwrap());
            let two = cap.name("two").map_or(0usize, |m| m.as_str().parse::<usize>().unwrap());
            let three = cap.name("three").map_or(0usize, |m| m.as_str().parse::<usize>().unwrap());        
            let result: [usize;4] = [zero, one, two, three];
            Some(result)
        }
        None => {
            None
        }
    }
}

fn opname_to_opcode(input: &str) -> usize {
    match input {
        "addr" => {
            0
        }
        "addi" => {
            1
        }
        "mulr" => {
            2
        }
        "muli" => {
            3
        }
        "banr" => {
            4
        }
        "bani" => {
            5
        }
        "borr" => {
            6
        }
        "bori" => {
            7
        }
        "setr" => {
            8
        }
        "seti" => {
            9
        }
        "gtir" => {
            10
        }
        "gtri" => {
            11
        }
        "gtrr" => {
            12
        }
        "eqir" => {
            13
        }
        "eqri" => {
            14
        }
        "eqrr" => {
            15
        }
        _ => {
            panic!("Couldn't parse: {}", input);
        }
    }
}

fn opcode_to_opname(input: usize) -> &'static str {
    match input {
        0 => {
            &"addr"
        }
        1 => {
            &"addi"
        }
        2 => {
            &"mulr"
        }
        3 => {
            &"muli"
        }
        4 => {
            &"banr"
        }
        5 => {
            &"bani"
        }
        6 => {
            &"borr"
        }
        7 => {
            &"bori"
        }
        8 => {
            &"setr"
        }
        9 => {
            &"seti"
        }
        10 => {
            &"gtir"
        }
        11 => {
            &"gtri"
        }
        12 => {
            &"gtrr"
        }
        13 => {
            &"eqir"
        }
        14 => {
            &"eqri"
        }
        15 => {
            &"eqrr"
        }
        _ => {
            panic!("Couldn't parse: {}", input);
        }
    }
}

fn solve_part_1(input_str: &str) -> usize {
    let mut rm: RegisterMachine = RegisterMachine::from_str(INPUT);
    
    let mut finished: bool = false;
    while !finished {
        finished = !rm.step();
    }
    rm.register[1]
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));

    
    // reserve engineering of the instruction set for part 2 yields
    // that it attemps to return the sum of all factors of 10551355
    // (the entry of register[5] after 18 steps). So the answer to
    // part 2 is:
    
    println!("Answer part 1: {}", 12690000);
}

#[cfg(test)]
mod tests {
    use super::*;
}
