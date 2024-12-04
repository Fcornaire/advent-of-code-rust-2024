use tracing::info;

advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut xmas_word_count = 0;
    grid.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, char)| {
            //Horizontal 4 word XMAS check
            if x <= line.len() - 4
                && char == &'X'
                && grid[x + 1][y] == 'M'
                && grid[x + 2][y] == 'A'
                && grid[x + 3][y] == 'S'
            {
                xmas_word_count += 1;
            }

            //Horizontal 4 word XMAS check backwards
            if x >= 3
                && char == &'X'
                && grid[x - 1][y] == 'M'
                && grid[x - 2][y] == 'A'
                && grid[x - 3][y] == 'S'
            {
                xmas_word_count += 1;
            }

            //Vertical 4 word XMAS check upwards
            if y >= 3
                && char == &'X'
                && grid[x][y - 1] == 'M'
                && grid[x][y - 2] == 'A'
                && grid[x][y - 3] == 'S'
            {
                xmas_word_count += 1;
            }

            //Vertical 4 word XMAS check downwards
            if y <= line.len() - 4
                && char == &'X'
                && grid[x][y + 1] == 'M'
                && grid[x][y + 2] == 'A'
                && grid[x][y + 3] == 'S'
            {
                xmas_word_count += 1;
            }

            //Diagonal 4 word XMAS check upwards right
            if x <= line.len() - 4
                && y >= 3
                && char == &'X'
                && grid[x + 1][y - 1] == 'M'
                && grid[x + 2][y - 2] == 'A'
                && grid[x + 3][y - 3] == 'S'
            {
                xmas_word_count += 1;
            }

            //Diagonal 4 word XMAS check upwards left
            if x >= 3
                && y >= 3
                && char == &'X'
                && grid[x - 1][y - 1] == 'M'
                && grid[x - 2][y - 2] == 'A'
                && grid[x - 3][y - 3] == 'S'
            {
                xmas_word_count += 1;
            }

            //Diagonal 4 word XMAS check downwards right
            if x <= line.len() - 4
                && y <= grid.len() - 4
                && char == &'X'
                && grid[x + 1][y + 1] == 'M'
                && grid[x + 2][y + 2] == 'A'
                && grid[x + 3][y + 3] == 'S'
            {
                xmas_word_count += 1;
            }

            //Diagonal 4 word XMAS check downwards left
            if x >= 3
                && y <= grid.len() - 4
                && char == &'X'
                && grid[x - 1][y + 1] == 'M'
                && grid[x - 2][y + 2] == 'A'
                && grid[x - 3][y + 3] == 'S'
            {
                xmas_word_count += 1;
            }
        });
    });

    info!("XMAS word count: {}", xmas_word_count);

    Some(xmas_word_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut mas_word_count = 0;
    grid.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, char)| {
            if char == &'A' && x >= 1 && x < line.len() - 1 && y >= 1 && y < grid.len() - 1 {
                if ((grid[x - 1][y - 1] == 'M' && grid[x + 1][y + 1] == 'S')
                    || (grid[x - 1][y - 1] == 'S' && grid[x + 1][y + 1] == 'M'))
                    && (grid[x + 1][y - 1] == 'S' && grid[x - 1][y + 1] == 'M'
                        || grid[x + 1][y - 1] == 'M' && grid[x - 1][y + 1] == 'S')
                {
                    mas_word_count += 1;
                }

                if ((grid[x - 1][y - 1] == 'S' && grid[x + 1][y + 1] == 'M')
                    || (grid[x - 1][y - 1] == 'M' && grid[x + 1][y + 1] == 'S'))
                    && ((grid[x + 1][y - 1] == 'M' && grid[x - 1][y + 1] == 'S')
                        || (grid[x + 1][y - 1] == 'S' && grid[x - 1][y + 1] == 'M'))
                {
                    mas_word_count += 1;
                }
            }
        });
    });

    info!("X-MAS word count: {}", mas_word_count);

    Some(mas_word_count / 2)
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
        assert_eq!(result, Some(18));
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
        assert_eq!(result, Some(9));
    }
}
