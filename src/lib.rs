pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Directions{
    UP,
    LEFT,
    RIGHT,
    DOWN
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32
}

pub fn concat_numbers(a : i64, b : i64) -> i64{
    let mut pow = 10;
    while b >= pow{
        pow *= 10;
    }
    return a * pow + b;
}