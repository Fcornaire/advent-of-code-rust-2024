use tracing::info;

advent_of_code::solution!(9);

fn parse_input(input: &str) -> String {
    let mut id = 0;
    input
        .lines()
        .into_iter()
        .map(|(line)| {
            let mut parsed = String::new();
            line.char_indices().for_each(|(i, c)| {
                let is_file_bloc = i % 2 == 0;

                let parsed_char: usize = c.to_string().parse().unwrap();
                if is_file_bloc {
                    parsed.push_str(&id.to_string().repeat(parsed_char));
                    id += 1;
                } else {
                    parsed.push_str(&'.'.to_string().repeat(parsed_char));
                }
            });

            parsed
        })
        .collect()
}

fn swap_chars(input: &mut str, index1: usize, index2: usize) {
    let bytes = unsafe { input.as_bytes_mut() };
    bytes.swap(index1, index2);
}

fn move_file_bloc_to_free_space_as_possible(input: &mut str) {
    let mut point_index = input.find('.').unwrap();

    let mut last_index = input.rfind(|c| c != '.').unwrap();
    while point_index < last_index {
        swap_chars(input, point_index, last_index);
        point_index = input.find('.').unwrap();
        last_index = input.rfind(|c| c != '.').unwrap();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parsed = parse_input(input);
    let clone = parsed.clone();
    info!("Parsed: {}", parsed);
    move_file_bloc_to_free_space_as_possible(&mut parsed);
    info!("Moved: {}", parsed);

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
        assert_eq!(result, Some(1928));
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
