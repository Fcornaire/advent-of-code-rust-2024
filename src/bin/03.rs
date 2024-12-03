use itertools::Itertools;
use regex::Regex;
use tracing::info;

advent_of_code::solution!(3);

fn extract_valid_mul_instructions(line: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(line)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            (x, y)
        })
        .collect()
}

fn calculate_mul_results(instructions: Vec<(i32, i32)>) -> Vec<i32> {
    instructions.iter().map(|(x, y)| x * y).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let valid_instructions = extract_valid_mul_instructions(input);
    info!("valid_instructions: {:?}", valid_instructions);

    let results = calculate_mul_results(valid_instructions);
    info!("results: {:?}", results);

    Some(results.iter().sum::<i32>() as u32)
}

fn extract_valid_mul_instructions_with_do_dont(line: &str) -> Vec<(i32, i32)> {
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut instructions = Vec::new();
    let dont_start_end_index:Vec<usize> = re_dont.captures_iter(line).map(|cap| cap.get(0).unwrap().start()).sorted() .collect();

    let mut do_start_end_index:Vec<usize> = re_do.captures_iter(line).map(|cap| cap.get(0).unwrap().start()).sorted().collect();
    do_start_end_index.insert(0, 0);


    re_mul.captures_iter(line).for_each((|cap| {
        let first_dont_index_before_current = dont_start_end_index.iter().filter(|filt| filt > &&cap.get(0).unwrap().start()).min().unwrap_or(&0 );
        let first_do_index_before_current = do_start_end_index.iter().filter(|filt| filt < &&cap.get(0).unwrap().start()).max().unwrap_or(&0 );

        info!("first_do_index_before_current {:?} < {:?} < first_dont_index_before_current {:?}", first_do_index_before_current, cap.get(0).unwrap().start(), first_dont_index_before_current);

        if *first_do_index_before_current < cap.get(0).unwrap().start() && cap.get(0).unwrap().start() > *first_dont_index_before_current   {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();

            instructions.push((x, y));
        }

    }));

    instructions
}

pub fn part_two(input: &str) -> Option<u32> {
    let valid_instructions = extract_valid_mul_instructions_with_do_dont(input);
    info!("valid_instructions: {:?}", valid_instructions);

    let results = calculate_mul_results(valid_instructions);
    info!("results: {:?}", results);

    Some(results.iter().sum::<i32>() as u32)
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
        assert_eq!(result, Some(161));
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
        assert_eq!(result, Some(48));
    }
}
