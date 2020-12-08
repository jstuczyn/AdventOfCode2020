// Copyright 2020 Jedrzej Stuczynski
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashSet;
use std::convert::TryFrom;
use utils::input_read;

const ACC_OPCODE: &str = "acc";
const JMP_OPCODE: &str = "jmp";
const NOP_OPCODE: &str = "nop";

enum Opcode {
    Acc,
    Jump,
    Nop,
}

#[derive(Debug)]
struct InvalidOpcode(String);

impl<'a> TryFrom<&'a str> for Opcode {
    type Error = InvalidOpcode;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            ACC_OPCODE => Ok(Opcode::Acc),
            JMP_OPCODE => Ok(Opcode::Jump),
            NOP_OPCODE => Ok(Opcode::Nop),
            _ => Err(InvalidOpcode(value.to_owned())),
        }
    }
}

#[derive(Debug)]
enum InvalidInstruction {
    MalformedOperand(String),
    MalformedOpcode(InvalidOpcode),
    TooShortInstruction,
}

impl From<InvalidOpcode> for InvalidInstruction {
    fn from(err: InvalidOpcode) -> Self {
        InvalidInstruction::MalformedOpcode(err)
    }
}

struct Instruction {
    opcode: Opcode,
    // currently there's only a single operand, so until requirements change, keep it like this
    operand: isize,
}

impl<'a> TryFrom<&'a String> for Instruction {
    type Error = InvalidInstruction;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        let code_operand: Vec<_> = value.split_ascii_whitespace().collect();
        if code_operand.len() != 2 {
            return Err(InvalidInstruction::TooShortInstruction);
        }
        let opcode = Opcode::try_from(code_operand[0])?;

        let operand = code_operand[1].parse().map_err(|err| {
            InvalidInstruction::MalformedOperand(format!(
                "{} is malformed - {}",
                code_operand[1], err
            ))
        })?;

        Ok(Instruction { opcode, operand })
    }
}

struct Computer {
    // currently there's only a single register, so until requirements change, keep it like this
    accumulator: isize,
    instruction_pointer: usize,
    instructions: Vec<Instruction>,
    // used to look for cycles
    executed_instructions: HashSet<usize>,
}

impl Computer {
    fn new(instructions: Vec<Instruction>) -> Computer {
        Computer {
            accumulator: 0,
            instruction_pointer: 0,
            instructions,
            executed_instructions: Default::default(),
        }
    }

    fn execute_until_loop(&mut self) -> isize {
        loop {
            if self
                .executed_instructions
                .contains(&self.instruction_pointer)
            {
                return self.accumulator;
            }
            self.executed_instructions.insert(self.instruction_pointer);
            let instruction = &self.instructions[self.instruction_pointer];
            match instruction.opcode {
                Opcode::Nop => self.instruction_pointer += 1,
                Opcode::Acc => {
                    self.accumulator += instruction.operand;
                    self.instruction_pointer += 1;
                }
                Opcode::Jump => {
                    self.instruction_pointer =
                        (self.instruction_pointer as isize + instruction.operand) as usize
                }
            }
        }
    }
}

fn parse_as_instructions(raw: &[String]) -> Vec<Instruction> {
    raw.into_iter()
        .map(|raw_instruction| {
            Instruction::try_from(raw_instruction).expect("failed to parse instruction")
        })
        .collect()
}

fn part1(input: &[String]) -> isize {
    let instructions = parse_as_instructions(input);
    Computer::new(instructions).execute_until_loop()
}

fn part2(input: &[String]) -> isize {
    0
}

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_line_input("input").expect("failed to read input file");

    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        let expected = 5;

        assert_eq!(expected, part1(&input))
    }
}
