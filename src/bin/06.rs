use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};
use tracing::info;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    // turn input into a grid
    input.lines().map(|line| line.chars().collect()).collect()
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
    // let mut visited_positions: Vec<(usize, usize)> = vec![];

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
        // visited_positions.push((col, row));

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

fn print_grid(grid: &Vec<Vec<char>>) {
    println!("Grid:");
    grid.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{}", c));
        println!();
    });
    println!("End of grid");
}

fn print_grid_tracing(grid: &Vec<Vec<char>>) {
    info!("Grid:");
    //print grind using tracing
    grid.iter().for_each(|row| {
        info!("{}", row.iter().collect::<String>());
    });
    info!("End of grid");
}

fn print_visited_grid(grid: &Vec<Vec<char>>, visited_positions: &Vec<(usize, usize)>) {
    let mut clone = grid.clone();
    visited_positions.iter().for_each(|(i, j)| {
        clone[*i][*j] = 'X';
    });

    print_grid_tracing(&clone);
}

pub fn part_two(input: &str) -> Option<u32> {
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

    let mut loop_count = 0;
    let mut visited_positions: Vec<(usize, usize)> = vec![];
    let mut visited_directions: Vec<Direction> = vec![];
    let mut visited_loops: Vec<Vec<(usize, usize)>> = vec![];

    grid[starting_position.unwrap().0][starting_position.unwrap().1] = '^';
    print_grid(&grid);
    let path = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| if *c == 'X' { Some((i, j)) } else { None })
        })
        .collect::<Vec<(usize, usize)>>();

    path.iter().for_each(|(i, j)| {
        let mut clone = ori_grid.clone();
        clone[*i][*j] = '#';

        visited_positions.clear();
        visited_directions.clear();

        let (mut start_col, mut start_row) = starting_position.unwrap().clone();

        info!("Path for ({}, {})", i, j);

        while start_col < clone.len() && start_row < clone[start_col].len() {
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
                info!("Break at: ({}, {})", start_col, start_row);

                break;
            }

            if visited_directions.len() == 5 {
                info!("No loop detected when visited all directions");
                print_visited_grid(&clone, &visited_positions.clone());
                break;
            }

            if visited_positions.contains(&(start_col, start_row)) && visited_directions.len() == 4
            {
                info!("position ({}, {}) already visited", start_col, start_row);
                print_visited_grid(&clone, &visited_positions.clone());

                visited_loops.push(visited_positions.clone());
                loop_count += 1;
                break;
            }

            let next = next.unwrap();

            match next {
                '.' | 'X' => {
                    visited_positions.push((start_col, start_row));

                    // continue in the same direction
                    match current {
                        '^' => {
                            clone[start_col][start_row] = '.';
                            start_col -= 1;
                            clone[start_col][start_row] = '^';

                            if visited_directions.last() != Some(&Direction::Up) {
                                visited_directions.push(Direction::Up);
                            }
                        }
                        '>' => {
                            clone[start_col][start_row] = '.';
                            start_row += 1;
                            clone[start_col][start_row] = '>';

                            if visited_directions.last() != Some(&Direction::Right) {
                                visited_directions.push(Direction::Right);
                            }
                        }
                        'v' => {
                            clone[start_col][start_row] = '.';
                            start_col += 1;
                            clone[start_col][start_row] = 'v';

                            if visited_directions.last() != Some(&Direction::Down) {
                                visited_directions.push(Direction::Down);
                            }
                        }
                        '<' => {
                            clone[start_col][start_row] = '.';
                            start_row -= 1;
                            clone[start_col][start_row] = '<';

                            if visited_directions.last() != Some(&Direction::Left) {
                                visited_directions.push(Direction::Left);
                            }
                        }
                        _ => {}
                    }
                }
                '#' => {
                    // change direction 90 degrees to the right
                    match current {
                        '^' => clone[start_col][start_row] = '>',
                        '>' => clone[start_col][start_row] = 'v',
                        'v' => clone[start_col][start_row] = '<',
                        '<' => clone[start_col][start_row] = '^',
                        _ => {}
                    }
                }
                _ => {
                    break;
                }
            }
        }
    });

    info!("printing visited loops");

    // visited_loops.iter().for_each(|loo| {
    //     print_visited_grid(&ori_grid, loo);
    // });

    println!("Loop count: {}", loop_count);

    Some(loop_count)
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
