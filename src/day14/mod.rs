use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

use crate::{Point, ORTHOGONAL};

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Robot{
    position : Point,
    velocity : Point,
}

impl Robot {
    const fn new (position: Point, velocity: Point) -> Robot{
        Robot{position, velocity}
    }

    fn update_ticks(&self, ticks : i32, bounds : &Point) -> Robot{
        let position = self.position + &self.velocity * ticks;
        return Robot::new(position.wrap(bounds), self.velocity)
    }
}


#[inline]
fn parse(input: &str) -> Vec<Robot> {
    let mut result = vec![];
    let re: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in input.lines() {
        let cap = re.captures(&line).unwrap();
        result.push(Robot {
            position: Point { x: cap.get(1).unwrap().as_str().parse::<i32>().unwrap(), y:  cap.get(2).unwrap().as_str().parse::<i32>().unwrap() } ,
            velocity: Point { x: cap.get(3).unwrap().as_str().parse::<i32>().unwrap(), y:  cap.get(4).unwrap().as_str().parse::<i32>().unwrap() } ,
        });
    }

    result
}

fn get_safety_factor(robots: Vec<Robot>, bounds : &Point) -> i64{
    let mut counts = Vec::from([0i64; 4]);
    let middle_height = bounds.y / 2;
    let middle_width = bounds.x / 2;
    for robot in robots {
        if robot.position.x < middle_width && robot.position.y < middle_height {
            counts[0] += 1;
        }
        if robot.position.x < middle_width && robot.position.y > middle_height {
            counts[1] += 1;
        }
        if robot.position.x > middle_width && robot.position.y < middle_height {
            counts[2] += 1;
        }
        if robot.position.x > middle_width && robot.position.y > middle_height {
            counts[3] += 1;
        }
    }
    println!("Quadrants: {:?}", counts);
    return counts.iter().fold(1, |a, b| a * b)
}

fn get_clustered(robots : &Vec<Robot>) -> i64{
    let hashed: HashSet<&Robot> = HashSet::from_iter(robots);
    robots.iter().filter(|robot| ORTHOGONAL.iter().any(|dir| hashed.contains(&Robot { position: robot.position + *dir , velocity: robot.velocity}))).count() as i64
}

fn print_robots(robots : &Vec<Robot>){
    for y in 0..103{
        let mut rowstr = String::new();
        for x in 0..101{
            let pt = Point::new(x, y);
            rowstr += &robots.iter().filter(|robot| robot.position == pt).count().to_string()
        }
        println!("{rowstr}");
    }
}

// test input bounds: 11,7
// main input bounds: 101, 103

#[inline]
pub fn part1(input: &str) -> i64 {
    let robots = parse(input);
    let bounds = Point::new(101, 103);
    let moved_robots = robots.iter().map(|robot| robot.update_ticks(100, &bounds)).collect_vec();
    get_safety_factor(moved_robots, &bounds)
}

#[inline]
pub fn part2(input: &str) -> i32{
    let robots = parse(input);
    let bounds = Point::new(101, 103);
    let mut i = 0;

    loop{
        let moved_robots = robots.iter().map(|robot| robot.update_ticks(i, &bounds)).collect_vec();
        if get_clustered(&moved_robots) > 200{
            print_robots(&moved_robots);
            println!("{}", get_clustered(&moved_robots));
            return i;
        }
        i+=1;
    }
}

crate::aoctest!(12, 218619324, 10, 6446);