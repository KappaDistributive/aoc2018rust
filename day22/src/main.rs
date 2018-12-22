const DEPTH: usize = 9465;
const TARGET: Point = (13,704);

type Point = (usize,usize);

struct Cave {
    width: usize,
    height: usize,
    pointer: Point,
    depth: usize,
    moving_right: bool,
    reduced_geological_indeces: [usize;100000],
    erosion_levels: [usize;100000],
    calculated: [bool;100000],
}

impl Cave {
    fn new(depth: usize) -> Self {
        Cave { width: 100,
               height: 1000,
               pointer: (0,0),
               depth: depth,
               moving_right: true,
               reduced_geological_indeces: [0usize;100000],
               erosion_levels: [0usize;100000],
               calculated: [false;100000] }
    }

    fn step(&mut self) {
        let (x,y) = self.pointer;
        match (x,y) {
            (0,0) => {
                self.reduced_geological_indeces[y * self.width + x] = 0;
                self.erosion_levels[y * self.width + x] = (self.reduced_geological_indeces[y * self.width + x] + self.depth) % 20183;
                self.pointer = (1,0);
                self.calculated[y * self.width + x] = true;
                self.moving_right = true;
            }
            (_,0) => {
                self.reduced_geological_indeces[y * self.width + x] = (x * 16807) % 20183;
                self.erosion_levels[y * self.width + x] = (self.reduced_geological_indeces[y * self.width + x] + self.depth) % 20183;
                self.pointer = (self.pointer.0, self.pointer.1 + 1);
                self.calculated[y * self.width + x] = true;
                self.moving_right = false;
            }
            (0,_) => {
                self.reduced_geological_indeces[y * self.width + x] = (x * 48271) % 20183;
                self.erosion_levels[y * self.width + x] = (self.reduced_geological_indeces[y * self.width + x] + self.depth) % 20183;
                self.pointer = (self.pointer.0 + 1, self.pointer.1);
                self.calculated[y * self.width + x] = true;
                self.moving_right = true;
            }
            (x,y) => {
                if self.moving_right {
                    if self.calculated[(y-1) * self.width + x] {
                        self.pointer = (self.pointer.0 + 1, self.pointer.1);
                    }
                    else {
                        self.pointer = (self.pointer.0, self.pointer.1 - 1);
                    }
                }
                else {
                    if self.calculated[y * self.width + x - 1] {
                        self.pointer = (self.pointer.0, self.pointer.1 + 1);
                    }
                    else {
                        self.pointer = (self.pointer.0 - 1, self.pointer.1);
                    }
                }
            }
        }
    }

    fn print_erosion_levels(&self) {
        'outer: for y in 0..self.height {
            for x in 0..self.width {
                if !self.calculated[y * self.width + x] {
                    break 'outer;
                }
                else if x == 0 && y == 0 {
                    print!("M");
                }
                else {
                    match self.erosion_levels[y * self.width + x] % 3 {
                        0 => {
                            print!(".");
                        }
                        1 => {
                            print!("=");
                        }
                        2 => {
                            print!("|");
                        }
                        _ => {
                            panic!("Couldn't print erosion levels at ({},{})", x, y);
                        }
                    }
                }
                println!();
            }
        }
    }
}

fn main() {
    let mut cave: Cave = Cave::new(510);
    for _ in 0..5 {
        cave.print_erosion_levels();
        cave.step();
    }
}
