#![allow(dead_code)]

mod loader;
mod day1;
mod day2;
mod day3;

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn day1() {
        let result = day1::part1();
        println!("{:?}", result);

        let result2 = day1::part2();
        println!("{:?}", result2);

        assert!(true);
    }

    #[test]
    fn day2() {
        let result = day2::part1();
        println!("day2::part1 => {:?}", result);

        let result2 = day2::part2();
        println!("day2::part2 => {:?}", result2);
    }

    #[test]
    fn day3() {
        let result = day3::part1();
        println!("day3::part1 => {:?}", result);
    }
}
