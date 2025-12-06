// use rayon::prelude::*;
use num_traits::{One, Zero, one};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub};
use std::{ops::RangeInclusive, str::FromStr};

pub struct DayInput<T>
where
    T: Ord
        + PartialEq
        + FromStr
        + Clone
        + PartialOrd
        + Debug
        + Default
        + Sub<Output = T>
        + Add<Output = T>
        + AddAssign
        + One,
{
    ranges: Vec<RangeInclusive<T>>,
    items: Vec<T>,
}
impl<T> DayInput<T>
where
    T: FromStr
        + PartialOrd
        + Ord
        + PartialEq
        + One
        + Clone
        + Debug
        + Add<Output = T>
        + Sub<Output = T>
        + Default
        + AddAssign,
{
    pub fn new(ranges: Vec<RangeInclusive<T>>, items: Vec<T>) -> Self {
        Self { ranges, items }
    }
    pub fn count_fresh(&self) -> usize {
        self.items
            .iter()
            .map(|item| self.in_range(item))
            .filter(|x| *x)
            .count()
    }
    fn in_range(&self, item: &T) -> bool {
        self.ranges.iter().any(|range| range.contains(item))
    }
    pub fn n_in_ranges(&self) -> T {
        let mut count = T::default();
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
                    *prev = (prev.start().clone()..=r.end().clone())
                }
            } else {
                ranges_condensed.push(r.clone());
            };
        }
        log::debug!("{:?}", ranges_condensed);
        ranges_condensed
            .iter()
            .for_each(|r| count += T::one() + (r.end().clone() - r.start().clone()));
        count
    }
}
