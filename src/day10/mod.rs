use std::{collections::{HashMap, HashSet}, vec};
use crate::{Point, ORTHOGONAL};

#[inline]
fn parse(input: &str) -> Vec<Vec<usize>> {
    let mut result = vec![];

    for line in input.lines(){
        result.push(line.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect());
    }

    result
}

fn find_paths(map : &Vec<Vec<usize>>) -> HashMap<Point, Vec<Point>>{
    let mut basecamps = HashMap::new();

    for (y, row) in map.iter().enumerate(){
        for (x, height) in row.iter().enumerate(){
            if *height == 0{
            let basecamp = Point::new(x as i32, y as i32);
            basecamps.insert(basecamp, find_paths_for_point(map, basecamp));    
            }
        }
    }
    basecamps
}

fn find_paths_for_point(map : &Vec<Vec<usize>>, point : Point) -> Vec<Point>{
    let mut result = vec![];
    
    let height = map[point.y as usize][point.x as usize];
    if height == 9{
        return [point].to_vec()
    }

    for direction in ORTHOGONAL{
        let new_pos = point + direction;
        if new_pos.in_bounds(&(map[0].len() as i32, map.len() as i32)) && map[new_pos.y as usize][new_pos.x as usize] == height + 1{
            result.append(&mut find_paths_for_point(map, new_pos));
        }
    }
    result
}

#[inline]
pub fn part1(input: &str) -> usize {
    let map = parse(input);

    let basecamps = find_paths(&map);
    basecamps.values().map(|peaks| HashSet::<Point>::from_iter(peaks.iter().cloned()).len()).sum()
}

#[inline]
pub fn part2(input: &str) -> usize{
    let map = parse(input);

    let basecamps = find_paths(&map);
    basecamps.values().map(|peaks| peaks.len()).sum()
}

crate::aoctest!(36, 482, 81, 1094);