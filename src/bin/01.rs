use std::str::FromStr;

advent_of_code::solution!(1);

#[derive(Debug)]
struct Dial {
    value: i16,
    password: u64,
    password_new_method: u64,
}

impl Dial {
    pub fn new() -> Self {
        Self {
            value: 50,
            password: 0,
            password_new_method: 0,
        }
    }

    pub fn rotate(&mut self, rotation: &Rotation) {
        let was_zero = self.value == 0;

        self.value += rotation.value();

        self.password_new_method += (self.value / 100).unsigned_abs() as u64;

        if !was_zero && self.value <= 0 {
            self.password_new_method += 1;
        }

        self.value = self.value.rem_euclid(100);

        if self.value == 0 {
            self.password += 1;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Rotation {
    Left(i16),
    Right(i16),
}

impl Rotation {
    pub fn value(&self) -> i16 {
        match self {
            Rotation::Left(amount) => -amount,
            Rotation::Right(amount) => *amount,
        }
    }
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

fn run(input: &str) -> Dial {
    let mut dial = Dial::new();
    let rotations = input.lines().map(|line| line.parse::<Rotation>().unwrap());
    for rotation in rotations {
        dial.rotate(&rotation)
    }
    dial
}

pub fn part_one(input: &str) -> Option<u64> {
    let dial = run(input);
    Some(dial.password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let dial = run(input);
    Some(dial.password_new_method)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn dial() {
        let mut dial = Dial::new();
        dial.rotate(&Rotation::Left(68));
        assert_eq!(dial.value, 82);
        assert_eq!(dial.password, 0);
        assert_eq!(dial.password_new_method, 1);
        dial.rotate(&Rotation::Left(30));
        assert_eq!(dial.value, 52);
        assert_eq!(dial.password, 0);
        assert_eq!(dial.password_new_method, 1);
        dial.rotate(&Rotation::Right(48));
        assert_eq!(dial.value, 0);
        assert_eq!(dial.password, 1);
        assert_eq!(dial.password_new_method, 2);
        dial.rotate(&Rotation::Left(5));
        assert_eq!(dial.value, 95);
        assert_eq!(dial.password, 1);
        assert_eq!(dial.password_new_method, 2);
        dial.rotate(&Rotation::Right(60));
        assert_eq!(dial.value, 55);
        assert_eq!(dial.password, 1);
        assert_eq!(dial.password_new_method, 3);
        dial.rotate(&Rotation::Left(55));
        assert_eq!(dial.value, 0);
        assert_eq!(dial.password, 2);
        assert_eq!(dial.password_new_method, 4);
        dial.rotate(&Rotation::Left(1));
        assert_eq!(dial.value, 99);
        assert_eq!(dial.password, 2);
        assert_eq!(dial.password_new_method, 4);
        dial.rotate(&Rotation::Left(99));
        assert_eq!(dial.value, 0);
        assert_eq!(dial.password, 3);
        assert_eq!(dial.password_new_method, 5);
        dial.rotate(&Rotation::Right(14));
        assert_eq!(dial.value, 14);
        assert_eq!(dial.password, 3);
        assert_eq!(dial.password_new_method, 5);
        dial.rotate(&Rotation::Left(82));
        assert_eq!(dial.value, 32);
        assert_eq!(dial.password, 3);
        assert_eq!(dial.password_new_method, 6);
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
