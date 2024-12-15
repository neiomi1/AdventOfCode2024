use std::collections::HashSet;

use crate::{Point, ORTHOGONAL};


#[inline]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn fence_prices(map : &Vec<Vec<char>>) -> usize{
    let mut result = 0;

    let mut areas : Vec<HashSet<Point>> = vec![];
    for (y, row) in map.iter().enumerate(){
        for (x, ch) in row.iter().enumerate(){
            let pt = Point::new(x as i32, y as i32);
            if areas.iter().any(|area| area.contains(&pt)){
                continue;
            }
            let mut area = HashSet::new();
            area.insert(pt);
            let perimeter = get_area_and_perimeter(map, &pt, &mut area);
            result += perimeter * area.len();
            println!("{ch}: area: {}, perimeter: {perimeter}", area.len());
            areas.push(area);
        }
    }

    result
}

fn get_area_and_perimeter(map : &Vec<Vec<char>>, starting_point: &Point, area: &mut HashSet<Point>) -> usize{
    let mut perimeter = 0;
    let ch = map[starting_point.y as usize][starting_point.x as usize];
    for direction in ORTHOGONAL{
        let next = direction + *starting_point;
        if area.contains(&next){
            continue;
        }

        if !next.in_bounds(&(map[0].len() as i32, map.len() as i32)) || ch != map[next.y as usize][next.x as usize]{
            perimeter += 1;
        } 
        else{
            area.insert(next);
            perimeter += get_area_and_perimeter(map, &next, area);
        }
    }

    perimeter
}

fn get_area_and_unique_perimeter(map : &Vec<Vec<char>>, starting_point: &Point, area: &mut HashSet<Point>) -> usize{
    let mut perimeter = 0;
    let ch = map[starting_point.y as usize][starting_point.x as usize];
    let bounds = &(map[0].len() as i32, map.len() as i32);
    for direction in ORTHOGONAL{
        let next = direction + *starting_point;
        if area.contains(&next){
            continue;
        }
      
        if !next.in_bounds(bounds) || ch != map[next.y as usize][next.x as usize]{
            for dir in ORTHOGONAL{
                if direction.x == 0 && dir.x != 0 || direction.y == 0 && dir.y != 0{
                    let companion = *starting_point + dir;
                    let boundary_to_test = companion + direction;
                    if !companion.in_bounds(bounds){
                        perimeter += 1;
                        continue;
                    }

                    if  !(ch == map[companion.y as usize][companion.x as usize] && (!boundary_to_test.in_bounds(bounds) || map[boundary_to_test.y as usize][boundary_to_test.x as usize] != ch)){
                        perimeter += 1;
                    }
                    else{
                    }
                }
            }
        } 
        else{
            area.insert(next);
            perimeter += get_area_and_unique_perimeter(map, &next, area);
        }
    }

    perimeter
}

fn bulk_fence_prices(map : &Vec<Vec<char>>) -> usize{
    let mut result = 0;

    let mut areas : Vec<HashSet<Point>> = vec![];
    for (y, row) in map.iter().enumerate(){
        for (x, ch) in row.iter().enumerate(){
            let pt = Point::new(x as i32, y as i32);
            if areas.iter().any(|area| area.contains(&pt)){
                continue;
            }
            let mut area = HashSet::new();
            area.insert(pt);
            let perimeter = get_area_and_unique_perimeter(map, &pt, &mut area);
            result += perimeter / 2 * area.len();
            println!("{ch}: area: {}, perimeter: {perimeter}", area.len());
            areas.push(area);
        }
    }

    result
}

#[inline]
pub fn part1(input: &str) -> usize {
    let map = parse(input);
    fence_prices(&map)
}

#[inline]
pub fn part2(input: &str) -> usize{
    let map = parse(input);
    bulk_fence_prices(&map)
}

crate::aoctest!(1930, 1533024, 1206, 910066);