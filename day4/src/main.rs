use ndarray::{Array2, array};
mod matricies;
mod parsing;
use crate::matricies::{convolv, erode};
use crate::parsing::{ParsingError, parse_input};
use std::{fs::read_to_string, path::Path, sync::Once};

struct Main {
    input: parsing::Map,
    inverse: Array2<u8>,
}
impl Main {
    fn new(path: &Path) -> Result<Self, ParsingError<String>> {
        INIT.call_once(|| env_logger::init());
        let raw_input = read_to_string(path)?;
        let input = parse_input(raw_input)?;
        let inverse = array![[1, 1, 1], [1, 1, 1], [1, 1, 1]];
        Ok(Self { input, inverse })
    }
    fn part2(&self) -> usize {
        let mut map = self.input.clone().get();
        let mut count: usize = 0;
        loop {
            let out = self.part1_loop(&map);
            let sum = out.iter().filter(|x| **x).count();
            count += sum;
            if sum == 0 {
                break;
            }
            erode(&mut map, &out);
        }
        count
    }
    fn part1(&self) -> usize {
        let map: Array2<u8> = self.input.get();
        let out = self.part1_loop(&map);
        out.into_iter().filter(|x| *x).count()
    }
    fn part1_loop(&self, arr: &Array2<u8>) -> Array2<bool> {
        let n_neighbours = convolv(&arr, &self.inverse) * arr;
        log::debug!("Mult\n\n{:?}\n\n", n_neighbours);
        let output = n_neighbours.map(|x| (*x <= 4) & (*x > 0));
        log::debug!("Bool\n\n{:?}\n\n", output);
        output
    }
}

fn main() {
    let path = Path::new("./input2.txt");
    if let Ok(m) = Main::new(&path) {
        let part1 = m.part1();
        println!("The answer to part 1 is: {:?}", part1);
        let part2 = m.part2();
        println!("The answer to part 2 is: {:?}", part2);
    }
}

static INIT: Once = Once::new();
#[cfg(test)]
mod tests {
    use ndarray::array;

    fn init_logging() {
        INIT.call_once(|| {
            let _ = env_logger::builder()
                .is_test(true)
                .try_init()
                .expect("Failed to initialise logging");
        })
    }

    use super::*;
    #[test]
    fn test_parsing() {
        init_logging();
        let path = Path::new("./test.txt");
        assert!(path.exists(), "Test file doesn't exist");
    }
    #[test]
    fn test_identity_conv() {
        init_logging();
        let identity = array![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        let test: Array2<u32> = Array2::ones((5, 5));
        assert_eq!(convolv(&test, &identity), test)
    }
    #[test]
    fn test_part1() {
        init_logging();
        let m = Main::new(&Path::new("./test.txt")).unwrap();
        assert_eq!(m.part1(), 13)
    }
    #[test]
    fn test_part2() {
        init_logging();
        let m = Main::new(&Path::new("./test.txt")).unwrap();
        assert_eq!(m.part2(), 43)
    }
}
