use std::fs;

pub fn recursive_fuel(modules_mass: Vec<String>) -> i32 {
    modules_mass.iter()
    .map(|s| s.parse::<i32>().unwrap())
    .flat_map(|i| dependent_calc(i))
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

fn modules_fuel(modules_mass: Vec<String>) -> i32 {
    modules_mass.iter()
    .map(|s| s.parse::<i32>().unwrap())
    .map(|i| calc(i))
    .sum()
}

fn calc(num: i32) -> i32 {
    (num as f32 / 3.0).floor() as i32 - 2
}

fn input_lines() -> Vec<String> {
    let raw_contents = fs::read_to_string("./priv/2019/day1.in").expect("Error reading the file.");
    raw_contents.trim_end().split("\n").map(|s| String::from(s)).collect()
}

fn main() {
    let modules_mass = input_lines();

    let part1 = modules_fuel(modules_mass.clone());
    println!("{:?}", part1);

    let part2 = recursive_fuel(modules_mass.clone());
    println!("{:?}", part2);
}
