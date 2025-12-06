#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str =
        "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";
    #[test]
    fn test_part1() {
        // env_logger::init();
        assert_eq!(part1(TEST_INPUT), Ok(357))
    }
    #[test]
    fn test_part2() {
        env_logger::init();
        assert_eq!(part2(TEST_INPUT), Ok(3121910778619))
    }
}

use nom::character::complete::{digit1, line_ending};
use nom::{Finish, IResult, Parser};
use nom::{combinator::map_res, error::Error};
use nom::{multi::many1, sequence::terminated};
use std::{fmt::Debug, fs::read_to_string, iter::Sum};

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
fn battery_check1(battery: &Vec<u32>) -> usize {
    let (idx, a) = battery[..battery.len() - 1]
        .iter()
        .enumerate()
        .fold((0, 0), |cur, (idx, a)| -> (usize, u32) {
            if *a > cur.1 { (idx, *a) } else { cur }
        });
    // log::debug!("{:?}\n{:?}, {:?}", battery, idx, a);
    let b = battery[idx + 1..].iter().max().unwrap();
    let prod = format!("{}{}", a, b);
    // log::debug!("jolts: {:?}", prod);
    prod.parse().expect("Parsing back into usize failed")
}
fn battery_check2(battery: &Vec<u32>) -> usize {
    const MAX: usize = 12;
    let mut vals: Vec<u32> = Vec::new();
    let len = battery.len();
    let mut ii = 0;
    log::debug!("current battery: {:?}", battery);
    for n in 1..=MAX {
        let max = len - MAX;
        log::debug!("current battery slice: {:?}", battery[ii..max + n].to_vec());
        let (idx, a) = battery[ii..max + n]
            .iter()
            .enumerate()
            .fold((0, 0), |cur, (idx, a)| -> (usize, u32) {
                if *a > cur.1 { (idx, *a) } else { cur }
            });
        vals.push(a);
        ii += idx + 1;
    }
    let prod = vals
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    log::debug!("jolts: {:?}", prod);
    prod.parse().expect("Parsing back into usize failed")
}

fn sum_jolts<T>(input: &str, battery_check: fn(&Vec<u32>) -> T) -> Result<T, Error<&str>>
where
    T: Sum,
{
    let parsed_input = parse_input(input);
    match parsed_input {
        Ok((_, input)) => {
            log::debug!("{:?}", input);
            let jolts = input
                .iter()
                .map(battery_check)
                .collect::<Vec<T>>()
                .into_iter()
                .sum();
            Ok(jolts)
        }
        Err(err) => {
            log::error!("Unable to parse input with error: {:?}", err);
            Err(err)
        }
    }
}
fn part1(input: &str) -> Result<usize, Error<&str>> {
    sum_jolts(input, battery_check1)
}
fn part2(input: &str) -> Result<usize, Error<&str>> {
    sum_jolts(input, battery_check2)
    // todo!()
}

fn print_answer<T: Debug>(part: usize, ans: T) {
    println!("The answer to part {} is: {:?}", part, ans);
}
fn main() {
    env_logger::init();
    let input = read_to_string("./input.txt");
    match input {
        Ok(input) => {
            if let Ok(ans1) = part1(&input) {
                print_answer(1, ans1)
            }
            if let Ok(ans2) = part2(&input) {
                print_answer(2, ans2)
            }
        }
        Err(err) => log::error!("Failed to read input with error {:?}", err),
    }
}
