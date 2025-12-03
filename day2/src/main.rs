use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;

use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{Finish, IResult, Parser};

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n";

    #[test]
    fn test_part1() -> () {
        let parsed_input = parse_input(TEST_INPUT).unwrap();
        assert_eq!(part1(&parsed_input), 1227775554);
    }
    #[test]
    fn test_part2() -> () {
        let parsed_input = parse_input(TEST_INPUT).unwrap();
        assert_eq!(part2(&parsed_input), 4174379265);
    }
}
#[derive(Debug, Copy, Clone)]
struct Range {
    start: usize,
    end: usize,
}
impl Range {
    fn to_strings(self) -> Vec<String> {
        (self.start..=self.end)
            .collect::<Vec<usize>>()
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
}
impl TryFrom<(&str, &str)> for Range {
    type Error = &'static str;
    fn try_from((fst, snd): (&str, &str)) -> Result<Self, Self::Error> {
        if let Ok(start) = fst.parse::<usize>() {
            if let Ok(end) = snd.parse::<usize>() {
                Ok(Self { start, end })
            } else {
                Err("Could not convert snd")
            }
        } else {
            Err("Cound not convert fst")
        }
    }
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let parser = separated_pair(digit1, char('-'), digit1);
    map_res(parser, |out: (&str, &str)| Range::try_from(out)).parse(input)
}
fn parse_input(input: &str) -> Result<Vec<Range>, nom::error::Error<&str>> {
    let (_, ranges) = separated_list1(char(','), parse_range)
        .parse(input)
        .finish()?;
    Ok(ranges)
}
fn check_bad2(input: &String) -> Option<usize> {
    let len = input.len();
    let window_len = 1..=len / 2;
    window_len
        .map(|n| {
            let mut nums = input
                .as_bytes()
                .chunks(n)
                .map(|s| unsafe { str::from_utf8_unchecked(s) })
                .collect::<Vec<&str>>();
            log::debug!("{:?}", nums);
            if let Some(head) = nums.pop() {
                nums.iter().all(|x| x.eq(&head))
            } else {
                false
            }
        })
        .any(|x| x)
        .then(|| Some(input.parse().unwrap()))?
}

fn check_bad(num: &String) -> Option<usize> {
    let len = num.len();
    let head = &num[..len / 2];
    let tail = &num[len / 2..];
    if head.eq(tail) {
        Some(num.parse::<usize>().expect("Failed to parsed num"))
    } else {
        None
    }
}

fn part1(parsed_input: &Vec<Range>) -> usize {
    log::debug!("{:?}", parsed_input);
    let ranges: Vec<String> = parsed_input
        .into_iter()
        .map(|x| x.to_strings())
        .flatten()
        .collect();
    let count: usize = ranges.iter().map(check_bad).flatten().sum();
    count
}
fn part2(parsed_input: &Vec<Range>) -> usize {
    log::debug!("{:?}", parsed_input);
    let ranges: Vec<String> = parsed_input
        .into_iter()
        .map(|x| x.to_strings())
        .flatten()
        .collect();
    let count: usize = ranges.iter().map(check_bad2).flatten().sum();
    count
}

fn print_ans<T: Debug>(part: usize, ans: T) -> () {
    println!("The answer to part {} is: {:?}", part, ans);
}

fn main() {
    env_logger::init();
    let file = read_to_string("./input.txt");
    match file {
        Ok(file) => {
            let parsed_input = parse_input(&file);
            match parsed_input {
                Ok(input) => {
                    print_ans(1, part1(&input));
                    print_ans(2, part2(&input));
                }
                Err(err) => log::error!("Parsing failed with error:\n{:?}", err),
            }
        }
        Err(err) => log::error!("Failed to read in file with error: {:?}", err),
    }
}
