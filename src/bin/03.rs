use std::str::FromStr;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Bank(Vec<u8>);

impl Bank {
    fn largest_joltage(&self) -> u8 {
        let mut left = u8::MIN;
        let mut right = u8::MIN;

        for n in &self.0 {
            let mut current = left * 10 + right;
            let current_right = right;
            let candidate = left * 10 + n;
            let candidate_shift = right * 10 + n;

            if candidate > current {
                current = candidate;
                right = *n;
            }
            if candidate_shift > current {
                left = current_right;
                right = *n;
            }
        }

        left * 10 + right
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

pub fn part_one(input: &str) -> Option<u64> {
    let batteries = input.lines().map(|line| line.parse::<Bank>().unwrap());
    Some(batteries.map(|bank| bank.largest_joltage() as u64).sum())
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn largest_joltage() {
        assert_eq!(
            "987654321111111".parse::<Bank>().unwrap().largest_joltage(),
            98
        );
        assert_eq!(
            "811111111111119".parse::<Bank>().unwrap().largest_joltage(),
            89
        );
        assert_eq!(
            "234234234234278".parse::<Bank>().unwrap().largest_joltage(),
            78
        );
        assert_eq!(
            "818181911112111".parse::<Bank>().unwrap().largest_joltage(),
            92
        );
    }
}
