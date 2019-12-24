use std::ops::RangeInclusive;
use itertools::Itertools;

fn main() {
    let input = "108457-562041";
    let part1 = count_by_rule(&input, Box::new(has_duplicate));
    println!("Day4 part1 => {:?}", part1);

    let part2 = count_by_rule(&input, Box::new(has_exactly_two_same_adjacent));
    println!("Day4 part2 => {:?}", part2);
}

fn parse_range(input: &str) -> RangeInclusive<usize> {
    let nums: Vec<usize> = input.split("-").map(|s| s.parse::<usize>().unwrap_or(0)).collect();
    nums[0]..=nums[1]
}

fn to_digits(num: usize) -> Vec<u8> {
    num.to_string().split("").map(|c| c.parse::<u8>()).filter(|o| o.is_ok()).map(|o| o.unwrap()).collect()
}

fn is_increase(nums: &Vec<u8>) -> bool {
    nums.iter().zip(nums.iter().skip(1)).all(|(prev, next)| prev <= next)
}

fn to_len_of_number(nums: &Vec<u8>) -> impl Itertools<Item=usize> {
    nums.iter().group_by(|i| *i).into_iter().map(|(_k, v)| v.collect::<Vec<_>>().len()).collect::<Vec<_>>().into_iter()
}

fn has_duplicate(nums: &Vec<u8>) -> bool {
    to_len_of_number(nums).any(|count| count > 1)
}

fn has_exactly_two_same_adjacent(nums: &Vec<u8>) -> bool {
    to_len_of_number(nums).any(|count| count == 2)
}

fn count_by_rule(input: &str, constraints: Box<dyn Fn(&Vec<u8>) -> bool>) -> usize {
    parse_range(input).map(to_digits).filter(is_increase).filter(constraints).collect::<Vec<_>>().len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_range_pass() {
        assert_eq!(
            parse_range("123-456"),
            RangeInclusive::new(123, 456)
        );
    }

    #[test]
    fn to_digits_pass() {
        assert_eq!(
            to_digits(123),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn is_increase_pass() {
        assert_eq!(
            is_increase(vec![1, 2, 3].as_ref()),
            true
        );
        assert_eq!(
            is_increase(vec![1, 1, 3].as_ref()),
            true
        );
        assert_eq!(
            is_increase(vec![1, 4, 3].as_ref()),
            false
        );

    }

    #[test]
    fn has_duplicate_pass() {
        assert_eq!(
            has_duplicate(vec![2, 2, 3].as_ref()),
            true
        );

        assert_eq!(
            has_duplicate(vec![1, 2, 3].as_ref()),
            false
        );
    }

    #[test]
    fn has_exactly_two_same_adjacent_pass() {
        assert_eq!(
            has_exactly_two_same_adjacent(vec![1, 2, 2, 2, 3].as_ref()),
            false
        );

        assert_eq!(
            has_exactly_two_same_adjacent(vec![1, 1, 1, 1, 2, 2].as_ref()),
            true
        );
    }
}
