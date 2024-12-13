use std::collections::HashSet;

use rayon::prelude::*;

advent_of_code::solution!(12);

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Corner {
    UpLeft(usize),
    UpRight(usize),
    DownLeft(usize),
    DownRight(usize),
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Cell {
    pub position: (usize, usize),
    pub garden_plot: char,
    pub corners: HashSet<Corner>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Region {
    pub garden_plot: char,
    pub position: Vec<(usize, usize)>,
}

impl Region {
    fn get_all_corners(&self, garden: &Vec<Vec<Cell>>) -> usize {
        let mut corners = 0;

        for position in &self.position {
            let cell = &garden[position.0][position.1];

            corners += cell.corners.len();
        }

        corners
    }

    pub fn get_cost_with_corners(&self, garden: &Vec<Vec<Cell>>) -> usize {
        self.position.len() * self.get_all_corners(garden)
    }

    pub fn get_perimeter(&self) -> usize {
        let positions_set: HashSet<(usize, usize)> = self.position.iter().cloned().collect();

        self.position
            .iter()
            .map(|&(x, y)| {
                let mut count = 0;

                if x == 0 || !positions_set.contains(&(x - 1, y)) {
                    count += 1;
                }

                if !positions_set.contains(&(x + 1, y)) {
                    count += 1;
                }

                if y == 0 || !positions_set.contains(&(x, y - 1)) {
                    count += 1;
                }

                if !positions_set.contains(&(x, y + 1)) {
                    count += 1;
                }
                count
            })
            .sum()
    }

    pub fn get_cost(&self) -> usize {
        self.position.len() * self.get_perimeter()
    }
}

fn get_siblings(
    garden: &Vec<Vec<char>>,
    position: (usize, usize),
    visited_positions: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut siblings = vec![];
    let (x, y) = position;
    let current = garden[x][y];

    visited_positions.insert((x, y));

    if x > 0 && garden[x - 1][y] == current && !visited_positions.contains(&(x - 1, y)) {
        siblings.push((x - 1, y));
    }

    if y > 0 && garden[x][y - 1] == current && !visited_positions.contains(&(x, y - 1)) {
        siblings.push((x, y - 1));
    }

    if x < garden.len() - 1
        && garden[x + 1][y] == current
        && !visited_positions.contains(&(x + 1, y))
    {
        siblings.push((x + 1, y));
    }

    if y < garden[0].len() - 1
        && garden[x][y + 1] == current
        && !visited_positions.contains(&(x, y + 1))
    {
        siblings.push((x, y + 1));
    }

    if siblings.len() == 0 {
        return siblings;
    }

    let new_siblings: Vec<(usize, usize)> = siblings
        .iter()
        .map(|sibling_position| get_siblings(garden, *sibling_position, visited_positions))
        .flatten()
        .collect();

    siblings.extend(new_siblings);

    siblings
}

fn get_all_regions(garden: &Vec<Vec<char>>) -> HashSet<Region> {
    let mut regions = HashSet::new();

    let positions: Vec<(usize, usize)> = (0..garden.len())
        .flat_map(|x| (0..garden[0].len()).map(move |y| (x, y)))
        .collect();

    let new_regions: Vec<Region> = positions
        .par_iter()
        .map(|&(x, y)| {
            let mut siblings = get_siblings(garden, (x, y), &mut HashSet::new());

            if !siblings.contains(&(x, y)) {
                siblings.push((x, y));
            }

            siblings.sort();
            siblings.dedup();

            Region {
                garden_plot: garden[x][y],
                position: siblings,
            }
        })
        .collect();

    for region in new_regions {
        regions.insert(region);
    }

    regions
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn parse_input_part_2(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, garden_plot)| {
                    let corners: HashSet<Corner> = vec![
                        Corner::UpLeft(1),
                        Corner::UpRight(1),
                        Corner::DownLeft(1),
                        Corner::DownRight(1),
                    ]
                    .into_iter()
                    .collect();

                    return Cell {
                        position: (x, y),
                        garden_plot,
                        corners,
                    };
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let garden = parse_input(input);
    let regions = get_all_regions(&garden);

    // info!("Regions: {:?}", regions);

    let cost: usize = regions.iter().map(|region| region.get_cost()).sum();

    Some(cost as u64)
}

fn get_neighbours(garden: &Vec<Vec<Cell>>, cell: Cell) -> Vec<&Cell> {
    let mut neighbours = vec![];
    let (x, y) = cell.position;

    if x > 0 {
        neighbours.push(&garden[x - 1][y]);
    }

    if y > 0 {
        neighbours.push(&garden[x][y - 1]);
    }

    if x < garden.len() - 1 {
        neighbours.push(&garden[x + 1][y]);
    }

    if y < garden[0].len() - 1 {
        neighbours.push(&garden[x][y + 1]);
    }

    neighbours
}

fn rezize_all_corner(garden: &mut Vec<Vec<Cell>>) {
    let cloned_garden = garden.clone();
    let all_neighbours: Vec<Vec<Vec<&Cell>>> = cloned_garden
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| get_neighbours(&cloned_garden, cell.clone()))
                .collect()
        })
        .collect();

    garden.iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, cell)| {
            let neighbours = &all_neighbours[i][j];

            neighbours.iter().for_each(|neighbour| {
                let is_same_plot = neighbour.garden_plot == cell.garden_plot;

                //check if if neighbour is up
                if cell.position.0 > neighbour.position.0 {
                    let left = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.1 < cell.position.1);

                    let right = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.1 > cell.position.1);

                    if is_same_plot {
                        if cell.corners.contains(&Corner::UpLeft(1)) {
                            cell.corners.remove(&Corner::UpLeft(1));
                        }

                        if cell.corners.contains(&Corner::UpRight(1)) {
                            cell.corners.remove(&Corner::UpRight(1));
                        }
                    } else {
                        if left.is_some() && left.unwrap().garden_plot != cell.garden_plot {
                            if !cell.corners.contains(&Corner::UpLeft(1)) {
                                cell.corners.insert(Corner::UpLeft(1));
                            }
                        }

                        if right.is_some() && right.unwrap().garden_plot != cell.garden_plot {
                            if !cell.corners.contains(&Corner::UpRight(1)) {
                                cell.corners.insert(Corner::UpRight(1));
                            }
                        }
                    }
                }

                //check if if neighbour is down
                if cell.position.0 < neighbour.position.0 {
                    let left = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.1 < cell.position.1);

                    let right = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.1 > cell.position.1);

                    if is_same_plot {
                        if cell.corners.contains(&Corner::DownLeft(1)) {
                            cell.corners.remove(&Corner::DownLeft(1));
                        }

                        if cell.corners.contains(&Corner::DownRight(1)) {
                            cell.corners.remove(&Corner::DownRight(1));
                        }
                    } else {
                        if left.is_some() && left.unwrap().garden_plot != cell.garden_plot {
                            if !cell.corners.contains(&Corner::DownLeft(1)) {
                                cell.corners.insert(Corner::DownLeft(1));
                            }
                        }

                        if right.is_some() && right.unwrap().garden_plot != cell.garden_plot {
                            if !cell.corners.contains(&Corner::DownRight(1)) {
                                cell.corners.insert(Corner::DownRight(1));
                            }
                        }
                    }
                }

                //check if if neighbour is left
                if cell.position.1 > neighbour.position.1 {
                    let up = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.0 < cell.position.0);

                    let down = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.0 > cell.position.0);

                    if is_same_plot {
                        if cell.corners.contains(&Corner::UpLeft(1)) {
                            cell.corners.remove(&Corner::UpLeft(1));
                        }

                        if cell.corners.contains(&Corner::DownLeft(1)) {
                            cell.corners.remove(&Corner::DownLeft(1));
                        }
                    } else {
                        if up.is_some() && up.unwrap().garden_plot != cell.garden_plot {
                            if !cell.corners.contains(&Corner::UpLeft(1)) {
                                cell.corners.insert(Corner::UpLeft(1));
                            }
                        }

                        if down.is_some() && down.unwrap().garden_plot != cell.garden_plot {
                            if !cell.corners.contains(&Corner::DownLeft(1)) {
                                cell.corners.insert(Corner::DownLeft(1));
                            }
                        }
                    }
                }

                //check if if neighbour is right
                if cell.position.1 < neighbour.position.1 {
                    let up = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.0 < cell.position.0);

                    let down = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.0 > cell.position.0);

                    if is_same_plot {
                        if cell.corners.contains(&Corner::UpRight(1)) {
                            cell.corners.remove(&Corner::UpRight(1));
                        }

                        if cell.corners.contains(&Corner::DownRight(1)) {
                            cell.corners.remove(&Corner::DownRight(1));
                        }
                    } else {
                        if up.is_some() && up.unwrap().garden_plot != cell.garden_plot {
                            if !cell.corners.contains(&Corner::UpRight(1)) {
                                cell.corners.insert(Corner::UpRight(1));
                            }
                        }

                        if down.is_some() && down.unwrap().garden_plot != cell.garden_plot {
                            if !cell.corners.contains(&Corner::DownRight(1)) {
                                cell.corners.insert(Corner::DownRight(1));
                            }
                        }
                    }
                }

                //checking diagonal

                //get up neighbour left and right
                let up_neighbour_left = neighbours
                    .iter()
                    .find(|neighbour| neighbour.position.0 < cell.position.0)
                    .map_or(None, |neighbour| {
                        let up_neighbour = get_neighbours(&cloned_garden, (**neighbour).clone());

                        up_neighbour
                            .iter()
                            .find(|neighbour| neighbour.position.1 < cell.position.1)
                            .cloned()
                    });
                let up_neighbour_right = neighbours
                    .iter()
                    .find(|neighbour| neighbour.position.0 < cell.position.0)
                    .map_or(None, |neighbour| {
                        let up_neighbour = get_neighbours(&cloned_garden, (**neighbour).clone());

                        up_neighbour
                            .iter()
                            .find(|neighbour| neighbour.position.1 > cell.position.1)
                            .cloned()
                    });

                //get down neighbour left and right
                let down_neighbour_left = neighbours
                    .iter()
                    .find(|neighbour| neighbour.position.0 > cell.position.0)
                    .map_or(None, |neighbour| {
                        let down_neighbour = get_neighbours(&cloned_garden, (**neighbour).clone());

                        down_neighbour
                            .iter()
                            .find(|neighbour| neighbour.position.1 < cell.position.1)
                            .cloned()
                    });
                let down_neighbour_right = neighbours
                    .iter()
                    .find(|neighbour| neighbour.position.0 > cell.position.0)
                    .map_or(None, |neighbour| {
                        let down_neighbour = get_neighbours(&cloned_garden, (**neighbour).clone());

                        down_neighbour
                            .iter()
                            .find(|neighbour| neighbour.position.1 > cell.position.1)
                            .cloned()
                    });

                if up_neighbour_left.is_some()
                    && up_neighbour_left.unwrap().garden_plot != cell.garden_plot
                {
                    let up_neighbour = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.0 < cell.position.0);
                    let left_neighbour = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.1 < cell.position.1);

                    if up_neighbour.is_some()
                        && left_neighbour.is_some()
                        && up_neighbour.unwrap().garden_plot == cell.garden_plot
                        && left_neighbour.unwrap().garden_plot == cell.garden_plot
                    {
                        if !cell.corners.contains(&Corner::UpLeft(1)) {
                            cell.corners.insert(Corner::UpLeft(1));
                        }
                    }
                }

                if up_neighbour_right.is_some()
                    && up_neighbour_right.unwrap().garden_plot != cell.garden_plot
                {
                    let up_neighbour = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.0 < cell.position.0);
                    let right_neighbour = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.1 > cell.position.1);

                    if up_neighbour.is_some()
                        && right_neighbour.is_some()
                        && up_neighbour.unwrap().garden_plot == cell.garden_plot
                        && right_neighbour.unwrap().garden_plot == cell.garden_plot
                    {
                        if !cell.corners.contains(&Corner::UpRight(1)) {
                            cell.corners.insert(Corner::UpRight(1));
                        }
                    }
                }

                if down_neighbour_left.is_some()
                    && down_neighbour_left.unwrap().garden_plot != cell.garden_plot
                {
                    let down_neighbour = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.0 > cell.position.0);
                    let left_neighbour = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.1 < cell.position.1);

                    if down_neighbour.is_some()
                        && left_neighbour.is_some()
                        && down_neighbour.unwrap().garden_plot == cell.garden_plot
                        && left_neighbour.unwrap().garden_plot == cell.garden_plot
                    {
                        if !cell.corners.contains(&Corner::DownLeft(1)) {
                            cell.corners.insert(Corner::DownLeft(1));
                        }
                    }
                }

                if down_neighbour_right.is_some()
                    && down_neighbour_right.unwrap().garden_plot != cell.garden_plot
                {
                    let down_neighbour = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.0 > cell.position.0);
                    let right_neighbour = neighbours
                        .iter()
                        .find(|neighbour| neighbour.position.1 > cell.position.1);

                    if down_neighbour.is_some()
                        && right_neighbour.is_some()
                        && down_neighbour.unwrap().garden_plot == cell.garden_plot
                        && right_neighbour.unwrap().garden_plot == cell.garden_plot
                    {
                        if !cell.corners.contains(&Corner::DownRight(1)) {
                            cell.corners.insert(Corner::DownRight(1));
                        }
                    }
                }
            });
        });
    });
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut garden = parse_input_part_2(input);
    let garden_ori = parse_input(input);

    rezize_all_corner(&mut garden);

    let regions = get_all_regions(&garden_ori);

    let cost: usize = regions
        .iter()
        .map(|region| region.get_cost_with_corners(&garden))
        .sum();

    Some(cost as u32)
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
        assert_eq!(result, Some(140));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
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
        assert_eq!(result, Some(80));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
    }
}
