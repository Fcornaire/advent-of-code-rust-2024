use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    result, vec,
};
use tracing::{debug, info, trace};

advent_of_code::solution!(16);

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Direction,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn get_score_from(&self, from: Direction) -> usize {
        match (self, from) {
            (Direction::North, Direction::North) => 1,
            (Direction::South, Direction::South) => 1,
            (Direction::East, Direction::East) => 1,
            (Direction::West, Direction::West) => 1,
            (Direction::North, Direction::East)
            | (Direction::East, Direction::South)
            | (Direction::South, Direction::West)
            | (Direction::West, Direction::North) => 1001,
            (Direction::North, Direction::West)
            | (Direction::West, Direction::South)
            | (Direction::South, Direction::East)
            | (Direction::East, Direction::North) => 1001,
            (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::East, Direction::West)
            | (Direction::West, Direction::East) => 2001,
        }
    }
}

fn parse_input(input: &str) -> (Option<(usize, usize)>, Vec<Vec<char>>) {
    let mut start_position = None;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == 'S' {
                start_position = Some((x as usize, y as usize));
            }
        });
    });

    let grid = input.lines().map(|line| line.chars().collect()).collect();

    (start_position, grid)
}

fn get_valid_siblings(
    position: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> HashMap<Direction, (usize, usize)> {
    vec![
        (Direction::North, (position.0, position.1 - 1)), //up
        (Direction::South, (position.0, position.1 + 1)), //down
        (Direction::East, (position.0 + 1, position.1)),  //left
        (Direction::West, (position.0 - 1, position.1)),  //right
    ]
    .iter()
    .filter(|(_, (x, y))| grid[*y][*x] != '#')
    .map(|(dir, pos)| (*dir, *pos))
    .collect()
}

fn search_for_end(
    start_position: (usize, usize),
    start_direction: Direction,
    grid: &Vec<Vec<char>>,
    start_visited_positions: HashSet<(usize, usize)>,
    start_score: usize,
) -> Option<(HashSet<(usize, usize)>, usize, Vec<(usize, usize)>)> {
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: start_score,
        position: start_position,
        direction: start_direction,
        visited_positions: start_visited_positions,
        path: vec![start_position],
    });

    let mut best_result: Option<(HashSet<(usize, usize)>, usize, Vec<(usize, usize)>)> = None;
    let mut visited_positions = HashSet::new();

    while let Some(State {
        cost,
        position,
        direction,
        visited_positions: current_visited_positions,
        path,
    }) = heap.pop()
    {
        if visited_positions.contains(&position) {
            continue;
        }
        visited_positions.insert(position);

        let valid_siblings = get_valid_siblings(position, &grid);

        for (dir, pos) in valid_siblings {
            if !current_visited_positions.contains(&pos) {
                let mut new_path = path.clone();
                new_path.push(pos);

                if grid[pos.1][pos.0] == 'E' {
                    let new_score = cost + direction.get_score_from(dir);
                    let mut new_visited_positions = current_visited_positions.clone();
                    new_visited_positions.insert(pos);

                    let result = Some((new_visited_positions, new_score, new_path));
                    if best_result.is_none()
                        || result.as_ref().unwrap().1 < best_result.as_ref().unwrap().1
                    {
                        best_result = result;
                    }
                    return best_result; // Early exit
                }

                let new_score = cost + direction.get_score_from(dir);
                let mut new_visited_positions = current_visited_positions.clone();
                new_visited_positions.insert(pos);

                heap.push(State {
                    cost: new_score,
                    position: pos,
                    direction: dir,
                    visited_positions: new_visited_positions,
                    path: new_path,
                });
            }
        }
    }

    best_result
}

fn calculate_score_for_custom_path(
    start_position: (usize, usize),
    current_end_position: (usize, usize),
    new_end_position: (usize, usize),
    grid: Vec<Vec<char>>,
) -> (HashSet<(usize, usize)>, usize) {
    let mut score = 0;
    let mut visited_positions = HashSet::new();
    visited_positions.insert(start_position);

    //remove the end position from the grid and replace it with  a new end position
    let mut new_grid = grid.clone();
    new_grid[current_end_position.1][current_end_position.0] = '.';
    new_grid[new_end_position.1][new_end_position.0] = 'E';

    let search = search_for_end(
        start_position,
        Direction::East,
        &new_grid,
        HashSet::new(),
        0,
    );

    if search.is_some() {
        let (visited, new_score, path) = search.unwrap();
        score = new_score;
        visited_positions = visited;
    }

    (visited_positions, score)
}

fn get_end_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut end_position = (0, 0);

    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            if *c == 'E' {
                end_position = (x, y);
            }
        });
    });

    end_position
}

fn find_alternative_shortest_paths(
    start_position: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> Vec<(HashSet<(usize, usize)>, usize)> {
    let mut visited = HashSet::new();
    visited.insert(start_position);
    let initial_path = search_for_end(start_position, Direction::East, grid, visited, 0);
    let mut all_paths = Vec::new();

    if let Some((initial_visited_positions, initial_score, path)) = initial_path {
        let initial_visited_positions_as_vec: Vec<(usize, usize)> = initial_visited_positions
            .clone()
            .into_iter()
            .sorted()
            .collect();

        all_paths.push((initial_visited_positions.clone(), initial_score));

        for initial_position in &initial_visited_positions {
            let valid_siblings_for_pos: HashMap<Direction, (usize, usize)> =
                get_valid_siblings(*initial_position, &grid)
                    .iter()
                    .filter(|(_, pos)| !initial_visited_positions.contains(pos))
                    .map(|(dir, pos)| (*dir, *pos))
                    .collect();

            valid_siblings_for_pos.iter().for_each(|(dir, pos)| {
                let (visited_positions, custom_score) = calculate_score_for_custom_path(
                    start_position,
                    get_end_position(grid),
                    *pos,
                    grid.clone(),
                );

                let search = search_for_end(*pos, *dir, grid, visited_positions, custom_score);

                if search.is_some() {
                    let (visited_positions, score, path) = search.unwrap();

                    if score == initial_score {
                        all_paths.push((visited_positions, score));
                    }
                }
            });
        }
    }

    all_paths
}

fn find_shortest_path_score(start_position: (usize, usize), grid: &Vec<Vec<char>>) -> Option<u32> {
    let mut visited = HashSet::new();
    visited.insert(start_position);
    let search = search_for_end(start_position, Direction::East, grid, visited, 0);

    if search.is_some() {
        let (visited_positions, score, path) = search.unwrap();
        // info!("Visited positions: {:?}", visited_positions);
        info!("Score: {}", score);

        // let mut new_grid = grid.clone();
        // visited_positions.iter().for_each(|(x, y)| {
        //     new_grid[*y][*x] = 'X';
        // });
        // pretty_print(new_grid);

        return Some(score as u32);
    }

    None
}

fn pretty_print(grid: Vec<Vec<char>>) {
    let mut res = String::new();
    res.push('\n');

    grid.iter().for_each(|line| {
        line.iter().for_each(|c| {
            res.push(*c);
        });
        res.push('\n');
    });
    info!("{}", res);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start_position, grid) = parse_input(input);
    if start_position.is_none() {
        return None;
    }

    info!("Start position: {:?}", start_position.unwrap());

    let res = find_shortest_path_score(start_position.unwrap(), &grid);

    res
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start_position, grid) = parse_input(input);
    if start_position.is_none() {
        return None;
    }

    info!("Start position: {:?}", start_position.unwrap());

    let res: Vec<(HashSet<(usize, usize)>, usize)> =
        find_alternative_shortest_paths(start_position.unwrap(), &grid);

    let as_vec: HashSet<(usize, usize)> = res.iter().flat_map(|(v, _)| v).copied().collect();

    Some(as_vec.len() as u32)
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

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(3022));
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

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }
}
