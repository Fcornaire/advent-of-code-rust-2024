use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use tracing::info;

advent_of_code::solution!(7);

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
            Operator::Concatenate => write!(f, "||"),
        }
    }
}

impl std::fmt::Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operations::Operator(o) => write!(f, "{}", o),
            Operations::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Operations {
    Operator(Operator),
    Number(u64),
}

fn generate_operations(numbers: &[u64]) -> Vec<Vec<Operations>> {
    let mut results: Vec<Vec<Operations>> = vec![];

    let n = numbers.len();
    let max_combinations = 2_usize.pow((n - 1) as u32);

    for i in 0..max_combinations {
        let mut current = vec![];
        for j in 0..n {
            current.push(Operations::Number(numbers[j]));
            if i & (1 << j) != 0 {
                current.push(Operations::Operator(Operator::Add));
            } else {
                current.push(Operations::Operator(Operator::Multiply));
            }
        }
        current.pop();
        results.push(current);
    }

    results
}

fn generate_operations_with_concat(numbers: &[u64]) -> Vec<Vec<Operations>> {
    let n = numbers.len();
    let operators = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];
    let total_combinations = 3_usize.pow((n - 1) as u32);

    (0..total_combinations)
        .into_par_iter()
        .map(|i| {
            let mut current = Vec::new();
            current.push(Operations::Number(numbers[0]));

            for j in 0..(n - 1) {
                let op_index = (i / 3_usize.pow(j as u32)) % 3;
                current.push(Operations::Operator(operators[op_index].clone()));
                current.push(Operations::Number(numbers[j + 1]));
            }

            current
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<(u64, Vec<Vec<Operations>>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(":");
            let expected_result: u64 = parts
                .clone()
                .take(1)
                .map(|s| s.parse().unwrap())
                .next()
                .unwrap();
            let numbers: Vec<u64> = parts
                .clone()
                .skip(1)
                .map(|part| part.split_whitespace().map(|s| s.parse().unwrap()))
                .flatten()
                .collect();

            let operations = generate_operations(&numbers);

            (expected_result, operations)
        })
        .collect()
}

fn parse_input_part2(input: &str) -> Vec<(u64, Vec<Vec<Operations>>)> {
    // Create the progress bar
    let progress_bar = ProgressBar::new(input.lines().count() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/red}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let res = input
        .par_lines()
        .map(|line| {
            let parts = line.split(":");
            let expected_result: u64 = parts
                .clone()
                .take(1)
                .map(|s| s.parse().unwrap())
                .next()
                .unwrap();
            let numbers: Vec<u64> = parts
                .clone()
                .skip(1)
                .map(|part| part.split_whitespace().map(|s| s.parse().unwrap()))
                .flatten()
                .collect();

            let operations = generate_operations_with_concat(&numbers);

            progress_bar.inc(1);

            (expected_result, operations)
        })
        .collect();

    progress_bar.finish_and_clear();

    res
}

fn operate(operations: Vec<Operations>) -> u64 {
    let mut current: Vec<Operations> = operations.clone();
    let mut stack_number = vec![];
    let mut stack_operator = vec![];
    while !current.is_empty() {
        let op = current.remove(0);
        match op {
            Operations::Number(n) => {
                stack_number.push(n);

                if stack_operator.len() == 1 && stack_number.len() == 2 {
                    let operator = stack_operator.pop().unwrap();
                    match operator {
                        Operations::Operator(Operator::Add) => {
                            let a = stack_number.pop().unwrap();
                            let b = stack_number.pop().unwrap();
                            stack_number.push(a + b);
                        }
                        Operations::Operator(Operator::Multiply) => {
                            let a = stack_number.pop().unwrap();
                            let b = stack_number.pop().unwrap();
                            stack_number.push(a * b);
                        }
                        Operations::Operator(Operator::Concatenate) => {
                            let a = stack_number.pop().unwrap();
                            let b = stack_number.pop().unwrap();
                            stack_number.push(format!("{}{}", b, a).parse::<u64>().unwrap());
                        }
                        _ => {}
                    }
                }
            }
            Operations::Operator(o) => {
                stack_operator.push(Operations::Operator(o));
            }
        }
    }
    stack_number.pop().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = parse_input(input);

    let result: u64 = parsed
        .par_iter()
        .map(|(expected_result, operations)| {
            operations
                .par_iter()
                .find_first(|operation| {
                    let current: Vec<Operations> = operation.to_vec().clone();
                    let res = operate(current);
                    res == *expected_result
                })
                .map_or(0 as u64, |_| *expected_result)
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    info!("Parsing input for part two");
    let parsed = parse_input_part2(input);

    info!("Starting part two");

    // Create the progress bar
    let progress_bar = ProgressBar::new(parsed.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/purple}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let result: u64 = parsed
        .par_iter()
        .map(|(expected_result, operations)| {
            let res = operations
                .par_iter()
                .find_first(|operation| {
                    let current: Vec<Operations> = operation.to_vec().clone();
                    let res = operate(current.clone());
                    res == *expected_result
                })
                .map_or(0 as u64, |_| *expected_result);

            progress_bar.inc(1);

            res
        })
        .sum();

    progress_bar.finish_and_clear();

    Some(result)
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
        assert_eq!(result, Some(3749));
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
        assert_eq!(result, Some(11387));
    }
}
