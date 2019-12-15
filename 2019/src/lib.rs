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

    #[ignore]
    #[test]
    fn day2() {
        let result = day2::part2();
        println!("{:?}", result);
    }

    #[test]
    fn day3() {
        let result = day3::part1();
        println!("{:?}", result);
    }
}
