use itertools::Itertools;

#[derive(Debug)]
struct Equation{
    result_x: i64,
    result_y: i64,
    ax : i64,
    ay : i64,
    bx : i64,
    by : i64
}

impl Equation {
    const fn new () -> Equation{
        Equation{result_x: 0, result_y :0, ax : 0, ay : 0, bx : 0, by : 0}
    }
}


#[inline]
fn parse(input: &str) -> Vec<Equation> {
    let mut result = vec![];
    let mut count = 0;
    let mut eq = Equation::new();
    for line in input.lines(){
        if line.is_empty(){
            continue;
        }

        let nums = line.split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<i64>().unwrap()).collect_vec();

        match count{
            0 => {eq.ax = nums[0]; eq.ay = nums[1]},
            1 => {eq.bx = nums[0]; eq.by = nums[1]},
            2 => {eq.result_x = nums[0]; eq.result_y = nums[1]; result.push(eq); eq = Equation::new()}
            _ => {}
        };

        count = (count + 1) % 3;
    }

    result
}

fn calculate_equation(equation : &Equation) -> i64{
    let denom = equation.ax * equation.by - equation.ay * equation.bx;
    
    if denom == 0{
        return 0;
    }

    let a = (equation.bx * equation.result_y - equation.by * equation.result_x) / -denom;
    let b = (equation.ax * equation.result_y - equation.ay * equation.result_x) / denom;

    if equation.result_x == a * equation.ax + equation.bx * b{
        return a * 3 + b;
    }
    return 0;
}


#[inline]
pub fn part1(input: &str) -> i64 {
    let claw_machines = parse(input);
    claw_machines.iter().map(|machine| calculate_equation(machine)).sum()
}

#[inline]
pub fn part2(input: &str) -> i64{
    let mut claw_machines = parse(input);
    claw_machines.iter_mut().for_each(|eq| {eq.result_x += 10000000000000; eq.result_y += 10000000000000;});
    claw_machines.iter().map(|machine| calculate_equation(machine)).sum()
}

crate::aoctest!(480, 36250, 875318608908, 83232379451012);