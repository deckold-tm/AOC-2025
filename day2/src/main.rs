use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::{Finish, IResult, Parser};

const TEST: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
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

fn main() {
    env_logger::init();
    log::warn!("Hello, world!");
    log::warn!("{:?}", parse_input(TEST))
}
