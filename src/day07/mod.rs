use std::vec;

use crate::concat_numbers;

#[derive(Debug, Clone)]
struct Equation {
    result : i64,
    numbers : Vec<i64>
}

#[inline]
fn parse(input: &str) -> Vec<Equation> {
    let mut equations = vec![];
    for line in input.lines() {
        let split = line.chars().position(|c |c ==':').unwrap();
        let result = line[..split].parse().unwrap();
        let numbers = line[split+1..].split_ascii_whitespace().map(|v| v.parse().unwrap()).collect::<Vec<_>>();
        equations.push(Equation{
            result,
            numbers,
        });
    }
    equations
}

#[inline]
pub fn part1(input: &str) -> i64 {
    let parsed_equations = parse(input);   

    let mut result = 0;
    let mut unprocessed = vec![];
    for equation in parsed_equations{
        unprocessed.clear();
        unprocessed.push((0,0));
        while let Some((current, index)) = unprocessed.pop(){
            if index == equation.numbers.len(){
                if current == equation.result{
                    result += equation.result;
                    break;
                }
                continue;
            }
            if current > equation.result{
                continue;
            }
            unprocessed.push((current + equation.numbers[index], index+1));
            unprocessed.push((current * equation.numbers[index], index+1));
        }
    }
    
    result
}

#[inline]
pub fn part2(input: &str) -> i64{
    let parsed_equations = parse(input);
      
    let mut result = 0;
    let mut unprocessed = vec![];
    for equation in parsed_equations{
        unprocessed.clear();
        unprocessed.push((0,0));
        while let Some((current, index)) = unprocessed.pop(){
            if index == equation.numbers.len(){
                if current == equation.result{
                    result += equation.result;
                    break;
                }
                continue;
            }
            if current > equation.result{
                continue;
            }
            unprocessed.push((current + equation.numbers[index], index+1));
            unprocessed.push((current * equation.numbers[index], index+1));
            unprocessed.push((concat_numbers(current, equation.numbers[index]), index+1));
        }
    }

    result
}

crate::aoctest!(3749, 465126289353, 11387, 70597497486371);