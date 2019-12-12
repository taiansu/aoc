use crate::loader::read_priv;

pub fn part1() -> i32 {
    let modules_mass = input_lines();
    modules_fuel(modules_mass)
}

pub fn part2() -> i32 {
    let modules_mass = input_lines();

    modules_mass.iter()
    .map(|s| s.parse::<i32>().unwrap())
    .flat_map(|i| dependent_calc(i))
    .sum()
}

fn modules_fuel(modules_mass: Vec<String>) -> i32 {
    modules_mass.iter()
    .map(|s| s.parse::<i32>().unwrap())
    .map(|i| calc(i))
    .sum()
}

fn dependent_calc(mass: i32) -> Vec<i32> {
    let mut total_fuel: Vec<i32> = vec![];
    let mut current_fuel = mass;

    while current_fuel >= 0 {
        current_fuel = calc(current_fuel);
        total_fuel.push(current_fuel);
    }

    total_fuel
}

fn calc(num: i32) -> i32 {
    (num as f32 / 3.0).floor() as i32 - 2
}

fn input_lines() -> Vec<String> {
    let content = read_priv(module_path!());
    content.trim_end().split("\n").map(|s| String::from(s)).collect()
}
