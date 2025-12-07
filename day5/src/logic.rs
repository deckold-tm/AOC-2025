// use rayon::prelude::*;
use num_traits::{One, Zero};
use std::ops::{Add, AddAssign, Sub};
use std::{ops::RangeInclusive, str::FromStr};

pub struct DayInput<T> {
    ranges: Vec<RangeInclusive<T>>,
    items: Vec<T>,
}
impl<T> DayInput<T> {
    pub fn new(ranges: Vec<RangeInclusive<T>>, items: Vec<T>) -> Self {
        Self { ranges, items }
    }
    pub fn part1(&self) -> usize
    where
        T: PartialOrd,
    {
        self.items
            .iter()
            .map(|item| self.in_range(item))
            .filter(|x| *x)
            .count()
    }
    fn in_range(&self, item: &T) -> bool
    where
        T: PartialOrd,
    {
        self.ranges.iter().any(|range| range.contains(item))
    }
    pub fn part2(&self) -> T
    where
        T: FromStr + Ord + One + Zero + Clone + Add<Output = T> + Sub<Output = T> + AddAssign,
    {
        let mut count = T::zero();
        let mut ranges = self.ranges.clone();
        ranges.sort_by_key(|range| range.start().clone());
        let mut rs = ranges.into_iter();
        let mut ranges_condensed: Vec<RangeInclusive<T>> = Vec::new();
        ranges_condensed.push(rs.next().expect("List is currently empty"));
        for r in rs {
            let prev = ranges_condensed
                .last_mut()
                .expect("ranges_condensed can't be empty");
            if prev.contains(r.start()) {
                if !prev.contains(r.end()) {
                    *prev = prev.start().clone()..=r.end().clone()
                }
            } else {
                ranges_condensed.push(r.clone());
            };
        }
        ranges_condensed
            .iter()
            .for_each(|r| count += T::one() + (r.end().clone() - r.start().clone()));
        count
    }
}
