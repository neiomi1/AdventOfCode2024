use std::collections::{HashMap, HashSet};
use crate::{Directions, Point};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Guard {
    position : Point,
    direction : Directions
}

#[inline]
fn parse(input: &str) -> (HashSet::<Point>, Guard) {
    let mut obstacles: HashSet::<Point> = HashSet::new();
    let mut position: Guard = Guard { position: Point { x: 0, y: 0 }, direction: Directions::UP };

    input
        .lines()
        .enumerate()
        .for_each(|(y, line)| 
            line.char_indices().for_each(|(x, obj)|  
            match obj {
                '^' => position = Guard{position : Point { x: x.try_into().unwrap(), y: y.try_into().unwrap()}, direction: Directions::UP},
                '#' => {obstacles.insert(Point { x: x.try_into().unwrap(), y: y.try_into().unwrap() }); return},
                 _ => return
                }
        )
    );
    (obstacles, position)
}

#[inline]
fn simulate_move(obstacles : &HashSet::<Point>, guard :&mut Guard){
    let next_pos = get_next_position(guard);
    
    if obstacles.contains(&next_pos){
        turn_right(guard);
        return;
    }

    guard.position = next_pos;
}

fn get_next_position(guard: &Guard) -> Point {
    let next_pos = match  guard.direction{
            Directions::UP => Point{x: guard.position.x, y : guard.position.y - 1},
            Directions::LEFT =>  Point{x: guard.position.x - 1, y : guard.position.y},
            Directions::RIGHT =>  Point{x: guard.position.x + 1, y : guard.position.y},
            Directions::DOWN =>  Point{x: guard.position.x, y : guard.position.y + 1},
        };
    next_pos
}

fn turn_right(guard: &mut Guard) {
    guard.direction =  match guard.direction {
        Directions::UP => Directions::RIGHT,
        Directions::LEFT => Directions::UP,
        Directions::RIGHT => Directions::DOWN,
        Directions::DOWN => Directions::LEFT,
    };
}

#[inline]
fn check_out_of_bounds(point : &Point, origin : &Point, end : &Point) -> bool{
    point.x < origin.x || point.x >= end.x || point.y < origin.y || point.y >= end.y
}

fn pretty_print(obstacles : &HashSet::<Point>, guard :&Guard, visited: &HashSet::<Point>, rows: i32, columns: i32){
    for y in -1..rows+1{
        let mut rowstr = String::new();
        for x in -1..columns+1{
            let point = Point{x: x.try_into().unwrap(),y: y.try_into().unwrap()};
            if obstacles.contains(&point){
                rowstr.push('#');
                continue;
            }
            if guard.position == point{
                rowstr.push(match guard.direction {
                    Directions::UP => '^',
                    Directions::LEFT => '<',
                    Directions::RIGHT => '>',
                    Directions::DOWN => 'v',
                });
                continue;
            }

            if visited.contains(&point){
                rowstr.push('X');
                continue;
            }
            rowstr.push('.');
        }
        println!("{rowstr}");
    }
}

#[inline]
pub fn part1(input: &str) -> i32 {
   let (obstacles, mut guard) = parse(input);
   let mut visited : HashSet::<Point> = HashSet::new();
   let origin = Point{ x: 0, y : 0};
   let end = Point{ x: input.lines().count().try_into().unwrap(), y : input.lines().next().unwrap().chars().count().try_into().unwrap()};
   while !check_out_of_bounds(&guard.position, &origin, &end){
        visited.insert(guard.position);
        simulate_move(&obstacles, &mut guard);
   }

   pretty_print(&obstacles, &guard, &visited, end.x, end.y);
   visited.len().try_into().unwrap()
}

fn test_next_position_for_loop(obstacles : &HashSet::<Point>, previous_positions : &HashSet::<Guard>, guard: &Guard, origin : &Point, end : &Point) -> Option<Point>{
    let mut guard_copy = guard.clone();
    let obstacle = get_next_position(&guard_copy);
    let mut obstacles_copy = obstacles.clone();
    if !obstacles_copy.insert(obstacle){
        return None;
    }
    let mut new_path_positions : HashSet::<Guard> = HashSet::new();
    while !check_out_of_bounds(&guard_copy.position, &origin, &end){
        new_path_positions.insert(guard_copy);
        simulate_move(&obstacles_copy, &mut guard_copy);

        if previous_positions.contains(&guard_copy) || new_path_positions.contains(&guard_copy){
            return Some(obstacle);
        }
    }
    return  None;
}

#[inline]
pub fn part2(input: &str) -> i32{
    let (obstacles, mut guard) = parse(input);
    let mut previous_positions : HashSet::<Guard> = HashSet::new();
    let mut loop_positions : HashSet::<Point> = HashSet::new();
    let origin = Point{ x: 0, y : 0};
    let end = Point{ x: input.lines().count().try_into().unwrap(), y : input.lines().next().unwrap().chars().count().try_into().unwrap()};
    while !check_out_of_bounds(&guard.position, &origin, &end){
        previous_positions.insert(guard);
        simulate_move(&obstacles, &mut guard);

        let loop_position = test_next_position_for_loop(&obstacles, &previous_positions, &guard, &origin, &end);
        if loop_position.is_some(){
           loop_positions.insert(loop_position.unwrap());
        }
    }
    
    println!("{:?}", loop_positions);
    loop_positions.len().try_into().unwrap()
}


crate::aoctest!(41, 4752, 6, 1719);
