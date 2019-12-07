#![allow(dead_code)]

mod loader;
mod day1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        day1::exec();

        assert!(true);
    }
}
