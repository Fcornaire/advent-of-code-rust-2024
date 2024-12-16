use std::collections::{HashMap, HashSet};

use indicatif::ParallelProgressIterator;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    result, vec,
};
use tracing::{debug, info, trace};

advent_of_code::solution!(16);

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
        (Direction::East, (position.0 - 1, position.1)),  //left
        (Direction::West, (position.0 + 1, position.1)),  //right
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
) -> Option<(HashSet<(usize, usize)>, usize)> {
    let mut stack = vec![(
        start_position,
        start_direction,
        start_visited_positions,
        start_score,
    )];

    let mut best_result: Option<(HashSet<(usize, usize)>, usize)> = None;

    while let Some((position, current_direction, visited_positions, score)) = stack.pop() {
        let valid_siblings = get_valid_siblings(position, &grid);

        let non_visited_siblings = valid_siblings
            .iter()
            .filter(|(_, pos)| !visited_positions.contains(pos))
            .collect::<HashMap<_, _>>();

        if non_visited_siblings.is_empty() {
            continue;
        }

        // Check if we are at the end
        if let Some((dir, pos)) = non_visited_siblings
            .iter()
            .find(|(_, (x, y))| grid[*y][*x] == 'E')
        {
            let new_score = score + current_direction.get_score_from(**dir);
            let mut new_visited_positions = visited_positions.clone();
            new_visited_positions.insert(**pos);

            let result = Some((new_visited_positions, new_score));
            if best_result.is_none() || result.as_ref().unwrap().1 < best_result.as_ref().unwrap().1
            {
                best_result = result;
            }
            continue;
        }

        let new_states: Vec<_> = non_visited_siblings
            .par_iter()
            .map(|(dir, pos)| {
                let new_score = score + current_direction.get_score_from(**dir);
                let mut new_visited_positions = visited_positions.clone();
                new_visited_positions.insert(**pos);

                (**pos, **dir, new_visited_positions, new_score)
            })
            .collect();

        stack.extend(new_states);
    }

    best_result
}

fn find_shortest_path_score(start_position: (usize, usize), grid: &Vec<Vec<char>>) -> Option<u32> {
    let search = search_for_end(start_position, Direction::East, grid, HashSet::new(), 0);

    if search.is_some() {
        let (visited_positions, score) = search.unwrap();
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

    info!("Grid:");
    pretty_print(grid.clone());

    let res = find_shortest_path_score(start_position.unwrap(), &grid);

    res
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

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
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
