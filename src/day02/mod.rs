#[inline]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.split(" "))
        .map(|substr| {substr.flat_map(|part| part.parse::<u32>()).collect()})
        .collect()
}

#[inline]
pub fn part1(input: &str) -> usize {
    parse(input)
    .iter()
    .filter(|line| is_monotonic(line, 3, None))
    .count()
}

#[inline]
pub fn part2(input: &str) -> usize {
    parse(input)
    .iter()
    .filter(|line| is_monotonic_safe(line))
    .count()

}

fn is_monotonic_safe(numbers: &[u32]) -> bool{
    if is_monotonic(numbers, 3, None) {
        return true;
    }

    for i in 0..numbers.len() {
        if is_monotonic(numbers, 3, Some(i)) {
           return true;
        }
    }

    false
}

fn is_monotonic(numbers: &[u32], max_diff : u32, skip_index : Option<usize>) -> bool{
    if numbers.len() == 0 || numbers.len() == 1 {
        return false;
    }
    let (first, second) = match skip_index {
        Some(0) => (1, 2),
        Some(1) => (0, 2),
        _ => (0, 1),
    };

    let is_increasing = numbers[first] < numbers[second];
    let skip_index = match skip_index {
        Some(s) => s,
        None => numbers.len() + 1
    };

    numbers
    .iter()
    .enumerate()
    .filter(|(i, _)| i != &skip_index)
    .zip(numbers.iter().enumerate().filter(|(i, _)| i != &skip_index).skip(1))
    .map(|((_, a), (_,  b))| {
        let is_valid = a.abs_diff(*b) <= max_diff;
        let result = if is_increasing  {a < b && is_valid} else {a > b && is_valid};
        // println!("{:?}, to_skip {skip_index}, current {i}, {a}, {b} : {result}", numbers);
        return result;
    })
    .reduce(|a, b| a && b)
    .expect("should work")
}   

crate::aoctest!(2, 252, 4, 324);