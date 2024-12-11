use std::vec;
use regex::Regex;

#[inline]
fn parse(input: &str, parse_disabled : bool) -> Vec<(i32, i32)> {
    let reg = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();

    let mut enabled = true;
    let mut result = vec![];
    for capture in reg.captures_iter(input) {
        let ctrl = capture.get(1).expect("expected ctrl val").as_str();
        match ctrl{
            "don't()" => {enabled = false; continue;}
            "do()" => {enabled = true; continue;}
            _ => if enabled || parse_disabled {
                let x = capture.get(2).expect("expected x val").as_str();
                let y = capture.get(3).expect("expected y val").as_str();
                result.push((x.parse().expect(&format!("x: {x} should be a number")), y.parse().expect(&format!("y: {y} should be a number"))))}
        }
    }
    result     
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let parsed = parse(input, true);
    
    parsed
    .iter()
    .fold(0, |acc, (x,y)| acc + x*y)
}

#[inline]
pub fn part2(input: &str) -> i32{
    let parsed = parse(input, false);
    
    parsed
    .iter()
    .fold(0, |acc, (x,y)| acc + x*y)
}

crate::aoctest!(161, 167650499, 48, 95846796);