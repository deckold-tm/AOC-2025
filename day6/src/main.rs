mod parser;
mod part2;
use part2::Part2;
use std::fs::read_to_string;
const FILE: &str = "./input.txt";

fn main() {
    env_logger::init();
    let input = read_to_string(FILE);
    match input {
        Err(err) => log::error!("Couldn't read in file with error: {:?}", err),
        Ok(s) => {
            let parsed_input = parser::parse_input::<usize>(&s);
            match parsed_input {
                Err(err) => log::error!("Couldn't parse input with error: {:?}", err),
                Ok(day6) => {
                    let part1 = day6.part1();
                    println!("The answer to part1 is: {}", part1);
                }
            }
            let part2 = Part2::<usize>::new(&s);
            match part2 {
                Err(err) => log::error!("Failed to solve part2 with error: {:?}", err),
                Ok(part) => {
                    let ans = part.run();
                    println!("The answer to part2 is: {:?}", ans)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    const FILE: &str = "./example.txt";
    use super::*;
    #[test]
    fn test_part1() {
        let input = read_to_string(FILE).unwrap();
        let day6 = parser::parse_input::<usize>(&input).unwrap();
        let ans = day6.part1();
        assert_eq!(ans, 4277556)
    }
}
