#![allow(dead_code)]
mod loader;
mod day1;
mod day2;
mod day3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc201901() {
        let result = day1::part1();
        println!("{:?}", result);

        let result2 = day1::part2();
        println!("{:?}", result2);

        assert!(true);
    }

    #[test]
    fn aoc201902() {
        let result = day2::part1();
        println!("day2::part1 => {:?}", result);

        let result2 = day2::part2();
        println!("day2::part2 => {:?}", result2);
    }

    #[test]
    fn aoc201903() {
        let result = day3::part1();
        println!("day3::part1 => {:?}", result);

        let result2 = day3::part2();
        println!("day3::part2 => {:?}", result2);
    }
}
