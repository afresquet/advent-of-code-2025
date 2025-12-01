use std::str::FromStr;

advent_of_code::solution!(1);

#[derive(Debug)]
struct Dial(u8);

impl Dial {
    pub fn new() -> Self {
        Self(50)
    }

    pub fn rotate(&mut self, rotation: &Rotation) -> u8 {
        match rotation {
            Rotation::Left(amount) => {
                let amount = (amount % 100) as u8;
                match self.0.checked_sub(amount) {
                    Some(amount) => self.0 = amount,
                    None => self.0 = 100 - (amount - self.0),
                }
            }
            Rotation::Right(amount) => self.0 = ((self.0 as u16 + amount) % 100) as u8,
        }

        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Rotation {
    Left(u16),
    Right(u16),
}

impl FromStr for Rotation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_at(1);
        let amount = amount.parse().map_err(|_| "not a number")?;
        match direction {
            "L" => Ok(Self::Left(amount)),
            "R" => Ok(Self::Right(amount)),
            _ => Err("invalid format"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::new();
    let rotations = input.lines().map(|line| line.parse::<Rotation>().unwrap());
    let password = rotations
        .filter(|rotation| dial.rotate(rotation) == 0)
        .count() as u64;
    Some(password)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn dial() {
        let mut dial = Dial::new();
        assert_eq!(dial.rotate(&"L68".parse().unwrap()), 82);
        assert_eq!(dial.rotate(&"L30".parse().unwrap()), 52);
        assert_eq!(dial.rotate(&"R48".parse().unwrap()), 0);
        assert_eq!(dial.rotate(&"L5".parse().unwrap()), 95);
        assert_eq!(dial.rotate(&"R60".parse().unwrap()), 55);
        assert_eq!(dial.rotate(&"L55".parse().unwrap()), 0);
        assert_eq!(dial.rotate(&"L1".parse().unwrap()), 99);
        assert_eq!(dial.rotate(&"L99".parse().unwrap()), 0);
        assert_eq!(dial.rotate(&"R14".parse().unwrap()), 14);
        assert_eq!(dial.rotate(&"L82".parse().unwrap()), 32);
    }

    #[test]
    fn rotation() {
        assert_eq!(Rotation::Left(68), "L68".parse().unwrap());
        assert_eq!(Rotation::Left(30), "L30".parse().unwrap());
        assert_eq!(Rotation::Right(48), "R48".parse().unwrap());
        assert_eq!(Rotation::Left(5), "L5".parse().unwrap());
        assert_eq!(Rotation::Right(60), "R60".parse().unwrap());
        assert_eq!(Rotation::Left(55), "L55".parse().unwrap());
        assert_eq!(Rotation::Left(1), "L1".parse().unwrap());
        assert_eq!(Rotation::Left(99), "L99".parse().unwrap());
        assert_eq!(Rotation::Right(14), "R14".parse().unwrap());
        assert_eq!(Rotation::Left(82), "L82".parse().unwrap());
    }
}
