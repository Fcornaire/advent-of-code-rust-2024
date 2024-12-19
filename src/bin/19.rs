use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(19);

type Stripes = Vec<String>;

#[derive(Debug)]
struct Design {
    pub stripes: Stripes,
    pub towels: Vec<String>,
}

impl Design {
    fn check_for_valid_design(&self) -> Vec<(String, u64)> {
        let max_towel_design_length = self.stripes.iter().map(|s| s.len()).max().unwrap();

        self.towels
            .par_iter()
            .map(|towel| {
                let mut cache = HashMap::new();
                let valid_designs =
                    self.get_all_valid_design(&towel.clone(), max_towel_design_length, &mut cache);

                return (towel.clone(), valid_designs);
            })
            .collect()
    }

    fn get_all_valid_design(
        &self,
        towel: &String,
        max_towel_design_length: usize,
        cache: &mut HashMap<String, u64>,
    ) -> u64 {
        let all_possible_ind: Vec<usize> = (1..max_towel_design_length + 1)
            .filter(|i| *i <= towel.len())
            .collect();

        let desings: u64 = all_possible_ind
            .iter()
            .map(|i| {
                let (design_to_check, rest) = towel.split_at(*i);

                if self.stripes.contains(&design_to_check.to_string()) {
                    if rest.len() == 0 {
                        return 1 as u64;
                    }

                    if cache.contains_key(&rest.to_string()) {
                        return cache.get(&rest.to_string()).unwrap().clone();
                    }
                    return self.get_all_valid_design(
                        &rest.to_string(),
                        max_towel_design_length,
                        cache,
                    );
                }

                return 0;
            })
            .sum();

        cache.insert(towel.clone(), desings);

        return desings;
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

    let valids = design.check_for_valid_design();

    let count = valids.iter().filter(|(_, v)| *v > 0).count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let design = parse_input(input);

    let valid_towels = design.check_for_valid_design();

    let all_combs = valid_towels.iter().map(|(_, v)| v).sum();

    Some(all_combs)
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
        assert_eq!(result, Some(16));
    }
}
