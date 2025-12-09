use ndarray::{Array1, Array2, Zip};
use nom::{
    Finish, IResult, Parser,
    branch::alt,
    character::complete::{char, digit1, line_ending, space0},
    combinator::value,
    error::Error,
    multi::many1,
    sequence::{delimited, pair, terminated},
};
use num_traits::{One, Zero};
use std::{ops::Add, ops::Mul, str::FromStr};

#[derive(Clone)]
enum Operation {
    PROD,
    SUM,
}

pub struct Part1<T> {
    matrix: Array2<T>,
    operations: Array1<Operation>,
}
impl<T> Part1<T> {
    fn new(digits: Vec<Vec<T>>, ops: Vec<Operation>) -> Self {
        let width = digits.len();
        let flat: Vec<T> = digits.into_iter().flatten().collect();
        let matrix = unsafe { Array2::from_shape_vec_unchecked((width, flat.len() / width), flat) };
        let operations = Array1::from_vec(ops);
        Self { matrix, operations }
    }
    pub fn part1(&self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Clone + Zero + One,
    {
        Zip::from(self.matrix.columns())
            .and(&self.operations)
            .map_collect(|col, op| match op {
                Operation::PROD => col.product(),
                Operation::SUM => col.sum(),
            })
            .sum()
    }
}

pub fn parse_input<T: FromStr>(input: &str) -> Result<Part1<T>, Error<&str>> {
    let (_, (digits, operations)) = pair(num_block::<T>, operation_line).parse(input).finish()?;
    Ok(Part1::new(digits, operations))
}

fn num_block<T: FromStr>(input: &str) -> IResult<&str, Vec<Vec<T>>> {
    many1(number_line).parse(input)
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    terminated(digit1.map_res(|s: &str| s.parse()), space0).parse(input)
}

fn number_line<T>(input: &str) -> IResult<&str, Vec<T>>
where
    T: FromStr,
{
    delimited(space0, many1(number), line_ending).parse(input)
}

fn operation_line(input: &str) -> IResult<&str, Vec<Operation>> {
    terminated(
        delimited(space0, many1(operation), line_ending),
        line_ending,
    )
    .parse(input)
}
fn operation(input: &str) -> IResult<&str, Operation> {
    terminated(
        alt((
            value(Operation::PROD, char('*')),
            value(Operation::SUM, char('+')),
        )),
        space0,
    )
    .parse(input)
}
#[cfg(test)]
mod tests {
    use super::*;
    const NUMS: &str = " 4  73 2   95 3 2\n45  2 3 87 1 43  \n";
    const OPS: &str = " * * + * +  *\n\n";

    #[test]
    fn test_num() {
        let (rem, n) = number::<usize>("5 ").unwrap();
        assert_eq!(rem, "");
        assert_eq!(n, 5);
    }

    #[test]
    fn test_nums() {
        let (rem, output) = num_block::<usize>(NUMS).unwrap();
        assert_eq!(rem, "")
    }
    #[test]
    fn test_operations() {
        let (rem, output) = operation_line(OPS).unwrap();
        assert_eq!(rem, "")
    }
}
