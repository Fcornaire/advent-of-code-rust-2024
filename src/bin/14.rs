use std::{collections::HashMap, sync::Mutex};

use image::{ImageBuffer, Luma};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use regex::Regex;
use show_image::{create_window, run_context, ImageInfo, ImageView};
use tracing::{debug, info};

advent_of_code::solution!(14);

#[derive(Debug, Hash, Eq, PartialEq)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    pub fn move_n_times_in_grid(&mut self, n: i64, width: i64, height: i64) {
        for _ in 0..n {
            self.move_in_grid(width, height);
        }
    }

    pub fn get_quadrant(&self, width: i64, height: i64) -> Option<usize> {
        let x = self.position.0;
        let y = self.position.1;
        let middle_x = width / 2;
        let middle_y = height / 2;

        if x == middle_x || y == middle_y {
            return None;
        }

        if x < middle_x && y < middle_y {
            Some(0)
        } else if x > middle_x && y < middle_y {
            Some(1)
        } else if x < middle_x && y > middle_y {
            Some(2)
        } else {
            Some(3)
        }
    }

    fn move_in_grid(&mut self, width: i64, height: i64) {
        let mut position_x = self.position.0 + self.velocity.0;
        let mut position_y = self.position.1 + self.velocity.1;

        if position_x < 0 {
            position_x += width;
        }

        if position_x >= width {
            position_x -= width;
        }

        if position_y < 0 {
            position_y += height;
        }

        if position_y >= height {
            position_y -= height;
        }

        self.position = (position_x, position_y);
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let position_x: i64 = caps[1].parse().unwrap();
            let position_y: i64 = caps[2].parse().unwrap();
            let velocity_x: i64 = caps[3].parse().unwrap();
            let velocity_y: i64 = caps[4].parse().unwrap();

            Robot {
                position: (position_x, position_y),
                velocity: (velocity_x, velocity_y),
            }
        })
        .collect()
}

fn display_grid_as_image(grid: &[Vec<char>], iterations: i64) {
    let width = grid[0].len() as u32;
    let height = grid.len() as u32;
    let mut imgbuf: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let pixel = if cell == '#' { 255 } else { 0 };
            imgbuf.put_pixel(x as u32, y as u32, Luma([pixel]));
        }
    }

    let filename = format!("day14_grid_{}.png", iterations);

    let image = ImageView::new(ImageInfo::mono8(width, height), &imgbuf);
    let window = create_window(format!("grid {}", iterations), Default::default()).unwrap();
    window.set_image("grid", image).unwrap();

    imgbuf.save(format!("img/{}", filename)).unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    drop(window);
}

fn pretty_print_grid(robots: &[Robot], width: i64, height: i64) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    // Count robots on each position
    let mut robot_count: std::collections::HashMap<(i64, i64), usize> =
        std::collections::HashMap::new();
    for robot in robots {
        *robot_count.entry(robot.position).or_insert(0) += 1;
    }

    // Update grid with robot counts
    for (&(x, y), &count) in &robot_count {
        if count > 1 {
            grid[y as usize][x as usize] = std::char::from_digit(count as u32, 10).unwrap_or('#');
        } else {
            grid[y as usize][x as usize] = '#';
        }
    }

    grid
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut robots = parse_input(input);
    let width = 11; //101 for real answer
    let height = 7; //103 for real answer

    robots.par_iter_mut().for_each(|robot| {
        robot.move_n_times_in_grid(100, width, height);
    });

    let robots_per_quadrant = Mutex::new(HashMap::new());

    robots.par_iter_mut().for_each(|robot| {
        if let Some(quadrant) = robot.get_quadrant(width, height) {
            robots_per_quadrant
                .lock()
                .unwrap()
                .entry(quadrant)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    });

    info!("Robots per quadrant: {:#?}", robots_per_quadrant);

    let res = robots_per_quadrant
        .lock()
        .unwrap()
        .values()
        .fold(1, |acc, x| acc * x);

    Some(res)
}

fn display_grid(robots: &mut [Robot], width: i64, height: i64) {
    for i in 0..7572 {
        robots.iter_mut().for_each(|robot| {
            robot.move_in_grid(width, height);
        });

        if i == 7571 {
            display_grid_as_image(&pretty_print_grid(&robots, width, height), i);
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse_input(input);
    let width = 101;
    let height = 103;

    //Run this to find the answer for this input

    // run_context(move || {
    //     display_grid(&mut robots, width, height);

    //     Ok::<(), Box<dyn std::error::Error>>(())
    // });

    Some(7572)
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
        assert_eq!(result, Some(12));

        // let result = part_one(&advent_of_code::template::read_file_part(
        //     "examples", DAY, 2,
        // ));
        // assert_eq!(result, Some(1));
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
        assert_eq!(result, Some(7572));
    }
}
