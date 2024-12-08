use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::info;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_all_antennas(grid: &Vec<Vec<char>>) -> Vec<(char, (i32, i32))> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c != '.' {
                    Some((c, (x as i32, y as i32)))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn get_all_siblings(
    all_antenna: &Vec<(char, (i32, i32))>,
    c: char,
    x: i32,
    y: i32,
) -> Vec<(char, (i32, i32))> {
    all_antenna
        .par_iter()
        .filter(|(c2, (x2, y2))| {
            if c == *c2 {
                if x == *x2 && y == *y2 {
                    return false;
                }
                return true;
            }

            false
        })
        .cloned()
        .collect()
}

fn get_all_possible_antinode_for_sibling(
    all_sibling: &Vec<(char, (i32, i32))>,
    x: i32,
    y: i32,
) -> Vec<(i32, i32)> {
    all_sibling
        .par_iter()
        .map(|(_, (x2, y2))| ((x - *x2), (y - y2)))
        .map(|(dx, dy)| {
            let x = x + dx;
            let y = y + dy;

            (x, y)
        })
        .collect()
}

fn get_all_possible_antinode_in_grind_for_sibling(
    grid: &Vec<Vec<char>>,
    all_sibling: &Vec<(char, (i32, i32))>,
    x: i32,
    y: i32,
) -> Vec<Vec<(i32, i32)>> {
    all_sibling
        .par_iter()
        .map(|(_, (x2, y2))| ((x - *x2), (y - y2)))
        .map(|(dx, dy)| {
            let mut x_res = x + dx;
            let mut y_res = y + dy;
            let mut res = Vec::new();

            while is_valid_antinode_position(&grid, x_res, y_res) {
                if grid[y_res as usize][x_res as usize] == '.' {
                    res.push((x_res, y_res));
                }
                x_res = x_res + dx;
                y_res = y_res + dy;
            }

            res
        })
        .collect()
}

fn is_valid_antinode_position(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    if y >= grid.len() as i32 {
        return false;
    }

    if x >= grid[y as usize].len() as i32 {
        return false;
    }

    if grid[y as usize][x as usize] != '.' {
        info!(
            "warning: there is something ( {} ) at ({},{}) is not valid",
            grid[y as usize][x as usize], x, y
        );
        return true;
    }

    true
}

pub fn part_one(input: &str) -> Option<i32> {
    let grid = parse_input(input);
    let mut clone = grid.clone();
    let antennas: Vec<(char, (i32, i32))> = get_all_antennas(&grid);

    antennas.iter().for_each(|(antenna, (x, y))| {
        let siblings = get_all_siblings(&antennas, *antenna, *x, *y);
        let all_possible_antinode = get_all_possible_antinode_for_sibling(&siblings, *x, *y);

        let valid_antinode = all_possible_antinode
            .par_iter()
            .filter(|(x, y)| is_valid_antinode_position(&clone, *x, *y))
            .collect::<Vec<_>>();
        info!("valid antinode: {:?}", valid_antinode);

        valid_antinode.iter().for_each(|(x, y)| {
            clone[*y as usize][*x as usize] = '#';
        });
    });

    let res = clone
        .par_iter()
        .map(|row| row.par_iter().filter(|&&c| c == '#').count() as i32)
        .sum();

    clone.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{}", c));
        println!();
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut clone = grid.clone();
    let antennas: Vec<(char, (i32, i32))> = get_all_antennas(&grid);

    antennas.iter().for_each(|(antenna, (x, y))| {
        let siblings = get_all_siblings(&antennas, *antenna, *x, *y);

        let all_possible_antinode_in_grind =
            get_all_possible_antinode_in_grind_for_sibling(&clone, &siblings, *x, *y);

        all_possible_antinode_in_grind.iter().for_each(|antinode| {
            antinode.iter().for_each(|(x, y)| {
                clone[*y as usize][*x as usize] = '#';
            });
        });
    });

    let res: u32 = clone
        .par_iter()
        .map(|row| row.par_iter().filter(|&&c| c != '.').count() as u32)
        .sum();

    clone.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{}", c));
        println!();
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
        assert_eq!(result, Some(14));
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
        assert_eq!(result, Some(34));
    }
}
