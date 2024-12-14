use std::ops::{self, Add, Mul};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;

#[macro_export]
macro_rules! aoctest {
    ($op_1_test: expr, $op_1: expr, $op_2_test: expr, $op_2: expr) => {
        #[cfg(test)]
        mod tests {
            static TEST_INPUT: &str = include_str!("test-input.txt");
            static FULL_INPUT: &str = include_str!("input.txt");

            #[test]
            fn part_1_test() {
                let output = super::part1(TEST_INPUT);
                assert_eq!(output, $op_1_test);
            }

            #[test]
            fn part_1() {
                let output = super::part1(FULL_INPUT);
                assert_eq!(output, $op_1);
            }

            #[test]
            fn part_2_test() {
                let output = super::part2(TEST_INPUT);
                assert_eq!(output, $op_2_test);
            }

            #[test]
            fn part_2() {
                let output = super::part2(FULL_INPUT);
                assert_eq!(output, $op_2);
            }
        }
    };
}

pub const UP : Point = Point::new(0, -1);
pub const DOWN : Point = Point::new(0, 1);
pub const LEFT : Point = Point::new(-1, 0);
pub const RIGHT : Point = Point::new(1, 0);

pub const ORTHOGONAL : [Point; 4] = [UP, RIGHT, DOWN, LEFT];

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Directions{
    UP,
    LEFT,
    RIGHT,
    DOWN
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    const fn new (x : i32, y : i32) -> Point{
        Point{x,y}
    }
    fn distance(&self, other : &Point) -> Point {
        return Point{ x: self.x - other.x, y : self.y - other.y};
    }

    fn in_bounds(&self, bounds : &(i32, i32)) -> bool{
        return self.x >= 0 && self.x < bounds.0 && self.y >= 0 && self.y < bounds.1
    }

    fn wrap(&self, bounds : &Point) -> Point{
        Point::new(self.x.rem_euclid(bounds.x),self.y.rem_euclid(bounds.y))
    }
}

impl Add for Point{
    type Output = Point;

    fn add(self, other: Point) -> Point{
        Point { x: self.x + other.x, y: self.y + other.y }
    }
    
}

impl ops::Mul<i32> for &Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub fn concat_numbers(a : i64, b : i64) -> i64{
    let mut pow = 10;
    while b >= pow{
        pow *= 10;
    }
    return a * pow + b;
}