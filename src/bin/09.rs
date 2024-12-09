use std::usize;

use tracing::{info, trace};

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
enum Space {
    Block(u64, u64),
    Free(u64),
}

impl Space {
    fn is_block(&self) -> bool {
        match self {
            Space::Block(_, _) => true,
            _ => false,
        }
    }

    fn size(&self) -> u64 {
        match self {
            Space::Block(_, size) => *size,
            Space::Free(size) => *size,
        }
    }
    fn id(&self) -> u64 {
        match self {
            Space::Block(id, _) => *id,
            _ => panic!("not a block"),
        }
    }

    fn defrag(&mut self) {
        match self {
            Space::Block(_, size) => {
                *size -= 1;
            }
            Self::Free(size) => {
                *size -= 1;
            }
        }
    }

    fn free(&mut self, size_to_free: u64) {
        match self {
            Self::Free(size) => *size -= size_to_free,
            _ => panic!("not a free space"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Space> {
    let mut id: i64 = -1;
    input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(i, c)| {
                    let is_block = i % 2 == 0;

                    if is_block {
                        id += 1;
                        Space::Block(id.try_into().unwrap(), c.to_digit(10).unwrap() as u64)
                    } else {
                        Space::Free(c.to_digit(10).unwrap() as u64)
                    }
                })
                .collect::<Vec<Space>>()
        })
        .flatten()
        .filter(|s| match s {
            Space::Block(_, _) => true,
            Space::Free(size) => *size > 0,
        })
        .collect::<Vec<Space>>()
}

fn find_last_block(disk: &Vec<Space>) -> Option<usize> {
    disk.iter().rposition(|s| match s {
        Space::Block(_, _) => true,
        _ => false,
    })
}

fn get_first_free_space(disk: &Vec<Space>) -> Option<usize> {
    disk.iter().position(|s| match s {
        Space::Free(_) => true,
        _ => false,
    })
}

fn move_block_to_free_space_when_possible(disk: &mut Vec<Space>) {
    let mut first_free_space_index = get_first_free_space(disk).unwrap();
    let mut last_block_index = find_last_block(disk).unwrap();

    while first_free_space_index < last_block_index {
        if disk[last_block_index].size() > 0 && disk[first_free_space_index].size() > 0 {
            disk[last_block_index].defrag();
            disk[first_free_space_index].defrag();

            disk.insert(
                first_free_space_index,
                Space::Block(disk[last_block_index].id(), 1),
            );

            if disk[first_free_space_index + 1].size() == 0 {
                disk.insert(last_block_index + 1, Space::Free(1));
            }

            if disk[last_block_index + 1].size() == 0 {
                disk.remove(last_block_index + 1);
            }
        }

        last_block_index = find_last_block(disk).unwrap();
        first_free_space_index = get_first_free_space(disk).unwrap();

        if disk[first_free_space_index].size() == 0 {
            disk.remove(first_free_space_index);
            first_free_space_index = get_first_free_space(disk).unwrap();
            last_block_index = find_last_block(disk).unwrap();
        }

        if disk[last_block_index].size() == 0 {
            disk.remove(last_block_index);
            first_free_space_index = get_first_free_space(disk).unwrap();
            last_block_index = find_last_block(disk).unwrap();
        }
    }
}

fn find_last_index_from_block_cursor(disk: &Vec<Space>, cursor: usize) -> Option<usize> {
    disk.iter().enumerate().rposition(|(i, s)| {
        i <= cursor
            && match s {
                Space::Block(_, _) => true,
                _ => false,
            }
    })
}

fn find_first_free_index_that_can_fit_block(
    disk: &Vec<Space>,
    last_block_index: usize,
) -> Option<usize> {
    disk.iter().enumerate().position(|(i, s)| {
        i < last_block_index
            && match s {
                Space::Free(size) => *size >= disk[last_block_index].size(),
                _ => false,
            }
    })
}

fn move_all_block_from_right_to_free_space_when_possible(disk: &mut Vec<Space>) {
    let mut last_block_cursor = disk.len() - 1;
    let mut last_block_index = find_last_index_from_block_cursor(disk, last_block_cursor);
    let mut first_free_index =
        find_first_free_index_that_can_fit_block(disk, last_block_index.unwrap());

    while last_block_cursor > 0 {
        if first_free_index.is_some() {
            if disk[first_free_index.unwrap()].size() == disk[last_block_index.unwrap()].size() {
                let save = disk[first_free_index.unwrap()];
                disk[first_free_index.unwrap()] = disk[last_block_index.unwrap()];
                disk[last_block_index.unwrap()] = save;

                last_block_cursor -= 1;
            } else if disk[first_free_index.unwrap()].size()
                > disk[last_block_index.unwrap()].size()
            {
                let block_size = disk[last_block_index.unwrap()].size();
                disk.insert(first_free_index.unwrap(), disk[last_block_index.unwrap()]);
                disk.remove(last_block_index.unwrap() + 1);
                disk[first_free_index.unwrap() + 1].free(block_size);
                disk.insert(last_block_index.unwrap() + 1, Space::Free(block_size));
            } else {
                trace!("this should not happen since we are looking for a free space that can fit the block");
            }
        } else {
            last_block_cursor -= 1;
        }

        last_block_index = find_last_index_from_block_cursor(disk, last_block_cursor);
        last_block_cursor = last_block_index.unwrap();
        first_free_index =
            find_first_free_index_that_can_fit_block(disk, last_block_index.unwrap());
    }
}

fn calulate_used_space(disk: &Vec<Space>) -> u64 {
    let mut res = 0;
    let mut all_ids = 0;

    disk.iter().filter(|s| s.is_block()).for_each(|s| {
        for _ in 0..s.size() {
            res += all_ids * s.id();
            all_ids += 1;
        }
    });

    res
}

fn calulate_used_space_with_free_space(disk: &Vec<Space>) -> u64 {
    let mut res = 0;
    let mut all_ids = 0;

    disk.iter().for_each(|s| match s {
        Space::Block(_, _) => {
            for _ in 0..s.size() {
                res += all_ids * s.id();
                all_ids += 1;
            }
        }
        Space::Free(_) => {
            for _ in 0..s.size() {
                all_ids += 1;
            }
        }
    });

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let disk = parse_input(input);
    let mut clone = disk.clone();

    move_block_to_free_space_when_possible(&mut clone);

    let used_space = calulate_used_space(&clone);

    Some(used_space)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = parse_input(input);

    move_all_block_from_right_to_free_space_when_possible(&mut disk);
    let used_space = calulate_used_space_with_free_space(&disk);

    info!("used space {:?}", used_space);

    Some(used_space)
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
        assert_eq!(result, Some(1928));
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
        assert_eq!(result, Some(2858));
    }
}
