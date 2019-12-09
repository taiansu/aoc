#![allow(dead_code)]

mod loader;
mod day1;
mod day2;

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
        println!("{:?}", result);
    }
}
