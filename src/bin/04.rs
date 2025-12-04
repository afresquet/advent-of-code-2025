use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

advent_of_code::solution!(4);

const NEIGHBOR_COORDS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Position {
    Roll,
    Empty,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Roll,
            _ => Self::Empty,
        }
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Position>>);

impl Grid {
    pub fn iter_neighbors(
        &self,
    ) -> impl Iterator<Item = (Position, Vec<Position>, (usize, usize))> {
        self.iter().enumerate().flat_map(move |(y, row)| {
            row.iter()
                .copied()
                .enumerate()
                .filter(|(_, position)| *position == Position::Roll)
                .map(move |(x, position)| {
                    let neighbors = NEIGHBOR_COORDS
                        .iter()
                        .flat_map(|(xd, yd)| {
                            let x = x.checked_add_signed(*xd)?;
                            let y = y.checked_add_signed(*yd)?;

                            let row = self.get(y)?;
                            row.get(x).copied()
                        })
                        .collect();

                    (position, neighbors, (x, y))
                })
        })
    }

    pub fn remove(&mut self, (x, y): (usize, usize)) {
        if let Some(position) = self.get_mut(y).and_then(|row| row.get_mut(x)) {
            *position = Position::Empty;
        }
    }
}

impl Deref for Grid {
    type Target = [Vec<Position>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .lines()
                .map(|line| line.chars().map(Position::from).collect())
                .collect(),
        ))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Grid = input.parse().unwrap();

    let accessible = grid
        .iter_neighbors()
        .filter(|(_, neighbors, _)| {
            neighbors
                .iter()
                .filter(|neighbor| matches!(*neighbor, Position::Roll))
                .count()
                < 4
        })
        .count();

    Some(accessible as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Grid = input.parse().unwrap();

    let mut accessible = 0;

    loop {
        let accessible_before = accessible;

        let accessible_to_remove = grid
            .iter_neighbors()
            .filter_map(|(_, neighbors, coords)| {
                (neighbors
                    .iter()
                    .filter(|neighbor| matches!(*neighbor, Position::Roll))
                    .count()
                    < 4)
                .then_some(coords)
            })
            .collect::<Vec<_>>();

        for coords in accessible_to_remove {
            grid.remove(coords);
            accessible += 1;
        }

        if accessible == accessible_before {
            break;
        }
    }

    Some(accessible as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
