use std::cmp::Ordering;

#[inline]
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .filter_map(|(first, second)| {
            let first = first.parse::<i32>().ok();
            let second = second.parse::<i32>().ok();
            first.zip(second)
        })
        .unzip()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let (mut first, mut second) = parse(input);
    first.sort_unstable();
    second.sort_unstable();
    first
        .iter()
        .zip(second.iter())
        .map(|(first, second)| (first - second).abs())
        .sum()
}

#[inline]
pub fn part2(input: &str) -> i32{
    let (mut first, mut second) = parse(input);
    first.sort_unstable();
    second.sort_unstable();

    let mut result = 0;
    for left_el in first.iter(){
        let mut sum = 0;
        for right_el in second.iter(){
            match left_el.cmp(right_el){
                Ordering::Greater => continue,
                Ordering::Less => break,
                Ordering::Equal => sum += left_el
            }
        }
        result += sum;
    }
    return result;
}

crate::aoctest!(11, 1830467, 31, 26674158);