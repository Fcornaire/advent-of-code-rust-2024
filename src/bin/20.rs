use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    sync::Mutex,
};

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use tracing::{debug, info};

advent_of_code::solution!(20);

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    visited_positions: HashSet<(usize, usize)>,
    elapsed_time: u64,
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
struct RaceTrack {
    map: Vec<Vec<char>>,
    walls: Vec<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
}

impl RaceTrack {
    fn all_fastest_path_with_cheat(&self, minimum_gain: u64) -> HashMap<u64, Vec<(usize, usize)>> {
        let original = self.fastest_path_without_cheating(None, None);

        if original.is_none() {
            return HashMap::new();
        }

        let (fastest_path_time, fastest_path) = original.unwrap();

        let all_fastest_path: Mutex<HashMap<u64, Vec<(usize, usize)>>> = Mutex::new(HashMap::new());

        let possible_cheat: Vec<(usize, usize)> = fastest_path
            .iter()
            .flat_map(|position| self.get_siblings_wall(*position))
            .unique()
            .collect();

        possible_cheat.par_iter().progress().for_each(|wall| {
            let mut map = self.map.clone();
            let new_walls: Vec<(usize, usize)> = self
                .walls
                .clone()
                .iter()
                .filter(|w| *w != wall)
                .cloned()
                .collect();

            map[wall.1][wall.0] = '.';
            let new_race_track = RaceTrack {
                map,
                walls: new_walls,
                start: self.start,
                end: self.end,
            };

            let possible_fastest = new_race_track
                .fastest_path_without_cheating(Some(minimum_gain), Some(fastest_path_time));

            if possible_fastest.is_none() {
                return;
            }

            let (new_fastest_path_time, _) = possible_fastest.unwrap();
            let gain = fastest_path_time - new_fastest_path_time;

            let mut all_fastest_path = all_fastest_path.lock().unwrap();

            if !all_fastest_path.contains_key(&gain) {
                all_fastest_path.insert(gain, vec![*wall]);
            } else {
                all_fastest_path.get_mut(&gain).unwrap().push(*wall);
            }
        });

        all_fastest_path.into_inner().unwrap()
    }

    fn fastest_path_without_cheating(
        &self,
        with_minimal_gain: Option<u64>,
        original_elapsed_time: Option<u64>,
    ) -> Option<(u64, Vec<(usize, usize)>)> {
        let mut heap = BinaryHeap::new();
        heap.push(State {
            cost: 0,
            position: self.start,
            visited_positions: HashSet::new(),
            elapsed_time: 0,
            path: vec![self.start],
        });

        let mut minimal_elapsed_time = u64::MAX;
        if let Some(minimal) = with_minimal_gain {
            if let Some(original) = original_elapsed_time {
                minimal_elapsed_time = original - minimal;
            }
        }

        let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();

        while let Some(State {
            cost,
            position,
            visited_positions: current_visited_positions,
            elapsed_time,
            path,
        }) = heap.pop()
        {
            if visited_positions.contains(&position) {
                continue;
            }
            visited_positions.insert(position);

            if elapsed_time > minimal_elapsed_time {
                continue;
            }

            let siblings = self.get_siblings(position);
            let valid_siblings: Vec<(usize, usize)> = siblings
                .iter()
                .filter(|sibling| !self.walls.contains(sibling))
                .cloned()
                .collect();

            for (row, col) in valid_siblings {
                if current_visited_positions.contains(&(row, col)) {
                    continue;
                }

                if (row, col) == self.end {
                    return Some((elapsed_time, path));
                }

                let mut new_path = path.clone();
                new_path.push((row, col));

                let mut new_visited_positions = current_visited_positions.clone();
                new_visited_positions.insert((row, col));

                heap.push(State {
                    cost: cost + 1,
                    position: (row, col),
                    visited_positions: new_visited_positions,
                    elapsed_time: elapsed_time + 1,
                    path: new_path,
                });
            }
        }

        None
    }

    fn get_siblings(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut siblings = vec![];

        let (row, col) = position;

        if row > 0 {
            siblings.push((row - 1, col));
        }

        if row < self.map.len() - 1 {
            siblings.push((row + 1, col));
        }

        if col > 0 {
            siblings.push((row, col - 1));
        }

        if col < self.map[0].len() - 1 {
            siblings.push((row, col + 1));
        }

        siblings
    }

    fn get_siblings_wall(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut siblings = vec![];

        let (row, col) = position;

        if row > 0 {
            if self.walls.contains(&(row - 1, col)) {
                siblings.push((row - 1, col));
            }
        }

        if row < self.map.len() - 1 {
            if self.walls.contains(&(row + 1, col)) {
                siblings.push((row + 1, col));
            }
        }

        if col > 0 {
            if self.walls.contains(&(row, col - 1)) {
                siblings.push((row, col - 1));
            }
        }

        if col < self.map[0].len() - 1 {
            if self.walls.contains(&(row, col + 1)) {
                siblings.push((row, col + 1));
            }
        }

        siblings
    }

    fn pretty_print(&self) {
        for row in &self.map {
            let row_str: String = row.iter().collect();
            info!("{}", row_str);
        }
    }
}

fn parse_input(input: &str) -> RaceTrack {
    let mut map = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => start = (x, y),
                'E' => end = (x, y),
                '#' => walls.push((x, y)),
                _ => {}
            }
            row.push(c);
        }
        map.push(row);
    }

    RaceTrack {
        map,
        start,
        end,
        walls,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let race_track = parse_input(input);

    // let fastest_path = race_track.fastest_path_without_cheating(None, None);
    // info!("Fastest path: {:#?}", fastest_path);

    let all_fastest_path = race_track.all_fastest_path_with_cheat(100);
    // info!("All fastest path: {:#?}", all_fastest_path);
    let res: usize = all_fastest_path.values().into_iter().map(|v| v.len()).sum();

    info!("Result: {:#?}", res);

    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
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
        assert_eq!(result, Some(1));
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
        assert_eq!(result, None);
    }
}
