#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str =
        "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";
    #[test]
    fn test_part1() {
        env_logger::init();
        assert_eq!(part1(TEST_INPUT).unwrap(), 357)
    }
}

use std::fmt::Debug;
use std::fs::read_to_string;

use nom::character::complete::{digit1, line_ending};
use nom::combinator::map_res;
use nom::error::Error;
use nom::{Finish, IResult, Parser};
use nom::{multi::many1, sequence::terminated};

fn digits(input: &str) -> IResult<&str, Vec<u32>> {
    let digits = terminated(digit1, line_ending);
    map_res(digits, |line: &str| {
        line.chars()
            .map(|s| s.to_digit(10).ok_or_else(|| "failed"))
            .collect()
    })
    .parse(input)
}

fn parse_input(input: &str) -> Result<(&str, Vec<Vec<u32>>), Error<&str>> {
    many1(digits).parse(input).finish()
}

fn part1(input: &str) -> Option<usize> {
    let parsed_input = parse_input(input);
    match parsed_input {
        Ok((_, input)) => {
            log::debug!("{:?}", input);
            let jolts: u32 = input
                .into_iter()
                .map(|battery: Vec<u32>| {
                    let (idx, a) = battery[..battery.len() - 1].iter().enumerate().fold(
                        (0, 0),
                        |cur, (idx, a)| -> (usize, u32) {
                            if *a > cur.1 { (idx, *a) } else { cur }
                        },
                    );
                    log::debug!("{:?}\n{:?}, {:?}", battery, idx, a);
                    let b = battery[idx + 1..].iter().max().unwrap();
                    let prod = format!("{}{}", a, b);

                    log::debug!("jolts: {:?}", prod);
                    prod.parse().unwrap()
                })
                .collect::<Vec<u32>>()
                .iter()
                .sum();
            Some(jolts as usize)
        }
        Err(err) => {
            log::error!("Unable to parse input with error: {:?}", err);
            None
        }
    }
}

fn print_answer<T: Debug>(part: usize, ans: T) {
    println!("The answer to part {} is: {:?}", part, ans);
}
fn main() {
    env_logger::init();
    let input = read_to_string("./input.txt");
    match input {
        Ok(input) => print_answer(1, part1(&input)),
        Err(err) => log::error!("Failed to read input with error {:?}", err),
    }
}
