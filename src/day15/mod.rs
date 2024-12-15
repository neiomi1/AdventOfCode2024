use std::hash::Hasher;
use std::{collections::HashSet, fs::OpenOptions, hash::Hash};
use std::io::Write;

use itertools::Itertools;

use crate::{Point, DOWN, LEFT, RIGHT, UP};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Box{
    area : HashSet<Point>,
}

impl Hash for Box {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.area.iter().collect_vec().hash(state)
    }
}

impl Box {
    fn check_move(&self, walls : &HashSet<Point>, boxes: &HashSet<Box>, direction: &Point, boundary : &Point) -> (bool, HashSet<Box>){
        let new_area : HashSet<Point> = self.area.iter().map(|pt| pt + direction).collect();
        let moved_boxes : HashSet<Box> = boxes.iter().filter(|bx| *bx != self && bx.area.iter().any(|pt| new_area.contains(pt))).into_iter().cloned().collect();
        let can_move = new_area.iter().all(|pt| pt.check_bounds(boundary) && !walls.contains(&pt));
        if !can_move{
            return (false, HashSet::new());
        }
        let mut inner_moved_boxes: HashSet<Box> = HashSet::new();
        let mut total_boxes = moved_boxes.clone();
        for bx in moved_boxes{
            let (can_move, moved) = bx.check_move(walls, boxes, direction, boundary);
            if !can_move{
                return (false, HashSet::new());
            }
            inner_moved_boxes.extend(moved.clone());
        }
        total_boxes.extend(inner_moved_boxes);
        total_boxes.insert(self.clone());
        (true, total_boxes)
    }

    fn move_self(&mut self, direction : &Point){
        self.area = self.area.iter().map(|pt| pt + direction).collect();
    }
    
    fn closest_edge(&self) -> i32{
        self.area.iter().map(|edge| edge.x + 2  + (edge.y+1)*100).min().expect("failed")
    }
}



#[inline]
fn parse(input: &str, part2: bool) -> (HashSet<Box>, HashSet<Point>, Point, Vec<Point>, Point) {
    let mut directions = vec![];
    let mut boxes = HashSet::<Box>::new();
    let mut walls = HashSet::<Point>::new();
    let mut robot : Point = Point::new(0, 0);
    let mut boundary : Point = Point::new(0, 0);

    for (y, line) in input.lines().enumerate() {
        if line.is_empty(){
            continue;
        }

        
        if y == 0{
            boundary.x = (line.len() - 2) as i32;
            continue;      
        }

        for (x, ch) in line.char_indices(){
            if x == line.len(){
                continue;
            }

            if x == 0 && ch == '#'{
                boundary.y += 1;
            }

            match ch {
                '@' => {
                    if part2{   robot = Point::new((x as i32 -1)*2, y as i32 -1) }
                    else{robot = Point::new(x as i32 -1, y as i32 -1)}
                },
                'O' => {
                    let mut bx =  Box{ area : HashSet::from_iter([Point::new(x as i32 -1, y as i32 -1)])};
                    if part2{
                      bx = Box{ area : HashSet::from_iter([Point::new((x as i32 -1)*2, y as i32 -1), Point::new((x as i32 -1)*2 + 1, y as i32 -1)])};
                    }
                    boxes.insert(bx);},
                '<' => directions.push(LEFT),
                '^' => directions.push(UP),
                '>' => directions.push(RIGHT),
                'v' => directions.push(DOWN),
                '#' => {
                    if part2{ walls.insert(Point::new((x as i32 -1)*2, y as i32 -1)); walls.insert(Point::new((x as i32 -1)*2+1, y as i32 -1));}
                    else{   walls.insert(Point::new(x as i32 - 1, y as i32 -1)); }
                }
                _ => continue
            }
        }
    }

    boundary.y -= 1;

    if part2{
        boundary.x *= 2;
    }

    (boxes, walls, boundary, directions, robot)
}

fn get_warehouse_gps_value(boxes : &HashSet<Box>) -> i32{
    boxes.iter().fold(0, |a, b| a + b.area.iter().fold(0, |acc, pt| acc + (pt.x+1) + (pt.y+1)*100))
}

fn get_warehouse_gps_value_2(boxes : &HashSet<Box>) -> i32{
    let test : Vec<i32> = boxes.iter().map(|b| b.closest_edge()).sorted().collect();
    println!("{:?}", test);
    boxes.iter().fold(0, |a, b| a + b.closest_edge())
}

fn apply_directions(mut boxes: HashSet<Box>, walls: &HashSet<Point>, boundary: &Point, mut robot: Point, directions: &Vec<Point>) -> (HashSet<Box>, Point){
    for dir in directions{
        // print_to_file(&boxes, walls, boundary, &robot, dir);
        
        let new_robot_pos = robot + *dir;
        
        if !new_robot_pos.check_bounds(boundary) || walls.contains(&new_robot_pos){
            continue;
        }

        let collision = boxes.iter().find(|b| b.area.contains(&new_robot_pos));
        if collision.is_none(){
            robot = new_robot_pos;
            continue;
        }
    
        let first_box = collision.unwrap();
        let (can_move, moved) = first_box.check_move(walls, &boxes, dir, boundary);

        if can_move{
            boxes = boxes.into_iter().map(|mut bx| {if moved.contains(&bx){bx.move_self(dir); bx}else{bx}}).collect();
            robot = new_robot_pos;
        }
    }
    (boxes, robot)
}

fn pretty_print(boxes : &HashSet<Box>, walls: &HashSet<Point>, boundary : &Point, robot : &Point, part2: bool){
    let mut stretch_factor = 1;
    if part2{
        stretch_factor +=1;
    }
    println!("{}", "#".repeat(boundary.x as usize + 2 * stretch_factor));
    for y in 0..boundary.y{
        let mut rowstr = String::new();
        for _ in 0..stretch_factor{
            rowstr.push('#');
        }
        for x in 0..boundary.x{
            let pt = Point::new(x, y);
            if boxes.iter().any(|b| b.area.contains(&pt)){
                rowstr.push('O');
                continue;
            }
            if walls.contains(&pt){
                rowstr.push('#');
                continue;
            }
            if pt == *robot{
                rowstr.push('@');
                continue;
            }
            rowstr.push('.');
        }
        for _ in 0..stretch_factor{
            rowstr.push('#');
        }
        println!("{rowstr}");
    }
    println!("{}", "#".repeat(boundary.x as usize + 2 * stretch_factor));
    println!();
}

#[allow(dead_code)]
fn print_to_file(boxes : &HashSet<Box>, walls: &HashSet<Point>, boundary : &Point, robot : &Point, direction : &Point){
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("output.txt")
        .unwrap();

        writeln!(file, "{}", "#".repeat(boundary.x as usize + 2)).expect("to have written");
        for y in 0..boundary.y{
            let mut rowstr = String::new();
            rowstr.push('#');
            for x in 0..boundary.x{
                let pt = Point::new(x, y);
                if boxes.iter().any(|b| b.area.contains(&pt)){
                    rowstr.push('O');
                    continue;
                }
                if walls.contains(&pt){
                    rowstr.push('#');
                    continue;
                }
                if pt == *robot{
                    rowstr.push('@');
                    continue;
                }
                rowstr.push('.');
            }
            rowstr.push('#');
            writeln!(file, "{rowstr}").expect("to have written");
        }
        writeln!(file, "{}", "#".repeat(boundary.x as usize + 2)).expect("to have written");
        writeln!(file).expect("to have written");

        let ch = match direction {
            &LEFT => '<',
            &UP => '^',
            &RIGHT =>  '>',
            &DOWN  => 'v',
            _ => '?'
        };
        writeln!(file, "moving robot in direction {ch}").expect("to have written");
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let (boxes, walls, boundary, directions, robot) = parse(input, false);
    let (boxes_copy, robot) = apply_directions(boxes.clone(), &walls, &boundary, robot, &directions);
    pretty_print(&boxes_copy, &walls, &boundary, &robot, false);
    get_warehouse_gps_value(&boxes_copy)
}

#[inline]
pub fn part2(input: &str) -> i32{
    let (boxes, walls, boundary, directions, robot) = parse(input, true);
    let (boxes_copy, robot) = apply_directions(boxes.clone(), &walls, &boundary, robot, &directions);
    pretty_print(&boxes_copy, &walls, &boundary, &robot, true);
    get_warehouse_gps_value_2(&boxes_copy)
}

crate::aoctest!(10092, 1552463, 9021, 1554058);