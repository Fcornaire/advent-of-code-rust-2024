use indicatif::ParallelProgressIterator;

use rayon::prelude::*;

advent_of_code::solution!(13);

fn find_smallest_n_m(x1: i64, y1: i64, z1: i64, x2: i64, y2: i64, z2: i64) -> Option<(i64, i64)> {
    (0..=100).into_par_iter().find_map_any(|n| {
        (0..=100).into_par_iter().find_map_any(|m| {
            if x1 * n as i64 + y1 * m as i64 == z1 && x2 * n as i64 + y2 * m as i64 == z2 {
                Some((n as i64, m as i64))
            } else {
                None
            }
        })
    })
}

fn solve_linear_equation(
    a1: i64,
    b1: i64,
    c1: i64,
    a2: i64,
    b2: i64,
    c2: i64,
) -> Option<(i64, i64)> {
    let determinant = a1 * b2 - a2 * b1;

    if determinant == 0 {
        return None;
    }

    let x_numerator = c1 * b2 - c2 * b1;
    let y_numerator = a1 * c2 - a2 * c1;

    if x_numerator % determinant != 0 || y_numerator % determinant != 0 {
        return None;
    }

    let x = x_numerator / determinant;
    let y = y_numerator / determinant;

    Some((x, y))
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    pub prize_location: (i64, i64),
    pub button_a_offset: (i64, i64),
    pub button_b_offset: (i64, i64),
}

impl Machine {
    fn add_to_prize(&mut self, x: i64, y: i64) {
        self.prize_location.0 += x;
        self.prize_location.1 += y;
    }

    fn get_cheapest(&self) -> i64 {
        let a_x = self.button_a_offset.0;
        let b_x = self.button_b_offset.0;

        let a_y = self.button_a_offset.1;
        let b_y = self.button_b_offset.1;

        let smallest = solve_linear_equation(
            a_x,
            b_x,
            self.prize_location.0,
            a_y,
            b_y,
            self.prize_location.1,
        );

        if smallest.is_some() {
            let (minimal_a_x, minimal_b_x) = smallest.unwrap();

            return minimal_a_x * 3 + minimal_b_x;
        }

        0
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let parts: Vec<&str> = input
        .split("\n\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.trim())
        .collect();

    let parts: Vec<Vec<&str>> = parts.iter().map(|x| x.split('\n').collect()).collect();

    parts
        .iter()
        .map(|machine| {
            let button_a_x = machine[0]
                .split(',')
                .take(1)
                .next()
                .unwrap()
                .split('+')
                .skip(1)
                .next()
                .unwrap();
            let button_a_y = machine[0]
                .split(',')
                .skip(1)
                .next()
                .unwrap()
                .split('+')
                .skip(1)
                .next()
                .unwrap();

            let button_b_x = machine[1]
                .split(',')
                .take(1)
                .next()
                .unwrap()
                .split('+')
                .skip(1)
                .next()
                .unwrap();
            let button_b_y = machine[1]
                .split(',')
                .skip(1)
                .next()
                .unwrap()
                .split('+')
                .skip(1)
                .next()
                .unwrap();

            let prize_x = machine[2]
                .split(',')
                .take(1)
                .next()
                .unwrap()
                .split('=')
                .skip(1)
                .next()
                .unwrap();
            let prize_y = machine[2]
                .split(',')
                .skip(1)
                .next()
                .unwrap()
                .split('=')
                .skip(1)
                .next()
                .unwrap();

            Machine {
                prize_location: (prize_x.parse().unwrap(), prize_y.parse().unwrap()),
                button_a_offset: (button_a_x.parse().unwrap(), button_a_y.parse().unwrap()),
                button_b_offset: (button_b_x.parse().unwrap(), button_b_y.parse().unwrap()),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse_input(input);

    Some(
        machines
            .par_iter()
            .progress()
            .map(|x| {
                let cheap = x.get_cheapest();

                cheap
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut machines = parse_input(input);

    let to_add = 10000000000000;

    machines.iter_mut().for_each(|x| {
        x.add_to_prize(to_add, to_add);
    });

    Some(
        machines
            .par_iter()
            .map(|x| {
                let cheap = x.get_cheapest();

                cheap
            })
            .sum(),
    )
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
        assert_eq!(result, Some(480));
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
        assert_eq!(result, Some(875_318_608_908));
    }
}
