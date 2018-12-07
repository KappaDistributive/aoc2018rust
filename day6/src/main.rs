#![feature(test)]
extern crate test;

#[macro_use] extern crate itertools;
use std::fs::File;
use std::io::prelude::*;
//use std::cmp;
//use std::error::Error;
use std::collections::HashSet;

type Point = (i32,i32);
//type Entry = (Point,Point);

fn format_input(input_str: &str) -> Vec<Point> {
    let mut coords: Vec<Point> = Vec::new();    
    for line in input_str.lines() {
        let temp: Vec<&str> = line.split(", ").collect();
        coords.push((temp[0].parse().unwrap(),temp[1].parse().unwrap()));
    }
    coords
}

fn get_map_boundaries(coords: &Vec<Point>) -> (Point,Point) {
    let mut min: Point = (0,0);
    let mut max: Point = (0,0);
    let mut assigned_min: Point = (0,0);
    let mut assigned_max: Point = (0,0);
    for c in coords {
        if assigned_min.0 == 0 || c.0 < min.0 {
            assigned_min.0 = 1;
            min.0 = c.0;
        }
        if assigned_min.1 == 0 || c.1 < min.1 {
            assigned_min.1 = 1;
            min.1 = c.1;
        }
        if assigned_max.0 == 0 || c.0 > max.0 {
            assigned_max.0 = 1;
            max.0 = c.0;
        }
        if assigned_max.1 == 0 || c.1 > max.1 {
            assigned_max.1 = 1;
            max.1 = c.1;
        }
    }
    if (assigned_min,assigned_max) == ((1,1),(1,1)) {
        (min,max)
    }
    else {
        unreachable!();
    }
}

fn distance(p: &Point, q: &Point) -> i32 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn get_closest_coordinate(p: &Point,duplicate: &Point, coords: &Vec<Point>) -> Point {
    // doesn't handle duplicates, yet
    let mut closest: Point = coords[0];
    for c in coords.clone() {
        if distance(&c,p) < distance(&closest,p) {
            closest = c;
        }
    }
    for c in coords {
        if *c != closest && distance(&c,p) == distance(&closest,p) {
            return *duplicate;
        }
    }
    closest
}

fn calculate_area(p: &Point, board: &Vec<(Point,Point)>) -> i32 {
    let mut area = 0i32;
    for (_,r) in board {
        if p==r {
            area += 1;
        }
    }
    area
}

fn solve_part_1(coords: &Vec<Point>) -> i32 {
    let (min,max) = get_map_boundaries(&coords);
    let disregard = (min.0-1, min.1-1);
    // println!("min: ({},{}) max: ({},{})", min.0, min.1, max.0, max.1);

    // create a [min.0, max.1]x[min.1, max.1] board such that board(x,y) = (x,y)
    let board: Vec<_> =iproduct!(min.0..=max.0,min.1..=max.1).map(|(x,y)| (x,y)).collect();

    // for p in board let c(p) be the closest coordinate to p. Replace
    // p either with (p,c(p)) or with (p, disregard) if two
    // coordinates have the same distance to p
    let board: Vec<_> = board.iter().map(|p| (*p,get_closest_coordinate(&p, &disregard, &coords))).collect();

    // if (p,q) in board contributes to an infinite region add q it to exclude
    // also add disregard to exclude
    let mut exclude: HashSet<Point> = HashSet::new();
    exclude.insert(disregard);
    for b in board.clone() {
        let ((x,y),q) = b;
        if x == min.0 || y == min.1 || x == max.0 || y == max.1 {
            exclude.insert(q);
        }
    }

    // for (p,q) in board, if q is not in exclude, calculate the area
    // a that q contributes to and insert (q,a) to areas
    let mut areas :HashSet<(Point,i32)> = HashSet::new();
    for c in coords {       
        areas.insert((*c,calculate_area(&c,&board)));
    }

    // calculate largest area of coords that are not excluded
    let mut answer_part_1 = 0i32;
    for x in areas {
        let (p,a) = x;
        if !exclude.contains(&p) && a > answer_part_1 {
            answer_part_1 = a;
        }
    }

    answer_part_1
}

fn main() {
    let filename = "../inputs/day6.txt";
    let mut input_str = String::new();
    File::open(filename).expect("Error reading file")
        .read_to_string(&mut input_str).expect("Error reading file");
    
    let coords: Vec<Point> = format_input(&input_str);

    println!("Answer part 1: {}",solve_part_1(&coords));

}

#[cfg(test)]
mod tests {
    use super::*;
    //use test::Bencher;

    #[test]
    fn test_distance() {
        assert_eq!(distance(&(0,0),&(0,1)),1);
        assert_eq!(distance(&(0,0),&(1,0)),1);
        assert_eq!(distance(&(0,0),&(1,1)),2);
        assert_eq!(distance(&(0,0),&(0,-1)),1);
        assert_eq!(distance(&(0,0),&(-1,0)),1);
        assert_eq!(distance(&(0,0),&(-1,-1)),2);
         assert_eq!(distance(&(3,4),&(4,5)),2);
    }
    
    #[test]
    fn test_get_closest_coordinate() {
        let coords: Vec<(i32,i32)> = vec![(100,100),(0,0)];
        assert_eq!(get_closest_coordinate(&(100,100), &(-1,-1), &coords), (100,100));
        assert_eq!(get_closest_coordinate(&(75,100), &(-1,-1), &coords), (100,100));
        assert_eq!(get_closest_coordinate(&(-12,10), &(-1,-1), &coords), (0,0));
    }
}
