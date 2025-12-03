use std::{cmp::Ordering, str::FromStr};

advent_of_code::solution!(3);

#[derive(Debug)]
struct Bank(Vec<u8>);

impl Bank {
    fn largest_joltage(&self, batteries: usize) -> usize {
        assert!(self.0.len() >= batteries, "not enough batteries");

        let mut skip = 0;
        let mut selected = Vec::new();

        for batteries_left in (0..batteries).rev() {
            let (i, battery) = self
                .0
                .iter()
                .skip(skip)
                .take(self.0.len() - skip - batteries_left)
                .enumerate()
                .max_by(|(i, a), (j, b)| match a.cmp(b) {
                    Ordering::Equal => j.cmp(i),
                    ordering => ordering,
                })
                .expect("iterator is at least length one");

            skip += i + 1;

            selected.push(battery);
        }

        selected
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, b)| *b as usize * 10usize.pow(i as u32))
            .sum()
    }
}

impl FromStr for Bank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| c.to_digit(10).expect("is a number") as u8)
                .collect::<Vec<_>>(),
        ))
    }
}

fn run(input: &str, batteries: usize) -> Option<u64> {
    let banks = input.lines().map(|line| line.parse::<Bank>().unwrap());
    Some(
        banks
            .map(|bank| bank.largest_joltage(batteries) as u64)
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    run(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    run(input, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn largest_two_battery_joltage() {
        assert_eq!(
            "987654321111111"
                .parse::<Bank>()
                .unwrap()
                .largest_joltage(2),
            98
        );
        assert_eq!(
            "811111111111119"
                .parse::<Bank>()
                .unwrap()
                .largest_joltage(2),
            89
        );
        assert_eq!(
            "234234234234278"
                .parse::<Bank>()
                .unwrap()
                .largest_joltage(2),
            78
        );
        assert_eq!(
            "818181911112111"
                .parse::<Bank>()
                .unwrap()
                .largest_joltage(2),
            92
        );
    }

    #[test]
    fn largest_twelve_battery_joltage() {
        assert_eq!(
            "987654321111111"
                .parse::<Bank>()
                .unwrap()
                .largest_joltage(12),
            987654321111
        );
        assert_eq!(
            "811111111111119"
                .parse::<Bank>()
                .unwrap()
                .largest_joltage(12),
            811111111119
        );
        assert_eq!(
            "234234234234278"
                .parse::<Bank>()
                .unwrap()
                .largest_joltage(12),
            434234234278
        );
        assert_eq!(
            "818181911112111"
                .parse::<Bank>()
                .unwrap()
                .largest_joltage(12),
            888911112111
        );
    }
}
