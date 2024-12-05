use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let orders: Vec<(u32, u32)> = parts[0]
        .lines()
        .map(|line| {
            let split = line.split("|").collect::<Vec<&str>>();
            (split[0].parse().unwrap(), split[1].parse().unwrap())
        })
        .collect();

    let updates: Vec<Vec<u32>> = parts[1]
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(",").collect();
            split.iter().map(|x| x.parse().unwrap()).collect()
        })
        .collect();

    (orders, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);

    let mut valid_updates_index: Vec<u32> = vec![];

    for (i, update) in updates.iter().enumerate() {
        let mut validatetd_update: Vec<u32> = vec![];

        for up in update {
            let concerned: Vec<(u32, u32)> = rules
                .iter()
                .filter(|(before, after)| {
                    (before == up && update.contains(after))
                        || (after == up) && update.contains(before)
                })
                .map(|(before, after)| (*before, *after))
                .collect();

            let mut is_valid = true;
            concerned.iter().for_each(|(before, after)| {
                if up == after {
                    is_valid = validatetd_update.contains(before);
                }
            });

            if is_valid {
                validatetd_update.push(*up);
            } else {
                break;
            }
        }

        if validatetd_update == *update {
            valid_updates_index.push(i as u32);
        }
    }

    let valid_updates: Vec<Vec<u32>> = valid_updates_index
        .iter()
        .map(|i| updates[*i as usize].clone())
        .collect();

    let mut res = 0;
    valid_updates.iter().for_each(|update| {
        res += update[update.len() / 2];
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);

    let mut valid_updates_index: Vec<u32> = vec![];
    let mut invalid_updates_index: Vec<u32> = vec![];

    for (i, update) in updates.iter().enumerate() {
        let mut validatetd_update: Vec<u32> = vec![];

        for up in update {
            let concerned: Vec<(u32, u32)> = rules
                .iter()
                .filter(|(before, after)| {
                    (before == up && update.contains(after))
                        || (after == up) && update.contains(before)
                })
                .map(|(before, after)| (*before, *after))
                .collect();

            let mut is_valid = true;
            concerned.iter().for_each(|(before, after)| {
                if up == after {
                    is_valid = validatetd_update.contains(before);
                }
            });

            if is_valid {
                validatetd_update.push(*up);
            } else {
                break;
            }
        }

        if validatetd_update == *update {
            valid_updates_index.push(i as u32);
        } else {
            invalid_updates_index.push(i as u32);
        }
    }

    let invalid_updates: Vec<Vec<u32>> = invalid_updates_index
        .iter()
        .map(|i| updates[*i as usize].clone())
        .collect();

    let sorted_invalid_updates: Vec<Vec<u32>> = invalid_updates
        .par_iter()
        .map(|update| {
            let mut sorted_update = update.clone();
            sorted_update.sort_by(|a, b| {
                if rules
                    .iter()
                    .any(|(before, after)| before == a && after == b)
                {
                    std::cmp::Ordering::Less
                } else if rules
                    .iter()
                    .any(|(before, after)| before == b && after == a)
                {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            sorted_update
        })
        .collect();

    let mut res = 0;
    sorted_invalid_updates.iter().for_each(|update| {
        res += update[update.len() / 2];
    });

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
        assert_eq!(result, Some(143));
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
        assert_eq!(result, Some(123));
    }
}
