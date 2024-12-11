use itertools::Itertools;
use std::{collections::{HashMap, HashSet}, vec};
use crate::{Point};

#[inline]
fn parse(input: &str) -> (HashMap<char, Vec<Point>>, (i32, i32)) {
    let mut antannae = HashMap::new();
    
    input
    .lines()
    .enumerate()
    .for_each(|(row, line)| line.trim().char_indices().for_each(|(col, chr)| if chr != '.' {antannae.entry(chr).or_insert(vec![]).push(Point{x: col as i32, y: row as i32})}));

    (antannae, (input.lines().next().unwrap().len() as i32, input.lines().count() as i32))
}

fn find_antinode(a : &Point, b: &Point, boundary : &(i32, i32)) -> Vec<Point>{
    let mut antinodes = vec![];
    let distance = b.distance(a);

    let an1 = Point{x : a.x - distance.x, y : a.y - distance.y};
    let an2 = Point{x : b.x + distance.x, y : b.y + distance.y};

    if an1.in_bounds(boundary){
        antinodes.push(an1);
    }
    if an2.in_bounds(boundary){
        antinodes.push(an2);
    }
    antinodes
}

fn find_all_antinodes(a : &Point, b: &Point, boundary : &(i32, i32)) -> Vec<Point>{
    let mut antinodes = vec![];
    let distance = b.distance(a);

    let mut i = 0;
    let mut an1 = Point{x : a.x, y : a.y};
    while an1.in_bounds(boundary){
        antinodes.push(an1);
        i+=1;
        an1 = Point{x : a.x - distance.x * i, y : a.y - distance.y * i};
    }
 
    let mut an2 = Point{x : b.x, y : b.y};
    i = 0;
    while an2.in_bounds(boundary){
        antinodes.push(an2);
        i+=1;
        an2 = Point{x : b.x + distance.x * i, y : b.y + distance.y * i};
    }

    antinodes
}

#[inline]
pub fn part1(input: &str) -> usize {
    let (charmap, bounds) = parse(input);   
    
    let mut antinodes = HashSet::new();

    for (_, pos) in charmap{
        pos
        .iter()
        .combinations(2)
        .for_each(|points| antinodes.extend(find_antinode(points[0], points[1], &bounds)));
    }
    antinodes.len()
}

#[inline]
pub fn part2(input: &str) -> usize{
    let (charmap, bounds) = parse(input);   
    let mut antinodes = HashSet::new();

    for (_, pos) in charmap{
        pos
        .iter()
        .combinations(2)
        .for_each(|points| antinodes.extend(find_all_antinodes(points[0], points[1], &bounds)));
    }
    antinodes.len()
}

#[allow(dead_code)]
fn pretty_print(antannae : &HashMap<char, Vec<Point>>, antinodes : &HashSet<Point>,  boundary : &(i32, i32)){
    for y in 0..boundary.1{
        let mut rowstr = String::new();
        'row : for x in 0..boundary.0{
            let point = Point{x: x as i32,y: y as i32};
            for (ch, positions) in antannae{
                if positions.contains(&point){
                    rowstr.push(*ch);
                    continue 'row;
                }
            }       if antinodes.contains(&point){
                rowstr.push('#');
                continue;
            }
            rowstr.push('.');
        }
        println!("{rowstr}");
    }
}

crate::aoctest!(14, 273, 34, 70597497486371);