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
use std::fmt::{self, Debug, Formatter};
use utils::input_read;

const ACC_OPCODE: &str = "acc";
const JMP_OPCODE: &str = "jmp";
const NOP_OPCODE: &str = "nop";

#[derive(Clone, Eq, PartialEq)]
enum Opcode {
    Acc,
    Jump,
    Nop,
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Acc => write!(f, "{}", ACC_OPCODE),
            Opcode::Jump => write!(f, "{}", JMP_OPCODE),
            Opcode::Nop => write!(f, "{}", NOP_OPCODE),
        }
    }
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

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.opcode, self.operand)
    }
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

enum ExecutionError {
    LoopDetected(isize),
}

impl ExecutionError {
    fn into_inner(self) -> isize {
        match self {
            ExecutionError::LoopDetected(final_accumulator) => final_accumulator,
        }
    }
}

struct Computer {
    // currently there's only a single register, so until requirements change, keep it like this
    accumulator: isize,
    instruction_pointer: usize,
    // used to look for cycles
    executed_instructions: HashSet<usize>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            accumulator: 0,
            instruction_pointer: 0,
            executed_instructions: Default::default(),
        }
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.instruction_pointer = 0;
        self.executed_instructions = HashSet::new();
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
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

    fn execute_program(&mut self, program: &[Instruction]) -> Result<isize, ExecutionError> {
        loop {
            if self
                .executed_instructions
                .contains(&self.instruction_pointer)
            {
                // we run into an execution loop
                return Err(ExecutionError::LoopDetected(self.accumulator));
            }

            // we have terminated
            if self.instruction_pointer == program.len() {
                return Ok(self.accumulator);
            }

            self.executed_instructions.insert(self.instruction_pointer);
            let next_instruction = &program[self.instruction_pointer];
            self.execute_instruction(next_instruction);
        }
    }

    fn determine_non_halting_set(&mut self, program: &[Instruction]) -> HashSet<usize> {
        if self.execute_program(program).is_ok() {
            panic!("program did not loop!")
        }
        let non_halting_set = self.executed_instructions.clone();
        self.reset();
        non_halting_set
    }
}

fn parse_as_instructions(raw: &[String]) -> Vec<Instruction> {
    raw.iter()
        .map(|raw_instruction| {
            Instruction::try_from(raw_instruction).expect("failed to parse instruction")
        })
        .collect()
}

fn part1(input: &[String]) -> isize {
    let instructions = parse_as_instructions(input);
    match Computer::new().execute_program(&instructions) {
        Err(err) => err.into_inner(),
        Ok(_) => panic!("managed to terminate in part1!"),
    }
}

fn substitute_instruction(instruction: &mut Instruction) -> bool {
    match instruction.opcode {
        Opcode::Nop => {
            instruction.opcode = Opcode::Jump;
            true
        }
        Opcode::Jump => {
            instruction.opcode = Opcode::Nop;
            true
        }
        _ => false,
    }
}

fn part2(input: &[String]) -> Option<isize> {
    let mut instructions = parse_as_instructions(input);
    let mut computer = Computer::new();
    // there is no point in trying to substitute instructions that were never executed in the
    // looping case
    let non_halting_set = computer.determine_non_halting_set(&instructions);
    for non_halting in non_halting_set.iter() {
        // attempt substitution of any non-halting instructions in the program
        // but don't run the program if we run into an 'acc'
        if substitute_instruction(&mut instructions[*non_halting]) {
            if let Ok(termination_result) = computer.execute_program(&instructions) {
                return Some(termination_result);
            } else {
                computer.reset();
                substitute_instruction(&mut instructions[*non_halting]);
            }
        }
    }

    None
}

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_line_input("input").expect("failed to read input file");

    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);

    let part2_result = part2(&input).expect("failed to solve part2");
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

    #[test]
    fn part2_sample_input() {
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

        let expected = 8;

        assert_eq!(expected, part2(&input).unwrap())
    }

    #[test]
    fn instruction_formatting() {
        assert_eq!(
            "jmp 2",
            format!(
                "{:?}",
                Instruction {
                    opcode: Opcode::Jump,
                    operand: 2
                }
            )
        );

        assert_eq!(
            "nop -3",
            format!(
                "{:?}",
                Instruction {
                    opcode: Opcode::Nop,
                    operand: -3
                }
            )
        );

        assert_eq!(
            "acc 5",
            format!(
                "{:?}",
                Instruction {
                    opcode: Opcode::Acc,
                    operand: 5
                }
            )
        )
    }
}
