use rayon::prelude::*;

advent_of_code::solution!(2);


fn can_become_adjacent_differ_max_by_3(row: &[u32]) -> bool {
    if row.len() < 3 {
        return false;
    }

    for i in 0..row.len() {
        let mut new_row = row.to_vec();
        new_row.remove(i);

        if is_adjacent_differ_max_by_3(&new_row) && is_all_increasing_or_decreasing(&new_row) {
            return true;
        }
    }

    false
}

fn is_adjacent_differ_max_by_3(row: &[u32]) -> bool {
    row.windows(2).all(|w| w[0].abs_diff(w[1]) > 0 && w[0].abs_diff(w[1]) <= 3)
}

fn is_all_increasing_or_decreasing(row: &[u32]) -> bool {
    if row.len() < 2 {
        return true;
    }

    let (mut increasing, mut decreasing) = (true, true);

    for window in row.windows(2) {
        match window[0].cmp(&window[1]) {
            std::cmp::Ordering::Less => decreasing = false,
            std::cmp::Ordering::Greater => increasing = false,
            _ => {}
        }

        if !increasing && !decreasing {
            return false;
        }
    }

    increasing || decreasing
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let lines = input.lines();

     lines
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let nums = parse_input(input);

    let res: u32 = nums.par_iter().map(|row| {
        if is_all_increasing_or_decreasing(row) && is_adjacent_differ_max_by_3(row) {
             1
        } else {
            0
        }
    }).sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
let nums = parse_input(input);

    let res: u32 = nums.par_iter().map(|row| {
        if (is_all_increasing_or_decreasing(row) && is_adjacent_differ_max_by_3(row)) || can_become_adjacent_differ_max_by_3(row)  {
             1
        } else {
            0
        }
    }).sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{warn, Level};
    use tracing_subscriber::FmtSubscriber;

    #[test]
    fn test_part_one() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .pretty()
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            warn!("setting default subscriber failed: {:?}", e)
        }

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .pretty()
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            warn!("setting default subscriber failed: {:?}", e)
        }

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
