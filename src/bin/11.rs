use std::collections::HashMap;
use tracing::{debug, info};

advent_of_code::solution!(11);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Stone {
    Zero,
    EvenDigit(u64),
    OddDigit(u64),
}

impl std::fmt::Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stone::Zero => write!(f, "0"),
            Stone::EvenDigit(n) => write!(f, "Even({})", n),
            Stone::OddDigit(n) => write!(f, "Odd({})", n),
        }
    }
}

impl Stone {
    fn new(s: &str) -> Self {
        match s {
            "0" => Stone::Zero,
            s => {
                if s.len() % 2 == 0 {
                    Stone::EvenDigit(s.parse().unwrap())
                } else {
                    Stone::OddDigit(s.parse().unwrap())
                }
            }
        }
    }
}

type Stones = HashMap<Stone, u64>;

struct PileOfStones {
    stones: Stones,
}

fn split_number(n: u64) -> (u64, u64) {
    let s = n.to_string();
    let len = s.len();
    let mid = len / 2;

    let (left, right) = s.split_at(mid);
    let left_num = left.parse::<u64>().unwrap_or(0);
    let right_num = right.parse::<u64>().unwrap_or(0);

    (left_num, right_num)
}

impl PileOfStones {
    pub fn get_stones_count(&self) -> u64 {
        self.stones.values().sum::<u64>()
    }

    fn pretty_print(&self) {
        for (stone, count) in self.stones.iter() {
            debug!("{}: {}", stone, count);
        }
    }

    fn blink(&mut self) {
        let mut new_stones = Stones::with_capacity(self.stones.len());
        for (stone, count) in self.stones.iter() {
            let num = match stone {
                Stone::Zero => 0,
                Stone::EvenDigit(n) => *n,
                Stone::OddDigit(n) => *n,
            };

            match stone {
                Stone::Zero => {
                    *new_stones.entry(Stone::OddDigit(1)).or_default() += count;
                }
                Stone::OddDigit(1) => {
                    *new_stones.entry(Stone::EvenDigit(2024)).or_default() += count;
                }
                _ => {
                    if num.to_string().len() % 2 == 0 {
                        let (left, right) = split_number(num);
                        let l = Stone::new(&left.to_string());
                        let r = Stone::new(&right.to_string());
                        *new_stones.entry(l).or_default() += count;
                        *new_stones.entry(r).or_default() += count;
                    } else {
                        let entry = Stone::new(format!("{}", 2024 * num).as_str());
                        *new_stones.entry(entry).or_default() += count;
                    }
                }
            }
        }

        self.stones = new_stones;
    }

    pub fn blink_n_times(&mut self, n: u64) {
        for _ in 0..n {
            self.blink();
        }
    }
}

fn parse_input(input: &str) -> PileOfStones {
    let stones = input
        .trim()
        .split_whitespace()
        .map(|s| Stone::new(s))
        .map(|s| (s, 1))
        .collect();

    PileOfStones { stones }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut pile_of_stones = parse_input(input);

    pile_of_stones.blink_n_times(25);

    Some(pile_of_stones.get_stones_count())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pile_of_stones = parse_input(input);

    pile_of_stones.blink_n_times(75);

    Some(pile_of_stones.get_stones_count())
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
        assert_eq!(result, Some(55312));
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
        assert_eq!(result, Some(65601038650482));
    }
}
