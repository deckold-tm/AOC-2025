use ndarray::{Array2, ArrayD, ArrayView2, IntoDimension, Ix2, IxDyn, Slice, Zip, array, s};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many1;
use nom::{Finish, IResult, Parser};
// use num_traits::{Zero, zero};
use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::iter::Sum;
use std::ops::{Add, Mul};
use std::path::Path;
use thiserror::Error;

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;
    #[test]
    fn test_parsing() {
        let path = Path::new("./test.txt");
        let test_input = std::fs::read_to_string(&path).unwrap();
        assert!(path.exists(), "Test file doesn't exist");
        let map = Main::new(&path).expect("Unable to create map");
        // println!("{:?}", map.input);
    }
    #[test]
    fn test_identity_conv() {
        let identity = array![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        let test: Array2<u32> = Array2::ones((5, 5));
        // println!("{:?}", pad(&test, 3, 0));
        assert_eq!(convolv(&test, &identity), test)
    }
    #[test]
    fn test_part1() {
        env_logger::init();
        let m = Main::new(&Path::new("./test.txt")).unwrap();
        assert_eq!(m.part1(), 13)
    }
}

#[derive(Error, Debug)]
enum ParsingError<T> {
    #[error("")]
    ReadFileError(#[from] std::io::Error),
    #[error("")]
    ParserError(#[from] nom::error::Error<T>),
}

struct Main {
    input: Map,
    identity: Array2<u32>,
    inverse: Array2<u32>,
}
impl Main {
    fn new(path: &Path) -> Result<Self, ParsingError<String>> {
        let raw_input = read_to_string(path)?;
        let input = parse_input(raw_input)?;
        let identity = array![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        let inverse = array![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
        Ok(Self {
            input,
            identity,
            inverse,
        })
    }
    fn part1(&self) -> usize {
        let map = self.input.0.map(|x| *x as u32);
        let output = (convolv(&map, &self.inverse) * convolv(&map, &self.identity))
            // .map(|x| *x < 4)
            // .into_iter()
            // .filter(|x| *x)
            // .count();
        ;
        log::debug!("\n\n{:?}\n\n", output);
        output
            .map(|x| (*x < 4) & (*x != 0))
            .into_iter()
            .filter(|x| *x)
            .count()
    }
}
fn pad<T>(x: &Array2<T>, pad_width: usize, fill: T) -> Array2<T>
where
    T: Clone + Debug,
{
    let new_shape: IxDyn = x
        .shape()
        .iter()
        .map(|size| 2 * pad_width + size)
        .collect::<Vec<usize>>()
        .into_dimension();
    let mut new_array = ArrayD::<T>::from_elem(new_shape, fill)
        .into_dimensionality::<Ix2>()
        .expect("Couldn't convert back to 2D.");
    x.assign_to(
        new_array.slice_each_axis_mut(|ax| Slice::from(pad_width..pad_width + x.len_of(ax.axis))),
    );
    new_array
}
fn convolv(map_array: &Array2<u32>, kernel: &Array2<u32>) -> Array2<u32> {
    // let map_array = map.0.map(|x| *x as u32);
    let kernel_size = kernel.shape();
    let pad_width = (kernel_size[0] - 1) / 2;
    assert!(
        kernel_size.iter().all(|x| *x == kernel_size[0]),
        "Not implemented for non-square kernels"
    );
    let padded = pad(&map_array, pad_width, 0);
    log::debug!("\n\n{:?}\n\n", padded);
    // let slice =
    //     padded.slice_each_axis(|ax| Slice::from(pad_width..pad_width + map_array.len_of(ax.axis)));
    // slice.indexed_iter().map(|(ii, jj), x| todo!());
    Zip::from(padded.windows(kernel.raw_dim())).map_collect(|window| (&window * kernel).sum())
    // .sum();
    // map_array.indexed_iter().map()
    // todo!()
}

fn main() {
    let path = Path::new("./input.txt");
    if let Ok(m) = Main::new(&path) {
        let part1 = m.part1();
        println!("The answer to part 1 is: {:?}", part1);
    }
}
#[derive(Debug)]
struct Map(Array2<Elem>);
#[derive(Clone, Copy)]
#[repr(u32)]
enum Elem {
    EMPTY = 0,
    PAPER = 1,
}
impl std::fmt::Debug for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::EMPTY => write!(f, "."),
            Elem::PAPER => write!(f, "@"),
        }
    }
}

fn parse_input(input: String) -> Result<Map, nom::error::Error<String>> {
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
