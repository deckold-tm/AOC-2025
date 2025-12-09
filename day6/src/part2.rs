use std::fmt::Debug;
use std::ops::{Add, Mul};
use std::str::FromStr;

use ndarray::{Array1, Array2, Zip};
use nom::branch::alt;
use nom::character::complete::{char, digit1, line_ending, none_of, space0, space1};
use nom::combinator::{opt, value};
use nom::error::Error;
use nom::multi::many1;
use nom::sequence::{delimited, pair, terminated};
use nom::{Finish, IResult, Parser};
use num_traits::{One, Zero};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError<T> {
    #[error("Failed to parse input")]
    NomError(#[from] nom::error::Error<T>),
    #[error("Failed to reparse with error: {0:?}")]
    ReparsingError(String),
}

#[derive(Clone, Debug)]
#[repr(u8)]
enum Ops {
    PROD,
    SUM,
}

#[derive(Debug)]
pub struct Part2<T> {
    pub matrix: Array1<Array1<T>>,
    ops: Array1<Ops>,
}
impl<T: FromStr + Clone + Debug> Part2<T> {
    pub fn new(input: &str) -> Result<Self, ParsingError<&str>> {
        let (num_block, ops) = parse_input(input)?;
        log::trace!("{:?}", num_block);
        let matrix = reparse_num_block::<T>(&num_block);
        log::trace!("{:?}", matrix);
        match matrix {
            Err(err) => Err(ParsingError::ReparsingError(err.to_string())),
            Ok(matrix) => Ok(Self { matrix, ops }),
        }
    }
    pub fn run(&self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Zero + One + Clone,
    {
        Zip::from(&self.matrix)
            .and(&self.ops)
            .map_collect(|row, op| match op {
                Ops::SUM => row.sum(),
                Ops::PROD => row.product(),
            })
            .sum()
    }
}

fn ops(input: &str) -> IResult<&str, Vec<Ops>> {
    terminated(
        many1(delimited(
            space0,
            alt((value(Ops::PROD, char('*')), value(Ops::SUM, char('+')))),
            space1,
        )),
        many1(line_ending),
    )
    .parse(input)
}
fn num_line(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many1(none_of("*+\n")), line_ending).parse(input)
}
fn num_block(input: &str) -> IResult<&str, String> {
    let (res, block) = many1(num_line).parse(input)?;
    let arr = {
        let h = block.len();
        let w = block[0].len();
        let buff: Vec<char> = block.into_iter().flatten().collect();
        log::debug!(
            "Creating array of shape ({},{}) from buff size: {}",
            h,
            w,
            buff.len()
        );
        Array2::from_shape_vec((h, w), buff).unwrap()
    };
    log::trace!("raw arr \n{:?}", arr);
    let mut transpose = arr
        .columns()
        .into_iter()
        .map(|col| col.to_vec().into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    transpose.push_str("\n \n");
    Ok((res, transpose))
}
fn reparse_num_block<T: FromStr + Debug>(input: &str) -> Result<Array1<Array1<T>>, Error<&str>> {
    let (_, nums) = many1(terminated(
        many1(ints::<T>),
        terminated(space1, line_ending),
    ))
    .parse(input)
    .finish()?;
    // log::debug!("nums:\n{:?}", nums);
    let arr = {
        let h = nums.len();
        let w = nums[0].len();
        let buff: Array1<Array1<T>> = nums.into_iter().map(|row| Array1::from_vec(row)).collect();
        log::debug!(
            "Reparsing into array of shape ({},{}) from buff size: {}",
            h,
            w,
            buff.len()
        );
        buff
    };
    Ok(arr)
}

fn ints<T: FromStr>(input: &str) -> IResult<&str, T> {
    terminated(delimited(space0, digit1, space0), opt(line_ending))
        .map_res(|s: &str| s.parse())
        .parse(input)
}

fn parse_input(input: &str) -> Result<(String, Array1<Ops>), nom::error::Error<&str>> {
    let (_, (nums, out)) = pair(num_block, ops).parse(input).finish()?;
    Ok((nums, Array1::from_vec(out)))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n\n";

    #[test]
    fn test_parsing() {
        let part2 = Part2::<usize>::new(&INPUT).unwrap();

        let ans = part2.run();
        assert_eq!(ans, 3263827)
    }
}
