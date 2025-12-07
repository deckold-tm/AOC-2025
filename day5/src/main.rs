use std::fs::read_to_string;

use crate::logic::DayInput;
mod logic;
mod parsing;

const INPUT: &str = "./input.txt";

fn main() {
    env_logger::init();
    let input = read_to_string(INPUT);
    match input {
        Err(err) => log::error!("Failed to read in input with err: {:?}", err),
        Ok(input) => {
            let parsed_input = parsing::parse_input::<usize>(&input);
            match parsed_input {
                Err(err) => log::error!("Failed to parse input with err: {:?}", err),
                Ok(day5) => {
                    let part1 = day5.part1();
                    println!("The answer to part 1: {}", part1);
                    let part2 = day5.part2();
                    println!("The answer to part 2: {}", part2);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("./example").unwrap();
        let parsed_input = parsing::parse_input::<usize>(&input).unwrap();
        assert_eq!((parsed_input.part1()), 3)
    }
    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("./example").unwrap();
        let parsed_input = parsing::parse_input::<usize>(&input).unwrap();
        assert_eq!((parsed_input.part2()), 14);
    }
}
