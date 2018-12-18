const INPUT: &str = include_str!("../input.txt");
const INPUT_SAMPLE: &str = include_str!("../input_sample.txt");

#[derive(Clone)]
struct GameOfTrees {
    area: Vec<char>,
    width: usize,
    height: usize,
}

impl PartialEq for GameOfTrees {
    fn eq(&self, other: &GameOfTrees) -> bool {
        self.area == other.area && self.width == other.width && self.height == other.height
    }
}

impl GameOfTrees {
    fn from_str(input: &str) -> Self {
        let mut width: usize = 0;
        let mut area: Vec<char> = Vec::new();
        
        for line in input.trim().lines() {
            if width == 0 {
                for c in line.chars() {
                    width += 1;
                    area.push(c);
                }
            }
            else {
                for c in line.chars() {
                    area.push(c);
                }
            }
        }
        if area.len() % width == 0 {
            let height: usize = area.len() / width;
            GameOfTrees { area, width, height }
        }
        else {
            panic!("Couldn't parse: {}", input);
        }
    }
    
    // returns value at area[x,y]
    fn loc(&self, x: usize, y: usize) -> char {
        self.area[y * self.width + x]
    }

    // prints area
    fn print(&self) {
        for i in 0..self.area.len() {
            if i % self.width == 0 {
                println!();
            }
            print!("{}", self.area[i]);
        }
    }

    fn neighbour_counter(&self, kind: char, x: usize, y: usize) -> usize {        
        let top: usize = if y > 0 { y-1 } else { 0 };
        let bottom: usize = if y+1 < self.height { y+1 } else { self.height-1 };
        let left: usize = if x > 0 { x-1 } else { 0 };
        let right: usize = if x+1 < self.width { x+1 } else { self.width-1 };
        let mut result: usize = 0;
        for j in top..=bottom {
            for i in left..=right {
                if self.loc(i,j) == kind && (j != y || i != x) {
                    result += 1;
                }
            }
        }
        result
    }

    fn step(&mut self) {
        let mut t_area = self.area.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                match self.loc(x,y) {
                    '.' => {
                        if self.neighbour_counter('|',x,y) > 2 {
                            t_area[y * self.width + x] = '|';
                        }
                    }
                    '|' => {
                        if self.neighbour_counter('#',x,y) > 2 {
                            t_area[y * self.width + x] = '#';
                        }
                    }
                    '#' => {
                        if self.neighbour_counter('#',x,y) == 0
                            || self.neighbour_counter('|',x,y) == 0 {
                                t_area[y * self.width + x] = '.';
                            }
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
        self.area = t_area;
    }

    fn resource_value(&self) -> u32 {
        let lumberyards: u32 = self.area.clone().iter().map(|m| if *m == '#' { 1 } else { 0 }).sum();
        let trees: u32 = self.area.clone().iter().map(|m| if *m == '|' { 1 } else { 0 }).sum();
        lumberyards*trees
    }
}


fn solve_part_1(input_str: &str) -> u32 {
    let mut g = GameOfTrees::from_str(input_str);
    for _ in 0..10 {
        g.step();
    }
    g.resource_value()
}

fn solve_part_2(input_str: &str) -> u32 {
    let mut g = GameOfTrees::from_str(input_str);
    let mut games: Vec<GameOfTrees> = Vec::new();
    games.push(g.clone());
    let mut iteration: usize = 0;
    let mut index: usize = 0;
    let mut cycle_len: usize = 0;
    let mut cycle: bool = false;
    
    // find cycle
    while !cycle { 
        g.step();
        iteration += 1;
        if games.contains(&g) {
            cycle = true;
            for i in 0..games.len() {
                if games[i].clone() == g.clone() {
                    index = i;
                    break;
                }
            }
            cycle_len = games.len()-index;
        }
        else {
            games.push(g.clone());
        }
    }

    // advance iteration one cycle at a time as long as possible
    while iteration + cycle_len <= 1000000000 {
        iteration += cycle_len;
    }
    
    // complete the remaining steps towards the goal
    for _ in iteration..1000000000 {
        g.step();
    }
    
    // return resource_value
    g.resource_value()
}


fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT)); 
}
