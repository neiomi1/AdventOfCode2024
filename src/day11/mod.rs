use std::collections::HashMap;
use itertools::Itertools;


#[inline]
fn parse(input: &str) -> Vec<u64> {
    input.lines().next().unwrap().split_ascii_whitespace().map(|numstr| numstr.parse().unwrap()).collect_vec()
}

fn to_digits(v: u64) -> Vec<u64> {
    let mut v_copy = v;
    let mut buf: Vec<u64> = Vec::with_capacity(10);

    while v_copy > 0 {
        let n = v_copy % 10;
        v_copy = v_copy / 10;
        buf.push(n);
    }
    buf.reverse();
    buf
}

fn to_number(digits: Vec<u64>) -> u64{
    let mut digit_copy = digits.clone();
    let mut result = 0;

    let mut factor = 1;
    while let Some(last_digit) = digit_copy.pop(){
        result += factor * last_digit;
        factor *= 10;
    }
    result
}


fn simulate_stones(stones: Vec<u64>, cycles : u64) -> u64{
    let mut result = 0;
    let mut mem = HashMap::new();
    for stone in stones{
        result += simulate_stone_cached(stone, cycles, &mut mem);
    }
    result
}

fn simulate_stone_cached(stone: u64, cycles_remaining: u64, mem: &mut HashMap<(u64, u64), u64>) -> u64{
    match mem.get(&(stone, cycles_remaining)) {
        Some(result) => return *result,
        None => {let result = simulate_stone(stone, cycles_remaining, mem); mem.insert((stone, cycles_remaining), result as u64); return result as u64;},
    }  
}

fn simulate_stone(stone: u64, mut cycles_remaining: u64, mem: &mut HashMap<(u64, u64), u64>) -> u64{
    if cycles_remaining == 0{
        return 1;
    }

    cycles_remaining -= 1;
    if stone == 0{
        return simulate_stone_cached(1, cycles_remaining, mem);
    }
    let digits = to_digits(stone);
    if digits.len() % 2 == 0{
        let stones = digits.split_at(digits.len() / 2);
        return simulate_stone_cached(to_number(stones.0.to_vec()), cycles_remaining, mem) + simulate_stone_cached(to_number(stones.1.to_vec()), cycles_remaining, mem);
    }
    return simulate_stone_cached(stone * 2024, cycles_remaining, mem);
}

#[inline]
pub fn part1(input: &str) -> u64 {
    let stones = parse(input);
    simulate_stones(stones, 25)
}

#[inline]
pub fn part2(input: &str) -> u64{
    let stones = parse(input);
    simulate_stones(stones, 75)
}

crate::aoctest!(55312, 207683, 81, 1094);