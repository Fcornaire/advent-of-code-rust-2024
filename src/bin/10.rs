advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Trail {
    pub position: (usize, usize),
    pub height: usize,
    pub heads: Option<Vec<Trail>>,
    pub already_visited_positions: Vec<(usize, usize)>,
}

impl Trail {
    pub fn score(&self) -> u32 {
        let mut next_height = self.height + 1;
        let mut clone = self.heads.clone();

        while clone.is_some() {
            let heads = clone.as_ref().unwrap();

            if heads.iter().any(|trail| trail.height != next_height) {
                return 0;
            }

            let same_height_trails: Vec<Trail> = heads
                .iter()
                .filter(|trail| trail.height == next_height)
                .flat_map(|trail| trail.heads.clone().unwrap_or_default())
                .collect();

            if next_height == 9 {
                let mut counted_positions = vec![];
                let mut score = 0;
                for trail in heads.iter() {
                    if counted_positions.contains(&trail.position) {
                        continue;
                    }

                    score += 1;
                    counted_positions.push(trail.position);
                }

                return score;
            }

            if same_height_trails.is_empty() {
                return 0;
            }

            clone = Some(same_height_trails.clone());
            next_height += 1;
        }

        return 0;
    }

    pub fn rating(&self) -> u32 {
        let mut next_height = self.height + 1;
        let mut clone = self.heads.clone();

        while clone.is_some() {
            let heads = clone.as_ref().unwrap();

            if heads.iter().any(|trail| trail.height != next_height) {
                return 0;
            }

            let same_height_trails: Vec<Trail> = heads
                .iter()
                .filter(|trail| trail.height == next_height)
                .flat_map(|trail| trail.heads.clone().unwrap_or_default())
                .collect();

            if next_height == 9 {
                return heads.len() as u32;
            }

            if same_height_trails.is_empty() {
                return 0;
            }

            clone = Some(same_height_trails.clone());
            next_height += 1;
        }

        return 0;
    }

    pub fn update_head(&mut self, grid: &Vec<Vec<usize>>) -> Option<Vec<Trail>> {
        let mut positions = vec![
            self.position.1.checked_sub(1).map(|y| (self.position.0, y)),
            Some((self.position.0 + 1, self.position.1)),
            Some((self.position.0, self.position.1 + 1)),
            self.position.0.checked_sub(1).map(|x| (x, self.position.1)),
        ];

        positions = positions
            .into_iter()
            .filter(|pos| {
                if let Some(pos) = pos {
                    !self.already_visited_positions.contains(&pos)
                } else {
                    false
                }
            })
            .collect();

        let mut res: Vec<Trail> = vec![];

        for (x, y) in positions.clone().into_iter().flatten() {
            if let Some(height) = grid.get(y).and_then(|row| row.get(x)) {
                if self.height < *height && height.abs_diff(self.height) == 1 {
                    self.already_visited_positions.push((x, y));

                    let mut new_trail = Trail {
                        position: (x, y),
                        height: *height,
                        heads: None,
                        already_visited_positions: self.already_visited_positions.clone(),
                    };

                    new_trail.update_head(grid);

                    res.push(new_trail);
                    self.heads = Some(res.clone());
                }
            }
        }

        match res.is_empty() {
            true => None,
            false => Some(res),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn build_all_trailheads(grid: Vec<Vec<usize>>, all_possible_trailheads: &mut Vec<Trail>) {
    for trailhead in all_possible_trailheads.iter_mut() {
        let res = trailhead.update_head(&grid);

        trailhead.heads = res;
    }
}

fn get_all_trailheads(grid: &Vec<Vec<usize>>) -> Vec<Trail> {
    let mut all_trailheads = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                let new_trail = Trail {
                    position: (x, y),
                    height: *height,
                    heads: None,
                    already_visited_positions: vec![(x, y)],
                };

                all_trailheads.push(new_trail);
            }
        }
    }

    all_trailheads
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut all_possible_trailheads: Vec<Trail> = get_all_trailheads(&grid);
    build_all_trailheads(grid.clone(), &mut all_possible_trailheads);

    let mut score = 0;
    let mut counted_positions = vec![];

    all_possible_trailheads.iter().for_each(|trail| {
        if counted_positions.contains(&trail.position) {
            return;
        }
        score += trail.score();
        counted_positions.push(trail.position);
    });

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut all_possible_trailheads: Vec<Trail> = get_all_trailheads(&grid);
    build_all_trailheads(grid.clone(), &mut all_possible_trailheads);

    // let valid_trailheads = get_all_valid_headtrail(all_possible_trailheads.clone());
    // info!("Valid trailheads: {:#?}", all_possible_trailheads);

    let mut rating = 0;

    all_possible_trailheads.iter().for_each(|trail| {
        rating += trail.rating();
    });

    Some(rating)
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
        assert_eq!(result, Some(1));

        let result_3 = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result_3, Some(2));

        let result_4 = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result_4, Some(4));

        let result_2 = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result_2, Some(36));
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
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(3));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(81));
    }
}
