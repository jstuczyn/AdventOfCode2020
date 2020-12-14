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

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use utils::input_read;

const UP_BIT: char = '1';
const DOWN_BIT: char = '0';
const NO_MASK: char = 'X';

struct Mask(HashMap<usize, usize>);

impl From<&String> for Mask {
    fn from(raw_mask: &String) -> Self {
        let mask = raw_mask
            .trim_start_matches("mask = ")
            .chars()
            .rev()
            .enumerate();
        let mut mask_map = HashMap::new();
        for (i, c) in mask {
            match c {
                UP_BIT => mask_map.insert(i, 1),
                DOWN_BIT => mask_map.insert(i, 0),
                NO_MASK => continue,
                _ => panic!("unexpected mask character - {}", c),
            };
        }

        Mask(mask_map)
    }
}

struct MemoryValue {
    address: usize,
    value: usize,
}

impl From<&String> for MemoryValue {
    fn from(raw: &String) -> Self {
        let without_prefix = raw.trim_start_matches("mem[");
        let address_value: Vec<_> = without_prefix.split("] = ").collect();
        Self {
            address: address_value[0]
                .parse()
                .expect("failed to parse memory address"),
            value: address_value[1]
                .parse()
                .expect("failed to parse memory value"),
        }
    }
}

impl Display for MemoryValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "mem[{}] = {}", self.address, self.value)
    }
}

impl MemoryValue {
    fn apply_bitmask(&mut self, mask: &Mask) {
        for (i, bit) in mask.0.iter() {
            self.value = (self.value & !(1 << i)) | (bit << i);
        }
    }
}

// part1 uses mask bits to substitute the memory values
fn part1(input: &[String]) -> usize {
    let mut current_mask = None;
    let mut memory = HashMap::new();
    // first entry MUST BE a mask
    for raw in input {
        if raw.starts_with("mask") {
            current_mask = Some(Mask::from(raw))
        } else {
            let mut mem_value = MemoryValue::from(raw);
            mem_value.apply_bitmask(current_mask.as_ref().expect("no mask was set!"));
            memory.insert(mem_value.address, mem_value.value);
        }
    }

    memory.values().sum()
}

// fn part2(input: &[String]) -> usize {
//     0
// }

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_line_input("input").expect("failed to read input file");

    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];

        let expected = 165;

        assert_eq!(expected, part1(&input))
    }
}
