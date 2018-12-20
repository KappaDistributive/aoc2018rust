use std::cmp::Ordering;
const INPUT_SAMPLE: &str = include_str!("../input_sample.txt");
const INPUT: &str = include_str!("../input.txt");

type Point = (usize,usize);

#[derive(Eq)]
struct Cart {
    pos: Point,
    direction: char,
    intersection_counter: usize,
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


    // implement this
    fn step(&mut self) {
        
    }

    // fn get_current_track(&self, x: usize, y: usize) -> char {
    //     self.current_track[y * self.width + x]
    // }

    fn print_current_track(&self) {
        let mut current_track: Vec<char> = self.track.clone();

        for c in &self.carts {
            current_track[c.pos.1 * self.width + c.pos.0] = c.direction;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", current_track[y * self.width + x]);
            }
            println!();
        }
    }
}

fn main() {
    let mut test: CartAndTrack = CartAndTrack::from_str(INPUT_SAMPLE);
    test.print_current_track();
    
}
