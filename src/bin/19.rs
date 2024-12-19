use std::{cmp::min, collections::VecDeque};

use indicatif::ParallelProgressIterator;
use tracing::{debug, info, trace};

use rayon::prelude::*;

advent_of_code::solution!(19);

type Stripes = Vec<String>;

#[derive(Debug)]
struct Design {
    pub stripes: Stripes,
    pub towels: Vec<String>,
}

impl Design {
    fn is_valid_towel_design(&self, towel: &String) -> (bool, usize) {
        let towel_length = towel.len();

        if towel_length == 0 {
            return (false, 0);
        }

        let stripes = self
            .stripes
            .iter()
            .filter(|s| s.len() == towel_length)
            .collect::<Vec<&String>>();

        for stripe in stripes.iter() {
            if stripe == &towel {
                return (true, towel_length);
            }
        }

        return self.is_valid_towel_design(&towel[..towel_length - 1].to_string());
    }

    fn get_searching_length(
        &self,
        pointer: usize,
        original_towel_len: usize,
        towel_mutable: &String,
    ) -> Vec<(usize, usize)> {
        if towel_mutable.len() == 0 {
            return vec![];
        }

        let max_stripe_length_searchable = min(
            self.stripes.iter().map(|s| s.len()).max().unwrap(),
            towel_mutable.len(),
        );

        let max_found_length = self
            .stripes
            .iter()
            .filter(|s| s.len() == max_stripe_length_searchable)
            .map(|s| s.len())
            .max();

        if max_found_length.is_none() {
            return vec![];
        }

        let max_found_length = max_found_length.unwrap();

        (0..max_found_length + 1)
            .into_iter()
            .filter(|length| pointer + length <= original_towel_len)
            .map(|length| (pointer, length))
            .collect()
    }

    fn check_design(&self, towel: &mut String) -> Option<String> {
        let clone = towel.clone();

        let mut stack = self.get_searching_length(0, clone.len(), &clone[0..].to_string().clone());

        while stack.len() > 0 {
            let (current_pointer, current_searching_length) = stack.pop().unwrap();

            if current_searching_length == 0 {
                continue;
            }

            let end_pointer = current_pointer + current_searching_length;

            let (is_valid, actual_lenght_found) = self
                .is_valid_towel_design(&clone[current_pointer..end_pointer].to_string().clone());

            if is_valid {
                let actual_end_pointer = current_pointer + actual_lenght_found;

                if actual_end_pointer == clone.len() {
                    return Some(clone.to_string());
                }

                self.get_searching_length(
                    actual_end_pointer,
                    clone.len(),
                    &clone[actual_end_pointer..].to_string().clone(),
                )
                .iter()
                .for_each(|(p, l)| stack.push((*p, *l)));
            }
        }

        None
    }

    pub fn get_all_valid_towels_design(&self) -> Vec<String> {
        self.towels
            .par_iter()
            .progress()
            .map(|towel| {
                let mut towel = towel.clone();
                if let Some(valid_towel) = self.check_design(&mut towel) {
                    // valid_towels.push(valid_towel);
                    return valid_towel;
                }

                return "".to_string();
            })
            .filter(|towel| towel.len() > 0)
            .collect::<Vec<String>>()
    }
}

fn parse_input(input: &str) -> Design {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let stripes = parts[0]
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    let towels = parts[1].lines().map(|l| l.to_string()).collect();

    Design { stripes, towels }
}

pub fn part_one(input: &str) -> Option<u32> {
    let design = parse_input(input);

    debug!("design: {:#?}", design);
    let valid_towels = design.get_all_valid_towels_design();

    info!("valid towels: {:#?}", valid_towels);

    Some(valid_towels.len() as u32)
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

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
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
