use tracing::{debug, info, trace};

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

        if stripes.len() == 0 {
            debug!("no stripes found for towel: {:#?}", towel);
        }

        for stripe in stripes.iter() {
            if stripe == &towel {
                return (true, towel_length);
            }
        }

        return self.is_valid_towel_design(&towel[..towel_length - 1].to_string());
    }

    fn get_searching_length(&self, pointer: usize, towel: &String) -> usize {
        trace!("searching length for towel: {:#?}", towel);

        trace!("towel length: {}", towel.len());

        if towel.len() == 0 {
            return 0;
        }

        let max_found_length = self
            .stripes
            .iter()
            .filter(|s| s.len() == towel.len())
            .map(|s| s.len())
            .max()
            .unwrap();

        if max_found_length > 0 {
            if pointer + max_found_length < towel.len() {
                return towel.len();
            }

            return self.get_searching_length(pointer, &towel[..towel.len() - 1].to_string());
        } else {
            return self.get_searching_length(pointer, &towel[..towel.len() - 1].to_string());
        }
    }

    fn check_design(&self, towel: &mut String) -> Option<String> {
        let mut pointer = 0;
        // let max_stripe_length = self.stripes.iter().map(|s| s.len()).max().unwrap();

        while pointer < towel.len() {
            let searching_length = self.get_searching_length(pointer, towel);
            if searching_length == 0 {
                break;
            }

            let end_pointer = pointer + searching_length;

            debug!("pointer: {}, end_pointer: {}", pointer, end_pointer);
            debug!("checking towel: {:#?}", &towel[pointer..end_pointer]);

            let (is_valid, actual_lenght_found) =
                self.is_valid_towel_design(&towel[pointer..end_pointer].to_string());

            if is_valid {
                debug!("is valiid towel: {:#?}", &towel[pointer..end_pointer]);
                pointer = pointer + actual_lenght_found;
                *towel = towel[pointer..].to_string();

                debug!("new towel to search {}", towel);

                if pointer == towel.len() {
                    return Some(towel.to_string());
                }
            } else {
                break;
            }
        }

        if pointer == towel.len() {
            return Some(towel.to_string());
        }

        None
    }

    pub fn get_all_valid_towels_design(&self) -> Vec<String> {
        let mut valid_towels = Vec::new();

        self.towels.iter().for_each(|towel| {
            debug!("towel: {:#?}", towel);
            let mut towel = towel.clone();
            if let Some(valid_towel) = self.check_design(&mut towel) {
                valid_towels.push(valid_towel);
            }
        });

        valid_towels
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

    None
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
