advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = input.split_once("\n\n").expect("to have correct format");

    let ranges = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once('-').expect("to have correct format");
            start.parse::<u64>().expect("to be a number")
                ..=end.parse::<u64>().expect("to be a number")
        })
        .collect::<Vec<_>>();

    let ids = ids
        .lines()
        .map(|id| id.parse::<u64>().expect("to be a number"));

    let fresh = ids.filter(|id| {
        for range in &ranges {
            if range.contains(id) {
                return true;
            }
        }

        false
    });

    Some(fresh.count() as u64)
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
}
