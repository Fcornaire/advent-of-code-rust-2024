use std::collections::HashSet;

use itertools::Itertools;
use tracing::{debug, info, trace};

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spot {
    Empty,
    Wall,
    Robot,
    Box,
    BigBoxL,
    BigBoxR,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Warehouse {
    empty_spots: HashSet<(usize, usize)>,
    walls: HashSet<(usize, usize)>,
    robot: (usize, usize),
    boxes: HashSet<(usize, usize)>,
    big_boxes_l: HashSet<(usize, usize)>,
    big_boxes_r: HashSet<(usize, usize)>,
}

impl Warehouse {
    fn pretty_print(&self) {
        let (max_x, max_y) = self
            .empty_spots
            .iter()
            .chain(self.walls.iter())
            .chain(self.boxes.iter())
            .chain(std::iter::once(&self.robot))
            .fold((0, 0), |(max_x, max_y), (x, y)| {
                (max_x.max(*x), max_y.max(*y))
            });

        for y in 0..=max_y {
            let mut line = String::new();
            for x in 0..=max_x {
                let spot = if self.robot == (x, y) {
                    '@'
                } else if self.boxes.contains(&(x, y)) {
                    'O'
                } else if self.walls.contains(&(x, y)) {
                    '#'
                } else if self.empty_spots.contains(&(x, y)) {
                    '.'
                } else if self.big_boxes_l.contains(&(x, y)) {
                    '['
                } else if self.big_boxes_r.contains(&(x, y)) {
                    ']'
                } else {
                    ' '
                };

                line.push(spot);
            }
            debug!("{}", line);
        }

        debug!("");
    }

    fn get_gps(&self) -> usize {
        self.boxes.iter().fold(0, |acc, (x, y)| acc + (100 * y) + x)
    }

    fn get_big_box_gps(&self) -> usize {
        self.big_boxes_l
            .iter()
            .fold(0, |acc, (x, y)| acc + (100 * y) + x)
    }

    fn move_box(&mut self, pos: (usize, usize), direction: char) -> bool {
        let (bx, by) = pos;
        let new_box_pos = match direction {
            '^' => (bx, by - 1),
            'v' => (bx, by + 1),
            '<' => (bx - 1, by),
            '>' => (bx + 1, by),
            _ => panic!("unexpected direction: {}", direction),
        };

        if self.walls.contains(&new_box_pos) {
            return false;
        }

        if self.empty_spots.contains(&new_box_pos) {
            self.empty_spots.remove(&new_box_pos);
            self.empty_spots.insert(pos);
            self.boxes.remove(&pos);
            self.boxes.insert(new_box_pos);
            return true;
        }

        if self.boxes.contains(&new_box_pos) {
            if self.move_box(new_box_pos, direction) {
                self.empty_spots.remove(&new_box_pos);
                self.empty_spots.insert(pos);
                self.boxes.remove(&pos);
                self.boxes.insert(new_box_pos);
                return true;
            }
        }

        false
    }

    fn can_move_big_box(
        &self,
        pos: (usize, usize),
        direction: char,
        succesful_move: &mut HashSet<(usize, usize)>,
    ) -> bool {
        let (bx, by) = pos;
        let new_box_pos = match direction {
            '^' => (bx, by - 1),
            'v' => (bx, by + 1),
            '<' => (bx - 1, by),
            '>' => (bx + 1, by),
            _ => panic!("unexpected direction: {}", direction),
        };

        if self.walls.contains(&new_box_pos) {
            return false;
        }

        if self.empty_spots.contains(&new_box_pos) {
            if direction == '<' || direction == '>' {
                return true;
            }

            if succesful_move.contains(&new_box_pos) {
                return true;
            }

            let is_left_box = self.big_boxes_l.contains(&pos);
            if is_left_box {
                let right_box_pos = (bx + 1, by);

                succesful_move.insert(new_box_pos);

                return self.can_move_big_box(right_box_pos, direction, succesful_move);
            } else {
                let left_box_pos = (bx - 1, by);

                succesful_move.insert(new_box_pos);

                return self.can_move_big_box(left_box_pos, direction, succesful_move);
            }
        }

        if direction == '<' || direction == '>' {
            return self.can_move_big_box(new_box_pos, direction, succesful_move);
        }

        if succesful_move.contains(&new_box_pos) {
            return true;
        }

        let is_left_box = self.big_boxes_l.contains(&pos);
        if is_left_box {
            let right_box_pos = (bx + 1, by);
            let res = self.can_move_big_box(new_box_pos, direction, succesful_move);
            if !res {
                return false;
            }

            succesful_move.insert(new_box_pos);

            let res2: bool = self.can_move_big_box(right_box_pos, direction, succesful_move);

            return res2;
        } else {
            let left_box_pos = (bx - 1, by);
            let res: bool = self.can_move_big_box(new_box_pos, direction, succesful_move);
            if !res {
                return false;
            }

            succesful_move.insert(new_box_pos);

            let res2 = self.can_move_big_box(left_box_pos, direction, succesful_move);
            return res2;
        }
    }

    fn move_big_box(&mut self, pos: (usize, usize), direction: char, can_move: bool) -> bool {
        let (box_x, box_y) = pos;
        let new_big_box_pos = match direction {
            '^' => (box_x, box_y - 1),
            'v' => (box_x, box_y + 1),
            '<' => (box_x - 1, box_y),
            '>' => (box_x + 1, box_y),
            _ => panic!("unexpected direction: {}", direction),
        };

        if self.walls.contains(&new_big_box_pos) {
            return false;
        }

        if direction == '<' || direction == '>' {
            if self.empty_spots.contains(&new_big_box_pos) {
                self.empty_spots.remove(&new_big_box_pos);
                self.empty_spots.insert(pos);

                let is_left_box = self.big_boxes_l.contains(&pos);
                if is_left_box {
                    self.big_boxes_l.remove(&pos);
                    self.big_boxes_l.insert(new_big_box_pos);
                    return true;
                } else {
                    self.big_boxes_r.remove(&pos);
                    self.big_boxes_r.insert(new_big_box_pos);
                    return true;
                }
            }

            let is_left_box = self.big_boxes_l.contains(&pos);
            if is_left_box {
                if self.move_big_box(new_big_box_pos, direction, false) {
                    self.empty_spots.remove(&new_big_box_pos);
                    self.empty_spots.insert(pos);
                    self.big_boxes_l.remove(&pos);
                    self.big_boxes_l.insert(new_big_box_pos);
                    return true;
                }
            } else {
                if self.move_big_box(new_big_box_pos, direction, false) {
                    self.empty_spots.remove(&new_big_box_pos);
                    self.empty_spots.insert(pos);
                    self.big_boxes_r.remove(&pos);
                    self.big_boxes_r.insert(new_big_box_pos);
                    return true;
                }
            }
        }

        let is_left_box = self.big_boxes_l.contains(&pos);

        if self.empty_spots.contains(&new_big_box_pos) {
            if can_move {
                if is_left_box {
                    self.empty_spots.remove(&new_big_box_pos);
                    self.empty_spots.insert(pos);
                    self.big_boxes_l.remove(&pos);
                    self.big_boxes_l.insert(new_big_box_pos);
                } else {
                    self.empty_spots.remove(&new_big_box_pos);
                    self.empty_spots.insert(pos);
                    self.big_boxes_r.remove(&pos);
                    self.big_boxes_r.insert(new_big_box_pos);
                }
                return true;
            }

            if is_left_box {
                let right_box_pos = (box_x + 1, box_y);

                if self.move_big_box(right_box_pos, direction, true) {
                    self.empty_spots.remove(&new_big_box_pos);
                    self.empty_spots.insert(pos);
                    self.big_boxes_l.remove(&pos);
                    self.big_boxes_l.insert(new_big_box_pos);
                    return true;
                }
            } else {
                let left_box_pos = (box_x - 1, box_y);

                if self.move_big_box(left_box_pos, direction, true) {
                    self.empty_spots.remove(&new_big_box_pos);
                    self.empty_spots.insert(pos);
                    self.big_boxes_r.remove(&pos);
                    self.big_boxes_r.insert(new_big_box_pos);
                    return true;
                }
            }
        }

        if is_left_box {
            let right_box_pos = (box_x + 1, box_y);

            if self.move_big_box(new_big_box_pos, direction, false)
                && self.move_big_box(right_box_pos, direction, true)
            {
                self.empty_spots.remove(&new_big_box_pos);
                self.empty_spots.insert(pos);
                self.big_boxes_l.remove(&pos);
                self.big_boxes_l.insert(new_big_box_pos);
                return true;
            }
        } else {
            let left_box_pos = (box_x - 1, box_y);

            if self.move_big_box(new_big_box_pos, direction, false)
                && self.move_big_box(left_box_pos, direction, true)
            {
                self.empty_spots.remove(&new_big_box_pos);
                self.empty_spots.insert(pos);
                self.big_boxes_r.remove(&pos);
                self.big_boxes_r.insert(new_big_box_pos);
                return true;
            }
        }

        false
    }

    pub fn move_robot(&mut self, direction: char) {
        let (x, y) = self.robot;

        let new_pos = match direction {
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            _ => panic!("unexpected direction: {}", direction),
        };

        if self.walls.contains(&new_pos) {
            return;
        }

        if self.empty_spots.contains(&new_pos) {
            self.empty_spots.remove(&new_pos);
            self.empty_spots.insert(self.robot);
            self.robot = new_pos;
            return;
        }

        //check for box
        if self.boxes.contains(&new_pos) {
            if self.move_box(new_pos, direction) {
                self.empty_spots.remove(&new_pos);
                self.empty_spots.insert(self.robot);
                self.robot = new_pos;
            }
        }

        //check for big box
        if self.big_boxes_l.contains(&new_pos) || self.big_boxes_r.contains(&new_pos) {
            let mut succesful_move: HashSet<(usize, usize)> = HashSet::new();
            if self.can_move_big_box(new_pos, direction, &mut succesful_move) {
                self.move_big_box(new_pos, direction, false);
                self.empty_spots.remove(&new_pos);
                self.empty_spots.insert(self.robot);
                self.robot = new_pos;
            }
        }
    }
}

fn duplicate(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '.' => {
                result.push(c);
                result.push(c);
            }
            '#' => {
                result.push(c);
                result.push(c);
            }
            '@' => {
                result.push('@');
                result.push('.');
            }
            'O' => {
                result.push('[');
                result.push(']');
            }
            _ => panic!("unexpected character: {}", c),
        }
    }
    result
}

fn parse_input(input: &str) -> (Warehouse, String, usize, usize) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let height = parts[0].lines().count();
    let width = parts[0].lines().map(|line| line.len()).max().unwrap();

    let warehouse = parts[0]
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let spot = match c {
                    '.' => Spot::Empty,
                    '#' => Spot::Wall,
                    '@' => Spot::Robot,
                    'O' => Spot::Box,
                    '[' => Spot::BigBoxL,
                    ']' => Spot::BigBoxR,
                    _ => panic!("unexpected character: {}", c),
                };

                match spot {
                    Spot::Robot => Some(((x, y), Spot::Robot)),
                    Spot::Box => Some(((x, y), Spot::Box)),
                    Spot::Wall => Some(((x, y), Spot::Wall)),
                    Spot::Empty => Some(((x, y), Spot::Empty)),
                    Spot::BigBoxL => Some(((x, y), Spot::BigBoxL)),
                    Spot::BigBoxR => Some(((x, y), Spot::BigBoxR)),
                }
            })
        })
        .fold(
            Warehouse {
                empty_spots: HashSet::new(),
                robot: (0, 0),
                boxes: HashSet::new(),
                walls: HashSet::new(),
                big_boxes_l: HashSet::new(),
                big_boxes_r: HashSet::new(),
            },
            |mut warehouse, spot| {
                if let Some((pos, spot)) = spot {
                    match spot {
                        Spot::Empty => {
                            warehouse.empty_spots.insert(pos);
                        }
                        Spot::Robot => {
                            warehouse.robot = pos;
                        }
                        Spot::Box => {
                            warehouse.boxes.insert(pos);
                        }
                        Spot::Wall => {
                            warehouse.walls.insert(pos);
                        }
                        Spot::BigBoxL => {
                            warehouse.big_boxes_l.insert(pos);
                        }
                        Spot::BigBoxR => {
                            warehouse.big_boxes_r.insert(pos);
                        }
                    }
                }

                warehouse
            },
        );

    //remove all \n in the second part
    let movements = parts[1].replace("\n", "");

    (warehouse, movements, width, height)
}

fn parse_input_with_dedup(input: &str) -> (Warehouse, String) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let warehouse = parts[0]
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let dedup = duplicate(line);
            dedup
                .chars()
                .enumerate()
                .map(move |(x, c)| {
                    let spot = match c {
                        '.' => Spot::Empty,
                        '#' => Spot::Wall,
                        '@' => Spot::Robot,
                        'O' => Spot::Box,
                        '[' => Spot::BigBoxL,
                        ']' => Spot::BigBoxR,
                        _ => panic!("unexpected character: {}", c),
                    };

                    match spot {
                        Spot::Robot => Some(((x, y), Spot::Robot)),
                        Spot::Box => Some(((x, y), Spot::Box)),
                        Spot::Wall => Some(((x, y), Spot::Wall)),
                        Spot::Empty => Some(((x, y), Spot::Empty)),
                        Spot::BigBoxL => Some(((x, y), Spot::BigBoxL)),
                        Spot::BigBoxR => Some(((x, y), Spot::BigBoxR)),
                    }
                })
                .collect::<Vec<_>>()
        })
        .fold(
            Warehouse {
                empty_spots: HashSet::new(),
                robot: (0, 0),
                boxes: HashSet::new(),
                walls: HashSet::new(),
                big_boxes_l: HashSet::new(),
                big_boxes_r: HashSet::new(),
            },
            |mut warehouse, spot| {
                if let Some((pos, spot)) = spot {
                    match spot {
                        Spot::Empty => {
                            warehouse.empty_spots.insert(pos);
                        }
                        Spot::Robot => {
                            warehouse.robot = pos;
                        }
                        Spot::Box => {
                            warehouse.boxes.insert(pos);
                        }
                        Spot::Wall => {
                            warehouse.walls.insert(pos);
                        }
                        Spot::BigBoxL => {
                            warehouse.big_boxes_l.insert(pos);
                        }
                        Spot::BigBoxR => {
                            warehouse.big_boxes_r.insert(pos);
                        }
                    }
                }

                warehouse
            },
        );

    //remove all \n in the second part
    let movements = parts[1].replace("\n", "");

    (warehouse, movements)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut warehouse, movements, _, _) = parse_input(input);

    println!("warehouse before moving:");
    // warehouse.pretty_print();

    for movement in movements.chars() {
        warehouse.move_robot(movement);
        // println!("warehouse after moving: {}", movement);
        // warehouse.pretty_print();
    }

    let gps = warehouse.get_gps();

    Some(gps as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut warehouse, movements) = parse_input_with_dedup(input);

    // info!("Warehouse initial state:");
    // warehouse.pretty_print();

    for movement in movements.chars() {
        warehouse.move_robot(movement);
        // info!("Warehouse after moving: {}", movement);
        // warehouse.pretty_print();
    }

    let gps = warehouse.get_big_box_gps();

    Some(gps as u64)
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
        assert_eq!(result, Some(2028));
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
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
