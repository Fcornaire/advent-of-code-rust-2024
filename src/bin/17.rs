use std::collections::HashSet;

use regex::Regex;
use tracing::{debug, info, trace};

advent_of_code::solution!(17);

#[derive(Debug, Clone)]
enum Action {
    Print(u64),
    Nothing,
    Jump(u64),
}

#[derive(Debug, Clone)]
enum Instruction {
    Opcode(u64),
    Operand(u64),
}

impl Instruction {
    pub fn get_combo_operand(&self, computer: Computer) -> u64 {
        match self {
            Instruction::Opcode(v) => *v,
            Instruction::Operand(v) => match *v {
                4 => computer.register_a,
                5 => computer.register_b,
                6 => computer.register_c,
                7 => panic!("Invalid operand"),
                _ => *v,
            },
        }
    }

    pub fn get_literal_operand(&self) -> u64 {
        match self {
            Instruction::Opcode(v) => *v,
            Instruction::Operand(v) => *v,
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    pub register_a: u64,
    pub register_b: u64,
    pub register_c: u64,
    pub program: Vec<Instruction>,
}

impl Computer {
    fn run_instruction(&mut self, pointer: usize) -> Action {
        let opcode = self
            .program
            .get(pointer)
            .unwrap()
            .get_combo_operand(self.clone());

        match opcode {
            0 => {
                //DIV
                let operand = self
                    .program
                    .get(pointer + 1)
                    .unwrap()
                    .get_combo_operand(self.clone());
                self.register_a = self.register_a / 2u64.pow(operand as u32);
                Action::Nothing
            }
            1 => {
                // XOR
                let operand = self.program.get(pointer + 1).unwrap().get_literal_operand();
                self.register_b = self.register_b ^ operand;
                Action::Nothing
            }
            2 => {
                // mod
                let operand = self
                    .program
                    .get(pointer + 1)
                    .unwrap()
                    .get_combo_operand(self.clone());
                self.register_b = operand % 8;
                Action::Nothing
            }
            3 => {
                //Jump
                let operand = self.program.get(pointer + 1).unwrap().get_literal_operand();
                if self.register_a == 0 {
                    return Action::Nothing;
                }

                Action::Jump(operand)
            }
            4 => {
                //Print
                self.register_b = self.register_b ^ self.register_c;
                Action::Nothing
            }
            5 => {
                //Print
                let operand = self
                    .program
                    .get(pointer + 1)
                    .unwrap()
                    .get_combo_operand(self.clone());
                Action::Print(operand % 8)
            }
            6 => {
                //DIV
                let operand = self
                    .program
                    .get(pointer + 1)
                    .unwrap()
                    .get_combo_operand(self.clone());
                self.register_b = self.register_a / 2u64.pow(operand as u32);
                Action::Nothing
            }
            7 => {
                let operand = self
                    .program
                    .get(pointer + 1)
                    .unwrap()
                    .get_combo_operand(self.clone());
                self.register_c = self.register_a / 2u64.pow(operand as u32);
                Action::Nothing
            }
            _ => return Action::Nothing,
        }
    }

    pub fn run_program(&mut self) -> String {
        let initial_program = self.program.clone();
        let mut output = String::new();
        let mut pointer = 0;

        while pointer < initial_program.len() {
            let result = self.run_instruction(pointer);

            match result {
                Action::Print(v) => {
                    output.push_str(format!("{},", v).as_str());
                    pointer += 2;
                }
                Action::Jump(v) => {
                    pointer = v as usize;
                }
                Action::Nothing => {
                    pointer += 2;
                }
            }
        }

        output.pop();

        output
    }

    pub fn reverse_engine_program(&self) -> u64 {
        let program = self.program.clone();
        let mut possible_values = HashSet::new();
        possible_values.insert(0);

        for num in program.iter().rev() {
            let current = num.get_literal_operand();
            let mut new_possible_values = HashSet::new();
            for &curr in &possible_values {
                for i in 0..8 {
                    let new_value = (curr << 3) + i;
                    if my_program(new_value) == current.into() {
                        new_possible_values.insert(new_value);
                    }
                }
            }
            possible_values = new_possible_values;
        }

        *possible_values.iter().min().unwrap()
    }
}

fn parse_input(input: &str) -> Computer {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let re = Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)").unwrap();
    let caps = re.captures(parts[0]).unwrap();

    let register_a = caps[1].parse().unwrap();
    let register_b = caps[2].parse().unwrap();
    let register_c = caps[3].parse().unwrap();

    let prog = parts[1]
        .split_whitespace()
        .map(|s| s.replace(",", ""))
        .collect::<Vec<String>>();

    let program = prog[1]
        .char_indices()
        .map(|(i, c)| {
            if i % 2 == 0 {
                Instruction::Opcode(c.to_digit(10).unwrap() as u64)
            } else {
                Instruction::Operand(c.to_digit(10).unwrap() as u64)
            }
        })
        .collect();

    Computer {
        register_a,
        register_b,
        register_c,
        program,
    }
}

fn my_program(input: u64) -> u64 {
    let a = input;
    let mut b = a % 8;
    b = b ^ 5;
    let  c = a / 2u64.pow(b as u32);
    b = b ^ 6;
    // a = a / 2u64.pow(3);
    b = b ^ c;

    let res = b % 8;

    res
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = parse_input(input);

    let result = computer.run_program();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut computer = parse_input(input);

    let reversed = computer.reverse_engine_program();
    info!("reversed is: {}", reversed);
    computer.register_a = reversed as u64;
    let res = computer.run_program();

    info!("Res should be equal to computer program {:?}: {}", computer.program, res);

    Some(reversed)
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
        assert_eq!(result, Some("".to_string()));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("0,1,2".to_string()));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some("4,2,5,6,7,7,7,7,3,1,0".to_string()));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some("".to_string()));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some("".to_string()));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
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
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(105706277661082));
    }
}
