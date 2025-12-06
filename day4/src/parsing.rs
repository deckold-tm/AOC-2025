use ndarray::Array2;
use nom::{Finish, IResult, Parser};
use nom::{branch::alt, character::complete::char, combinator::value, multi::many1};
use std::ops::Mul;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError<T> {
    #[error("")]
    ReadFileError(#[from] std::io::Error),
    #[error("")]
    ParserError(#[from] nom::error::Error<T>),
}
#[derive(Debug, Clone)]
pub struct Map(Array2<Elem>);
impl Map {
    pub fn get<T>(&self) -> Array2<T>
    where
        T: From<Elem>,
    {
        self.0.map(|x| T::from(*x))
    }
}
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Elem {
    EMPTY = 0,
    PAPER = 1,
}
impl Default for Elem {
    fn default() -> Self {
        Self::EMPTY
    }
}
impl std::fmt::Debug for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::EMPTY => write!(f, "."),
            Elem::PAPER => write!(f, "@"),
        }
    }
}
macro_rules! from_elem {
    ($($t:ty),*) => {$(
        impl From<Elem> for $t {
            fn from(value: Elem) -> $t {
                value as $t
            }
        }
        impl<'a> From<&'a Elem> for $t {
            fn from(value: &'a Elem) -> $t {
                *value as $t
            }
        }
    )*}
}
from_elem![usize, u64, u32, u16, u8];
impl<T> Mul<T> for Elem
where
    T: Mul<Output = T> + From<Elem>,
{
    type Output = T;
    fn mul(self, rhs: T) -> Self::Output {
        let lhs: T = self.into();
        lhs * rhs
    }
}
impl<'a, T> Mul<T> for &'a Elem
where
    T: Mul<Output = T> + From<&'a Elem>,
{
    type Output = T;
    fn mul(self: &'a Elem, rhs: T) -> Self::Output {
        let lhs: T = self.into();
        lhs * rhs
    }
}

pub fn parse_input(input: String) -> Result<Map, nom::error::Error<String>> {
    let (_, elems) = many1(parse_line).parse(&input).finish()?;
    let width = elems.len();
    let height = elems[0].len();
    Ok(unsafe {
        Map(Array2::from_shape_vec_unchecked(
            (width, height),
            elems.into_iter().flatten().collect(),
        ))
    })
}
fn parse_line(input: &str) -> IResult<&str, Vec<Elem>> {
    nom::sequence::terminated(parse_elements, nom::character::complete::line_ending).parse(input)
}
fn parse_elements(input: &str) -> IResult<&str, Vec<Elem>> {
    many1(alt((
        value(Elem::EMPTY, char('.')),
        value(Elem::PAPER, char('@')),
    )))
    .parse(input)
}
