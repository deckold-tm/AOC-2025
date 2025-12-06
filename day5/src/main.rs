use std::fmt::Debug;
use std::fs::read_to_string;
use std::ops::{Add, AddAssign, RangeInclusive, Sub};
use std::str::FromStr;
use std::sync::Once;

use num_traits::One;

use crate::logic::DayInput;
mod logic;
mod parsing;

#[allow(unused)]
const ONCE: Once = Once::new();
const INPUT: &str = "./input.txt";

fn main() {
    env_logger::init();
    let input = read_to_string(INPUT);
    match input {
        Err(err) => log::error!("Failed to read in input with err: {:?}", err),
        Ok(input) => {
            let parsed_input = parsing::parse_input::<usize>(&input);
            match parsed_input {
                Err(err) => log::error!("Failed to parse input with err: {:?}", err),
                Ok(day5) => {
                    let part1 = part1(&day5);
                    println!("The answer to part 1: {}", part1);
                    let part2 = part2(&day5);
                    println!("The answer to part 2: {}", part2);
                }
            }
        }
    }
}

fn part1<T>(parsed_input: &DayInput<T>) -> usize
where
    T: PartialEq
        + FromStr
        + PartialOrd
        + Clone
        + Ord
        + Debug
        + Sub<Output = T>
        + Add<Output = T>
        + One
        + AddAssign
        + Default,
{
    parsed_input.count_fresh()
}
fn part2<T>(parsed_input: &DayInput<T>) -> T
where
    T: PartialEq
        + FromStr
        + PartialOrd
        + Clone
        + Ord
        + Debug
        + Add<Output = T>
        + One
        + Sub<Output = T>
        + AddAssign
        + Default,
{
    parsed_input.n_in_ranges()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        ONCE.call_once(env_logger::init)
    }
    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("./example").unwrap();
        let parsed_input = parsing::parse_input::<usize>(&input).unwrap();
        assert_eq!(part1(&parsed_input), 3)
    }
    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("./example").unwrap();
        let parsed_input = parsing::parse_input::<usize>(&input).unwrap();
        assert_eq!(part2(&parsed_input), 14);
    }
}
