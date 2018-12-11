#![feature(test)]
extern crate test;

use std::i32;
    
const INPUT: i32 = 6042;

fn power_level(pos: (usize,usize), input: i32) -> i32 {
    let rack_id: i32 = (pos.0 as i32) + 10;
    let mut power_level: i32 = rack_id * (pos.1 as i32);
    power_level += input;
    power_level *= rack_id;
    (power_level / 100) % 10 - 5
        
}

fn locate_hotspot(grid: Vec<Vec<i32>>,
                  grid_width: usize, grid_height: usize,
                  block_width: usize, block_height: usize) -> (usize,usize) {
    let mut pos: (usize,usize) = (0,0);
    let mut max: i32 = i32::MIN;
    for y in 0..(grid_height - block_height) {
        for x in 0..(grid_width - block_width) {
            let mut temp: i32 = 0;
            
            for j in 0..block_height {
                for i in 0..block_width {
                    temp += grid[y+j][x+i];
                }
            }
            
            if temp > max {
                println!("{}", max);
                max = temp;
                pos = (x,y);
            }
        }
    }
    pos
}

fn solve_part_1(grid_serial_number: i32) -> (usize,usize) {
    let grid_width: usize = 300;
    let grid_height: usize = 300;
    let mut power_levels = vec![vec![0i32;grid_width];grid_height];

    // fill grid with power levels
    for y in 0usize..grid_height {
        for x in 0usize..grid_width {
            power_levels[y][x] = power_level((x+1,y+1), grid_serial_number);
        }
    }
    
    let pos: (usize,usize) = locate_hotspot(power_levels,
                                                    grid_width, grid_height,
                                                    3, 3);
        (pos.0 + 1, pos.1 + 1)
}

fn main() {
    let pos: (usize,usize) = solve_part_1(INPUT);
    println!("Answer part 1: ({},{})", pos.0, pos.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_power_level() {
        assert_eq!(power_level((3,5),8), 4);
        assert_eq!(power_level((122,79),57), -5);
        assert_eq!(power_level((217,196),39), 0);
        assert_eq!(power_level((101,153),71), 4);
    }
}
