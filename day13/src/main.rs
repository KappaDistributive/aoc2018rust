use std::cmp::Ordering;

const INPUT_SAMPLE: &str = include_str!("../input_sample.txt");
const INPUT: &str = include_str!("../input.txt");

type Point = (usize,usize);

#[derive(Eq,Clone)]
struct Cart {
    pos: Point,
    direction: char,
    intersection_counter: usize,
}

impl Cart {
    fn step(&mut self, track: &Vec<char>, width: usize) -> Self {
        let mut pos: Point = (0,0);
        let mut direction: char = ' ';
        let mut intersection_counter: usize = 0;
        match self.direction {
            '^' => {
                pos = (self.pos.0, self.pos.1 - 1);
                direction = self.update_direction(track, width);
                intersection_counter = self.update_intersection_counter(track, width);
            }
            'v' => {
                pos = (self.pos.0, self.pos.1 + 1);
                direction = self.update_direction(track, width);
                intersection_counter = self.update_intersection_counter(track, width);
            }
            '<' => {
                pos = (self.pos.0 - 1, self.pos.1);
                direction = self.update_direction(track, width);
                intersection_counter = self.update_intersection_counter(track, width);
            }
            '>' => {
                pos = (self.pos.0 + 1, self.pos.1);
                direction = self.update_direction(track, width);
                intersection_counter = self.update_intersection_counter(track, width);
            }
            _ => {
                panic!("Unable to complete step for cart at ({},{})", self.pos.0, self.pos.1);
            }
        }        
        Cart { pos, direction, intersection_counter }
    }

    fn update_intersection_counter(&self, track: &Vec<char>, width: usize) -> usize {
        let mut tile: char = ' ';
        match self.direction {
            '^' => {
                tile = track[(self.pos.1 - 1) * width + self.pos.0];
            }
            'v' => {
                tile = track[(self.pos.1 + 1) * width + self.pos.0];
            }
            '<' => {
                tile = track[self.pos.1 * width + self.pos.0 - 1];
            }
            '>' => {
                tile = track[self.pos.1 * width + self.pos.0 + 1];
            }
            _ => {
                panic!("Couldn't update intersection counter for cart at ({},{})", self.pos.0, self.pos.1);
            }
        }
        if tile == '+' {
            return self.intersection_counter + 1;
        }
        else {
            return self.intersection_counter;
        }
    }
    
    fn update_direction(&self, track: &Vec<char>, width: usize) -> char {
        match self.direction {
            '^' => {
                match track[(self.pos.1 - 1) * width + self.pos.0] {
                    '|' => {
                        return '^';
                    }
                    '/' => {
                        return '>';
                    }
                    '\\' => {
                        return '<';
                    }
                    '+' => {
                        match  self.intersection_counter % 3 {
                            0 => {
                                return '<';
                            }
                            1 => {
                                return '^';
                            }
                            2 => {
                                return '>';
                            }
                            _ => {
                                panic!("Couldn't update direction for cart at ({},{})", self.pos.0, self.pos.1);
                            }
                            
                        }
                    }
                    _ => {
                        panic!("Couldn't update direction for cart at ({},{})", self.pos.0, self.pos.1);
                    }
                }
            }

            'v' => {
                match track[(self.pos.1 + 1) * width + self.pos.0] {
                    '|' => {
                        return 'v';
                    }
                    '/' => {
                        return '<';
                    }
                    '\\' => {
                        return '>';
                    }
                    '+' => {
                        match  self.intersection_counter % 3 {
                            0 => {
                                return '>';
                            }
                            1 => {
                                return 'v';
                            }
                            2 => {
                                return '<';
                            }
                            _ => {
                                panic!("Couldn't update direction for cart at ({},{})", self.pos.0, self.pos.1);
                            }
                        }
                    }
                    _ => {
                        panic!("Couldn't update direction for cart at ({},{})", self.pos.0, self.pos.1);
                    }
                }
            }

            '<' => {
                match track[self.pos.1 * width + self.pos.0 - 1] {
                    '-' => {
                        return '<';
                    }
                    '/' => {
                        return 'v';
                    }
                    '\\' => {
                        return '^';
                    }
                    '+' => {
                        match  self.intersection_counter % 3 {
                            0 => {
                                return 'v';
                            }
                            1 => {
                                return '<';
                            }
                            2 => {
                                return '^';
                            }
                            _ => {
                                panic!("Couldn't update direction for cart at ({},{})", self.pos.0, self.pos.1);
                            }
                            
                        }
                    }
                    _ => {
                        panic!("Couldn't update direction for cart at ({},{})", self.pos.0, self.pos.1);
                    }
                }
            }

            '>' => {
                match track[self.pos.1 * width + self.pos.0 + 1] {
                    '-' => {
                        return '>';
                    }
                    '/' => {
                        return '^';
                    }
                    '\\' => {
                        return 'v';
                    }
                    '+' => {
                        match  self.intersection_counter % 3 {
                            0 => {
                                return '^';
                            }
                            1 => {
                                return '>';
                            }
                            2 => {
                                return 'v';
                            }
                            _ => {
                                panic!("Couldn't update direction for cart at ({},{})", self.pos.0, self.pos.1);
                            }
                            
                        }
                    }
                    _ => {
                        panic!("Couldn't update direction for cart at ({},{})", self.pos.0, self.pos.1);
                    }
                }
            }
            
            _ => {
                panic!("Couldn't update direction for cart at ({},{})", self.pos.0, self.pos.1);
            }
        }
    }
}

        



impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        if self.pos.1 < other.pos.1
            || (self.pos.1 == other.pos.1 && self.pos.0 < other.pos.0)
            || (self.pos.1 == other.pos.1 && self.pos.0 == other.pos.0 && (self.direction as u8) < (other.direction as u8))
            || (self.pos.1 == other.pos.1 && self.pos.0 == other.pos.0 && (self.direction as u8) == (other.direction as u8) && self.intersection_counter < other.intersection_counter) {
                return Ordering::Less;
            }
        if self.pos.1 < other.pos.1
            || (self.pos.1 == other.pos.1 && self.pos.0 > other.pos.0)
            || (self.pos.1 == other.pos.1 && self.pos.0 == other.pos.0 && (self.direction as u8) > (other.direction as u8))
            || (self.pos.1 == other.pos.1 && self.pos.0 == other.pos.0 && (self.direction as u8 == other.direction as u8) && self.intersection_counter > other.intersection_counter) {
                return Ordering::Greater;
        }
        else {
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1
    }
}

struct CartAndTrack {
    track: Vec<char>,
    width: usize,
    height: usize,
    carts: Vec<Cart>,
    tick: usize,
    crashes: Vec<Point>,
}

impl CartAndTrack {
    fn from_str(input: &str) -> Self {
        let mut track: Vec<char> = Vec::new();
        let mut carts: Vec<Cart> = Vec::new();
        let mut width: usize = input.lines().next().unwrap().chars().count();
        let mut x: usize = 0;
        let mut y: usize = 0;
        for line in input.lines() {
            x = 0;
            for c in line.chars() {
                match c {
                    '^' | 'v' => {
      	                carts.push(Cart { pos: (x,y), direction: c, intersection_counter: 0 });
                        track.push('|');
                   }
                    '>' | '<' => {
                        carts.push(Cart { pos: (x,y), direction: c, intersection_counter: 0 });
                        track.push('-');
                    }
                    _ => {
                        track.push(c);
                    }
                }
                x += 1;
            }
            y += 1;
        }
        if track.len() % width != 0 {
            panic!("Couldn't parse:\n{}", input);
        }
        let height = track.len() / width;
        let tick: usize = 0;
        let crashes: Vec<Point> = Vec::new();
        carts.sort();
        CartAndTrack { track, width, height, carts, tick, crashes }
    }
    
    fn occupied (&self, cart: &Cart) -> Option<usize> {
        for i in 0..self.carts.len() {
            if cart.pos == self.carts[i].pos {
                return Some(i);
            }
        }
        return None;
    }

    fn step(&mut self) {
        let mut crashed: Vec<usize> = Vec::new();
        self.carts.sort();
        for i in 0..self.carts.len() {                
            let mut t_cart = self.carts[i].step(&self.track, self.width);
            match self.occupied(&t_cart) {
                // carts with index i and j get into a crash
                Some(j) => {
                    self.crashes.push(self.carts[j].pos);
                    if !crashed.contains(&i) {
                        crashed.push(i);
                    }
                    if !crashed.contains(&j) {
                        crashed.push(j);
                    }
                }
                // cart with index i is free to move
                None => {
                    self.carts[i] = t_cart;
                }
            }
        }
        crashed.sort();
        crashed.reverse();
        for i in crashed {
            self.carts.remove(i);
        }
    }

    fn print_current_track(&self) {
        let mut current_track: Vec<char> = self.track.clone();

        for c in &self.carts {
            current_track[c.pos.1 * self.width + c.pos.0] = c.direction;
        }

        for c in &self.crashes {
            current_track[c.1 * self.width + c.0] = 'X';
        }

        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", current_track[y * self.width + x]);
            }
            println!();
        }
    }
}

fn solve_part_1(input_str: &str) -> Point {
    let mut track: CartAndTrack = CartAndTrack::from_str(input_str);
    while track.crashes.len() == 0 {
        track.step();
    }
    track.crashes[0]
}

fn solve_part_2(input_str: &str) -> Point {
    let mut track: CartAndTrack = CartAndTrack::from_str(input_str);
    while track.carts.len() > 1 {
        track.step();
    }
    track.carts[0].pos
}

fn main() {
    let result_1: Point = solve_part_1(INPUT);
    println!("Answer part 1: ({},{})", result_1.0, result_1.1);
    
    let result_2: Point = solve_part_2(INPUT);
    println!("Answer part 2: ({},{})", result_2.0, result_2.1);
}
