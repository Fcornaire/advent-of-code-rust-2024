use std::{collections::HashSet, sync::Mutex};

use indicatif::{ ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::info;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn pretty_print(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);
    let starting_position: Option<(usize, usize)> = grid.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(|(j, c)| {
            if *c == '^' || *c == '>' || *c == 'v' || *c == '<' {
                Some((i, j))
            } else {
                None
            }
        })
    });

    info!("Starting position: {:?}", starting_position);
    let (mut col, mut row) = starting_position.unwrap();

    while col < grid.len() && row < grid[col].len() {
        let current = grid[col][row];
        let next = match current {
            '^' => {
                if col > 0 {
                    Some(grid[col - 1][row])
                } else {
                    None
                }
            }
            '>' => {
                if row + 1 < grid[col].len() {
                    Some(grid[col][row + 1])
                } else {
                    None
                }
            }
            'v' => {
                if col + 1 < grid.len() {
                    Some(grid[col + 1][row])
                } else {
                    None
                }
            }
            '<' => {
                if row > 0 {
                    Some(grid[col][row - 1])
                } else {
                    None
                }
            }

            _ => None,
        };

        if next.is_none() {
            info!("Break at: ({}, {})", col, row);
            grid[col][row] = 'X';
            break;
        }

        let next = next.unwrap();

        match next {
            '.' | 'X' => {
                // continue in the same direction
                match current {
                    '^' => {
                        grid[col][row] = 'X';
                        col -= 1;
                        grid[col][row] = '^';
                    }
                    '>' => {
                        grid[col][row] = 'X';
                        row += 1;
                        grid[col][row] = '>';
                    }
                    'v' => {
                        grid[col][row] = 'X';
                        col += 1;
                        grid[col][row] = 'v';
                    }
                    '<' => {
                        grid[col][row] = 'X';
                        row -= 1;
                        grid[col][row] = '<';
                    }
                    _ => {}
                }
            }
            '#' => {
                // change direction 90 degrees to the right
                match current {
                    '^' => grid[col][row] = '>',
                    '>' => grid[col][row] = 'v',
                    'v' => grid[col][row] = '<',
                    '<' => grid[col][row] = '^',
                    _ => {}
                }
            }
            _ => {
                break;
            }
        }
    }

    let visited = grid.iter().flatten().filter(|&c| *c == 'X').count();

    Some(visited as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let ori_grid = grid.clone();
    let starting_position: Option<(usize, usize)> = grid.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(|(j, c)| {
            if *c == '^' || *c == '>' || *c == 'v' || *c == '<' {
                Some((i, j))
            } else {
                None
            }
        })
    });

    info!("Starting position: {:?}", starting_position);
    let (mut col, mut row) = starting_position.unwrap().clone();
    let mut path: Vec<(usize, usize)> = vec![];

    while col < grid.len() && row < grid[col].len() {
        let current = grid[col][row];
        let next = match current {
            '^' => {
                if col > 0 {
                    Some(grid[col - 1][row])
                } else {
                    None
                }
            }
            '>' => {
                if row + 1 < grid[col].len() {
                    Some(grid[col][row + 1])
                } else {
                    None
                }
            }
            'v' => {
                if col + 1 < grid.len() {
                    Some(grid[col + 1][row])
                } else {
                    None
                }
            }
            '<' => {
                if row > 0 {
                    Some(grid[col][row - 1])
                } else {
                    None
                }
            }

            _ => None,
        };

        if next.is_none() {
            grid[col][row] = 'X';
            path.push((col, row));
            break;
        }

        let next = next.unwrap();

        match next {
            '.' | 'X' => {
                path.push((col, row));
                // continue in the same direction
                match current {
                    '^' => {
                        grid[col][row] = 'X';
                        col -= 1;
                        grid[col][row] = '^';
                    }
                    '>' => {
                        grid[col][row] = 'X';
                        row += 1;
                        grid[col][row] = '>';
                    }
                    'v' => {
                        grid[col][row] = 'X';
                        col += 1;
                        grid[col][row] = 'v';
                    }
                    '<' => {
                        grid[col][row] = 'X';
                        row -= 1;
                        grid[col][row] = '<';
                    }
                    _ => {}
                }
            }
            '#' => {
                // change direction 90 degrees to the right
                match current {
                    '^' => grid[col][row] = '>',
                    '>' => grid[col][row] = 'v',
                    'v' => grid[col][row] = '<',
                    '<' => grid[col][row] = '^',
                    _ => {}
                }
            }
            _ => {
                break;
            }
        }
    }

    let loops: Mutex<HashSet<(usize, usize)>> = Mutex::new(HashSet::new());

    grid[starting_position.unwrap().0][starting_position.unwrap().1] = '^';

    path.remove(0);

    let progress_bar = ProgressBar::new(path.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/red}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    info!("Total of path to process: {}", path.len());

    path.par_iter().for_each(|(i, j)| {
        let mut visited_positions_directions: HashSet<(usize, usize, Direction)> = HashSet::new();
        let mut visiteds_directions: Vec<Direction> = Vec::with_capacity(6);

        visiteds_directions.clear();

        let mut clone = ori_grid.clone();
        clone[*i][*j] = '🟨';

        let (mut start_col, mut start_row) = starting_position.unwrap().clone();
        let mut direction = match clone[start_col][start_row] {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => Direction::Up,
        };

        while start_col < clone.len() && start_row < clone[start_col].len() {
            if visiteds_directions.len() > 5 {
                visiteds_directions.clear();
            }

            let current = clone[start_col][start_row];
            let next = match current {
                '^' => {
                    if start_col > 0 {
                        Some(clone[start_col - 1][start_row])
                    } else {
                        None
                    }
                }
                '>' => {
                    if start_row + 1 < clone[start_col].len() {
                        Some(clone[start_col][start_row + 1])
                    } else {
                        None
                    }
                }
                'v' => {
                    if start_col + 1 < clone.len() {
                        Some(clone[start_col + 1][start_row])
                    } else {
                        None
                    }
                }
                '<' => {
                    if start_row > 0 {
                        Some(clone[start_col][start_row - 1])
                    } else {
                        None
                    }
                }

                _ => None,
            };

            if next.is_none() {
                break;
            }

            if visited_positions_directions.contains(&(start_col, start_row, direction.clone()))
                && visiteds_directions.len() >= 4
            {
                visited_positions_directions.clear();
                visiteds_directions.clear();
                loops.lock().unwrap().insert((*i, *j));
                break;
            }

            let next = next.unwrap();

            match next {
                '#' | '🟨' => {
                    // change direction 90 degrees
                    match current {
                        '^' => {
                            visiteds_directions.push(Direction::Right);

                            clone[start_col][start_row] = '>'
                        }
                        '>' => {
                            visiteds_directions.push(Direction::Down);

                            clone[start_col][start_row] = 'v'
                        }
                        'v' => {
                            visiteds_directions.push(Direction::Left);

                            clone[start_col][start_row] = '<'
                        }
                        '<' => {
                            visiteds_directions.push(Direction::Up);

                            clone[start_col][start_row] = '^'
                        }
                        _ => {}
                    }
                }
                _ => {
                    // same direction
                    visited_positions_directions.insert((start_col, start_row, direction.clone()));
                    match current {
                        '^' => {
                            start_col -= 1;
                            clone[start_col][start_row] = '^';
                            direction = Direction::Up;
                        }
                        '>' => {
                            start_row += 1;
                            clone[start_col][start_row] = '>';
                            direction = Direction::Right;
                        }
                        'v' => {
                            start_col += 1;
                            clone[start_col][start_row] = 'v';
                            direction = Direction::Down;
                        }
                        '<' => {
                            start_row -= 1;
                            clone[start_col][start_row] = '<';
                            direction = Direction::Left;
                        }
                        _ => {}
                    }
                }
            }
        }

        progress_bar.inc(1);
    });

    progress_bar.finish_with_message("Finished processin loops");

    let count = loops.lock().unwrap().len();
    println!("Loop count: {}", count);

    Some(count as u64)
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
        assert_eq!(result, Some(41));
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
        assert_eq!(result, Some(6));
    }
}
