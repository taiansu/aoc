#![allow(dead_code)]

mod loader;
mod day1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        let result = day1::part1();
        println!("{:?}", result);

        let result2 = day1::part2();
        println!("{:?}", result2);

        assert!(true);
    }
}
