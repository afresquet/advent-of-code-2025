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

    pub fn invalid_ids_multisequence(&self) -> impl Iterator<Item = u64> {
        (self.first..=self.last).filter(|id| {
            let s = id.to_string();

            'outer: for i in 1..=(s.len() / 2) {
                let (pat, mut rest) = s.split_at(i);

                while !rest.is_empty() {
                    let next = rest.trim_start_matches(pat);
                    if rest == next {
                        continue 'outer;
                    }
                    rest = next;
                }

                return true;
            }

            false
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

fn ranges(input: &str) -> impl Iterator<Item = RangeIds> {
    input
        .trim()
        .split(',')
        .map(|range| range.parse::<RangeIds>().unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        ranges(input)
            .map(|range| range.invalid_ids().sum::<u64>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        ranges(input)
            .map(|range| range.invalid_ids_multisequence().sum::<u64>())
            .sum(),
    )
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
        assert_eq!(result, Some(4174379265));
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

    #[test]
    fn invalid_ids_multisequence() {
        assert_eq!(
            vec![11, 22],
            RangeIds::new(11, 22)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![99, 111],
            RangeIds::new(95, 115)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![999, 1010],
            RangeIds::new(998, 1012)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![1188511885],
            RangeIds::new(1188511880, 1188511890)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![222222],
            RangeIds::new(222220, 222224)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            Vec::<u64>::new(),
            RangeIds::new(1698522, 1698528)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![446446],
            RangeIds::new(446443, 446449)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![38593859],
            RangeIds::new(38593856, 38593862)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![565656],
            RangeIds::new(565653, 565659)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![824824824],
            RangeIds::new(824824821, 824824827)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![2121212121],
            RangeIds::new(2121212118, 2121212124)
                .invalid_ids_multisequence()
                .collect::<Vec<_>>()
        );
    }
}
