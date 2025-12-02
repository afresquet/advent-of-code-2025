use std::{num::ParseIntError, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug)]
struct RangeIds {
    first: u64,
    last: u64,
}

impl RangeIds {
    pub fn new(first: u64, last: u64) -> Self {
        Self { first, last }
    }

    pub fn invalid_ids(&self) -> impl Iterator<Item = u64> {
        (self.first..=self.last).filter(|id| {
            let s = id.to_string();
            if s.len() % 2 != 0 {
                return false;
            }
            let half = s.len() / 2;
            s[..half] == s[half..]
        })
    }
}

impl FromStr for RangeIds {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, last) = s.split_once('-').expect("correct formatting");
        Ok(Self::new(first.parse()?, last.parse()?))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let range_ids = input.trim().split(',').map(|range| {
        range
            .parse::<RangeIds>()
            .unwrap_or_else(|_| panic!("{range}"))
    });

    Some(
        range_ids
            .map(|range| range.invalid_ids().sum::<u64>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn invalid_ids() {
        assert_eq!(
            vec![11, 22],
            RangeIds::new(11, 22).invalid_ids().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![99],
            RangeIds::new(95, 115).invalid_ids().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![1010],
            RangeIds::new(998, 1012).invalid_ids().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![1188511885],
            RangeIds::new(1188511880, 1188511890)
                .invalid_ids()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![222222],
            RangeIds::new(222220, 222224)
                .invalid_ids()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<u64>::new(),
            RangeIds::new(1698522, 1698528)
                .invalid_ids()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![446446],
            RangeIds::new(446443, 446449)
                .invalid_ids()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![38593859],
            RangeIds::new(38593856, 38593862)
                .invalid_ids()
                .collect::<Vec<_>>()
        );
    }
}
