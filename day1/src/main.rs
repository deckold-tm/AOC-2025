use log;
use nom::Finish;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::{pair, terminated};
use std::fs::read_to_string;

const WRAP: isize = 100;
#[derive(Clone, Debug)]
enum Direction {
    L(isize),
    R(isize),
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map_res(pair(char('L'), parse_value), |(_, amount)| {
            Ok::<Direction, &str>(Direction::L(amount))
        }),
        map_res(pair(char('R'), parse_value), |(_, amount)| {
            Ok::<Direction, &str>(Direction::R(amount))
        }),
    ))
    .parse(input)
}
fn parse_line(input: &str) -> IResult<&str, Direction> {
    terminated(parse_direction, newline).parse(input)
}

fn parse_value(input: &str) -> IResult<&str, isize> {
    map_res(digit1, |s: &str| s.parse()).parse(input)
}

fn parse_file(input: &str) -> Result<Vec<Direction>, nom::error::Error<&str>> {
    let (_, moves) = many1(parse_line).parse(input).finish()?;
    Ok(moves)
}

struct ZeroCount {
    count: isize,
}
impl ZeroCount {
    fn new() -> ZeroCount {
        Self { count: 0 }
    }
    fn rotate1(&mut self, cur: isize, m: &Direction) -> isize {
        let new_val = match m {
            Direction::L(val) => {
                let x = cur - val;
                x.rem_euclid(WRAP)
            }
            Direction::R(val) => {
                let x = cur + val;
                x % WRAP
            }
        };
        if new_val == 0 {
            self.count += 1
        };
        new_val
    }
    fn rotate2(&mut self, cur: isize, m: &Direction) -> isize {
        match m {
            Direction::L(val) => {
                let x = if cur == 0 { 100 - val } else { cur - val };
                self.count += (x.div_euclid(WRAP)).abs();
                let new_val = x.rem_euclid(WRAP);
                if new_val == 0 {
                    self.count += 1
                };
                new_val
            }
            Direction::R(val) => {
                let x = cur + val;
                self.count += x / WRAP;
                x % WRAP
            }
        }
    }
}

fn main() {
    env_logger::init();
    let path = "./input.txt";
    if let Ok(input) = read_to_string(path) {
        let mut part1 = ZeroCount::new();
        let mut part2 = ZeroCount::new();
        if let Ok(commands) = parse_file(&input) {
            commands.iter().fold(50, |a, b| part1.rotate1(a, b));
            println!("Result of part1 is: {:?}", part1.count);
            commands.iter().fold(50, |a, b| part2.rotate2(a, b));
            println!("Result of part2 is: {:?}", part2.count);
        } else {
            log::error!("Parsing of input file failed")
        }
    } else {
        log::error!("Failed to parse input file to string {:?}", path)
    }
}
