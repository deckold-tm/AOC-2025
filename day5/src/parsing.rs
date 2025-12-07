use crate::DayInput;
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::map_res;
use nom::error::Error as _NomError;
use nom::multi::{many0, many1};
use nom::sequence::{pair, separated_pair, terminated};
use nom::{Finish, IResult, Parser};
use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum ParsingError<T> {
    #[error("")]
    NomError(#[from] _NomError<T>),
}

pub fn parse_input<T>(input: &str) -> Result<DayInput<T>, ParsingError<&str>>
where
    T: FromStr + PartialOrd,
{
    let (_, (ranges, items)) = pair(top, bottom).parse(input).finish()?;
    Ok(DayInput::new(ranges, items))
}

fn number<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(digit1, |x: &str| x.parse()).parse(input)
}

fn range<T>(input: &str) -> IResult<&str, RangeInclusive<T>>
where
    T: Sized + PartialOrd + FromStr,
{
    let (rem, (start, end)) = terminated(
        separated_pair(number::<T>, char('-'), number::<T>),
        line_ending,
    )
    .parse(input)?;
    Ok((rem, (start..=end)))
}
fn top<T>(input: &str) -> IResult<&str, Vec<RangeInclusive<T>>>
where
    T: FromStr + PartialOrd,
{
    terminated(many1(range), line_ending).parse(input)
}

fn bottom<T>(input: &str) -> IResult<&str, Vec<T>>
where
    T: FromStr,
{
    terminated(many1(terminated(number, line_ending)), many0(line_ending)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_parsing_input() {
        let text = read_to_string("./example").unwrap();
        let (rem, parsed_top) = top::<usize>(&text).unwrap();
        log::debug!("Parsed input for the top:\n {:?}", parsed_top);
        let top_expected: Vec<Vec<usize>> = (vec![(3..=5), (10..=14), (16..=20), (12..=18)])
            .into_iter()
            .map(|x| x.collect::<Vec<usize>>())
            .collect();
        assert_eq!(
            &parsed_top
                .into_iter()
                .map(|x| x.collect::<Vec<usize>>())
                .collect::<Vec<Vec<usize>>>(),
            &top_expected
        );
        let (rem, parsed_bottom) = bottom::<usize>(rem).unwrap();
        let expected_bottom: Vec<usize> = vec![1, 5, 8, 11, 17, 32];
        log::debug!("Parsed bottom is:\n {:?}", parsed_bottom);
        assert_eq!(parsed_bottom, expected_bottom);
        assert_eq!(rem, "")
    }
}
