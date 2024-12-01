use itertools::Itertools;

advent_of_code::solution!(1);

fn parse_input_sorted(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines = input.lines();

    let left: Vec<u32> = lines
        .clone()
        .map(|line| {
            line.split_whitespace()
                .take(1)
                .map(|s| s.parse().unwrap())
                .next()
                .unwrap()
        })
        .sorted()
        .collect();

    let right: Vec<u32> = lines
        .clone()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .next()
                .unwrap()
        })
        .sorted()
        .collect();

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_input_sorted(input);
    let mut sum = 0;
    let mut ind = 0;

    for i in 0..left.len() {
        let l = left[i];
        let r = right[i];
        sum += l.abs_diff(r);
        ind += 1;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input_sorted(input);
    let mut sum = 0;

    for i in 0..left.len() {
        let l = left[i];
        let count: u32 = right.iter().filter(|&r| l == *r).count() as u32;
        sum += l * count;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
