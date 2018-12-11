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

fn locate_hotspot(grid: &Vec<Vec<i32>>,
                  grid_width: usize, grid_height: usize,
                  block_width: usize, block_height: usize) -> ((usize,usize),i32) {
    let summed_area_table: Vec<Vec<i32>> = summed_area(&grid, grid_width, grid_height);
    let mut pos: (usize,usize) = (0,0);
    let mut max: i32 = i32::MIN;
    for y in 0..(grid_height+1 - block_height) {
        for x in 0..(grid_width+1 - block_width) {
            let mut temp: i32 = 0;
            if x == 0 && y == 0{
                temp = summed_area_table[y+block_height-1][x+block_width-1];
            }
            if x > 0 && y == 0{
                temp = summed_area_table[y+block_height-1][x+block_width-1]
                     - summed_area_table[y+block_height-1][x-1];
            }
            if x == 0 && y >  0{
                temp = summed_area_table[y+block_height-1][x+block_width-1]
                     - summed_area_table[y-1][x+block_width-1];
            }
            if x > 0 && y > 0 {
                temp = summed_area_table[y+block_height-1][x+block_width-1]
                     + summed_area_table[y-1][x-1]
                     - summed_area_table[y-1][x+block_width-1]
                     - summed_area_table[y+block_height-1][x-1];
            }
            if temp > max {
                max = temp;
                pos = (x,y);
            }
        }
    }
    (pos, max)
}

fn fill_grid(grid_width: usize, grid_height: usize, grid_serial_number: i32) -> Vec<Vec<i32>> {

    let mut power_levels = vec![vec![0i32;grid_width];grid_height];

    // fill grid with power levels
    for y in 0usize..grid_height {
        for x in 0usize..grid_width {
            power_levels[y][x] = power_level((x+1,y+1), grid_serial_number);
        }
    }
    power_levels
}

fn summed_area(grid: &Vec<Vec<i32>>, grid_width: usize, grid_height: usize) -> Vec<Vec<i32>> {
    let mut summed_area_table: Vec<Vec<i32>> = vec![vec![0i32; grid_width]; grid_height];
    summed_area_table[0][0] = grid[0][0];
    for x in 1..grid_width {
        summed_area_table[0][x] = summed_area_table[0][x-1] + grid[0][x];
    }
    for y in 1..grid_height {
        summed_area_table[y][0] = summed_area_table[y-1][0] + grid[y][0];
    }
    for y in 1..grid_height {
        for x in 1..grid_width {
            summed_area_table[y][x] = grid[y][x]
                                    + summed_area_table[y-1][x]
                                    + summed_area_table[y][x-1]
                                    - summed_area_table[y-1][x-1];
        }
    }
    summed_area_table
}

fn solve_part_1(grid_serial_number: i32) -> (usize,usize) {
    let grid_width: usize = 300;
    let grid_height: usize = 300;
    let power_levels: Vec<Vec<i32>> = fill_grid(grid_width, grid_height, grid_serial_number);
    
    let (pos,_): ((usize,usize),i32) = locate_hotspot(&power_levels,
                                                    grid_width, grid_height,
                                                    3, 3);
        (pos.0 + 1, pos.1 + 1)
}

fn solve_part_2(grid_serial_number: i32) -> ((usize,usize),usize) {
    let grid_width: usize = 300;
    let grid_height: usize = 300;
    let power_levels: Vec<Vec<i32>> = fill_grid(grid_width, grid_height, grid_serial_number);
    
    let mut max:i32 = i32::MIN;
    let mut pos: (usize,usize) = (0,0);
    let mut size: usize = 0;
    
    for i in 1..=300 {
        let (t_pos, t_max): ((usize,usize),i32) = locate_hotspot(&power_levels,
                                                                 grid_width, grid_height,
                                                                 i, i);
        if t_max > max {
            max = t_max;
            pos = t_pos;
            size = i;
        }
    }
    ((pos.0 + 1, pos.1 + 1), size)
}


fn main() {
    let pos: (usize,usize) = solve_part_1(INPUT);
    println!("Answer part 1: ({},{})", pos.0, pos.1);
    
    let (pos,size): ((usize,usize),usize) = solve_part_2(INPUT);
    println!("Answer part 1: ({},{},{})", pos.0, pos.1, size);
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
