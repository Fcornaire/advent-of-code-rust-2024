use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing::{debug, info};

advent_of_code::solution!(18);

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    visited_positions: HashSet<(usize, usize)>,
    path: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Memory {
    pub corrupted: HashSet<(usize, usize)>,
    pub width: usize,
    pub height: usize,
}

impl Memory {
    pub fn pretty_print(&self) {
        for col in 0..self.height {
            let mut row_str = String::new();
            for row in 0..self.width {
                if self.corrupted.contains(&(row, col)) {
                    row_str.push('#');
                } else {
                    row_str.push('.');
                }
            }
            info!("{}", row_str);
        }
    }

    pub fn pretty_print_with_visited(&self, visited_positions: &HashSet<(usize, usize)>) {
        for col in 0..self.height {
            let mut row_str = String::new();
            for row in 0..self.width {
                if self.corrupted.contains(&(row, col)) {
                    row_str.push('#');
                } else if visited_positions.contains(&(row, col)) {
                    row_str.push('X');
                } else {
                    row_str.push('.');
                }
            }
            info!("{}", row_str);
        }
    }

    fn get_siblings(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut siblings = vec![];

        let (row, col) = position;

        if row > 0 {
            siblings.push((row - 1, col));
        }

        if row < self.width - 1 {
            siblings.push((row + 1, col));
        }

        if col > 0 {
            siblings.push((row, col - 1));
        }

        if col < self.height - 1 {
            siblings.push((row, col + 1));
        }

        siblings
    }

    pub fn search_for_end(
        &self,
        start: (usize, usize),
    ) -> Option<(HashSet<(usize, usize)>, Vec<(usize, usize)>)> {
        let mut heap = BinaryHeap::new();
        heap.push(State {
            cost: 0,
            position: start,
            visited_positions: HashSet::new(),
            path: vec![start],
        });

        let mut best_result: Option<(HashSet<(usize, usize)>, Vec<(usize, usize)>)> = None;
        let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();

        while let Some(State {
            cost,
            position,
            visited_positions: current_visited_positions,
            path,
        }) = heap.pop()
        {
            if visited_positions.contains(&position) {
                continue;
            }
            visited_positions.insert(position);

            let siblings = self.get_siblings(position);
            let valid_siblings: Vec<(usize, usize)> = siblings
                .iter()
                .filter(|sibling| !self.corrupted.contains(sibling))
                .cloned()
                .collect();

            for (row, col) in valid_siblings {
                if current_visited_positions.contains(&(row, col)) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push((row, col));

                if (row, col) == (self.width - 1, self.height - 1) {
                    let mut new_visited_positions = current_visited_positions.clone();
                    new_visited_positions.insert((row, col));

                    let result = Some((new_visited_positions, new_path.clone()));
                    match best_result {
                        Some((_, ref best_path)) => {
                            if new_path.len() < best_path.len() {
                                best_result = result;
                            }
                        }
                        None => {
                            best_result = result;
                        }
                    }

                    return best_result;
                }

                let mut new_visited_positions = current_visited_positions.clone();
                new_visited_positions.insert((row, col));

                heap.push(State {
                    cost: cost + 1,
                    position: (row, col),
                    visited_positions: new_visited_positions,
                    path: new_path,
                });
            }
        }

        best_result
    }
}

fn parse_input(input: &str, width: usize, height: usize, line_to_take: usize) -> (Memory, String) {
    let lines: Vec<&str> = input.lines().take(line_to_take).collect();
    let last_line_taken = input.lines().nth(line_to_take - 1).unwrap();

    let corrupted: HashSet<(usize, usize)> = lines
        .iter()
        .map(|line| {
            let re = regex::Regex::new(r"(\d+),(\d+)").unwrap();
            re.captures_iter(&line)
                .map(|cap| {
                    let row = cap[1].parse::<usize>().unwrap();
                    let col = cap[2].parse::<usize>().unwrap();

                    (row, col)
                })
                .collect::<HashSet<(usize, usize)>>()
        })
        .flatten()
        .collect();

    (
        Memory {
            corrupted,
            width,
            height,
        },
        last_line_taken.to_string(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (memory, _) = parse_input(input, 7, 7, 12); //for real input,     let memory = parse_input(input, 71, 71, 1024);

    // info!("Memory: {:?}", memory);

    let result = memory.search_for_end((0, 0));

    if result.is_some() {
        let (_, path) = result.unwrap();

        return Some((path.len() - 1) as u32);
    }

    None
}

pub fn part_two(input: &str) -> Option<String> {
    let total_lines = input.lines().count();

    let res = (12..total_lines) //1024 for real input
        .into_par_iter()
        .progress()
        .map(|line_index| {
            let (memory, line) = parse_input(input, 7, 7, line_index); //for real input, 71, 71

            (memory.search_for_end((0, 0)), line, line_index)
        })
        .filter(|(result, _, _)| result.is_none())
        .min_by_key(|(_, _, line_index)| *line_index);

    debug!("Result: {:?}", res);

    if res.is_some() {
        let (_, line, _) = res.unwrap();
        return Some(line);
    }

    None
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
        assert_eq!(result, Some(22)); //260 for real input
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
        assert_eq!(result, Some("6,1".to_string())); //24,48
    }
}
