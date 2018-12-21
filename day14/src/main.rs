
const INPUT: usize = 939601;

struct Scoreboard {
    alice: usize,
    bob: usize,
    scores: Vec<u8>,
}

impl Scoreboard {
    fn new() -> Self {
        Scoreboard { alice: 0, bob: 1, scores: vec![3,7] }
    }

    fn print(&self) {
        for i in 0..self.scores.len() {
            if i == self.alice {
                print!("({})", self.scores[i]);
            }
            else if i == self.bob {
                print!("[{}]", self.scores[i]);
            }
            else {
                print!(" {} ", self.scores[i]);
            }            
        }
        println!();
    }

    fn step(&mut self) {
        let score: u8 = self.scores[self.alice] + self.scores[self.bob];
        if score < 10 {
            self.scores.push(score);
        }
        else {
            self.scores.push(score / 10);
            self.scores.push(score % 10);
        }
        self.alice = (1 + self.alice + (self.scores[self.alice] as usize)) % self.scores.len();
        self.bob = (1 + self.bob + (self.scores[self.bob] as usize)) % self.scores.len();
    }
}

fn solve_part_1(input: usize) -> usize {
    let mut board: Scoreboard = Scoreboard::new();
    while board.scores.len() < input + 10 {
        board.step();
    }
    let mut result: usize = 0;
    for i in 0..10 {
        result *= 10;
        result += board.scores[input+i] as usize;
    }
    result
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
}
