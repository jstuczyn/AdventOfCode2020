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
use std::convert::TryInto;
use utils::input_read;

const ONE_BIT: char = '1';
const ZERO_BIT: char = '0';
const FLOATING_BIT: char = 'X';

#[derive(Debug, Copy, Clone)]
enum MaskBit {
    One,
    Zero,
    Floating,
}

impl MaskBit {
    fn is_floating(&self) -> bool {
        matches!(self, MaskBit::Floating)
    }
}

impl From<MaskBit> for usize {
    fn from(mask: MaskBit) -> Self {
        match mask {
            MaskBit::One => 1,
            MaskBit::Zero => 0,
            MaskBit::Floating => panic!("tried to convert 'floating' mask into usize!"),
        }
    }
}

struct Mask([MaskBit; 36]);

impl From<&String> for Mask {
    fn from(raw_mask: &String) -> Self {
        let mask = raw_mask.trim_start_matches("mask = ").chars().rev();

        // since TryFrom<Vec<T>> for [T; N] was stabilised, we can just use that to ensure correct length
        let mut mask_bits = Vec::with_capacity(36);

        for c in mask {
            match c {
                ONE_BIT => mask_bits.push(MaskBit::One),
                ZERO_BIT => mask_bits.push(MaskBit::Zero),
                FLOATING_BIT => mask_bits.push(MaskBit::Floating),
                _ => panic!("unexpected mask character - {}", c),
            };
        }

        Mask(mask_bits.try_into().expect("failed to parse mask"))
    }
}

type MemoryAddress = usize;
type MemoryValue = usize;

#[derive(Debug)]
struct Memory(HashMap<MemoryAddress, MemoryValue>);

impl Memory {
    fn new() -> Self {
        Memory(Default::default())
    }

    fn write_with_value_mask(&mut self, address: MemoryAddress, value: MemoryValue, mask: &Mask) {
        let mut init = value;
        for (i, &bit) in mask.0.iter().enumerate() {
            if !bit.is_floating() {
                init = (init & !(1 << i)) | (Into::<usize>::into(bit) << i);
            }
        }
        self.0.insert(address, init);
    }

    fn write_with_address_mask(&mut self, address: MemoryAddress, value: MemoryValue, mask: &Mask) {
        let mut target_addresses = vec![address];

        for (i, &bit) in mask.0.iter().enumerate() {
            match bit {
                // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
                MaskBit::Zero => (),
                // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
                MaskBit::One => {
                    for address in target_addresses.iter_mut() {
                        *address |= 1 << i;
                    }
                }
                // If the bitmask bit is X, the corresponding memory address bit is floating.
                // Floating bits will take on all possible values.
                MaskBit::Floating => {
                    // set existing ones to 1, and push the 0 variants
                    let mut new = Vec::with_capacity(target_addresses.len());
                    for address in target_addresses.iter_mut() {
                        new.push(*address & !(1 << i));
                        *address |= 1 << i;
                    }
                    target_addresses.append(&mut new);
                }
            }
        }

        for address in target_addresses {
            self.0.insert(address, value);
        }
    }
}

fn parse_into_address_and_value(raw: &str) -> (MemoryAddress, MemoryValue) {
    let without_prefix = raw.trim_start_matches("mem[");
    let address_value: Vec<_> = without_prefix.split("] = ").collect();
    (
        address_value[0]
            .parse()
            .expect("failed to parse memory address"),
        address_value[1]
            .parse()
            .expect("failed to parse memory value"),
    )
}

fn part1(input: &[String]) -> usize {
    let mut current_mask = None;
    let mut memory = Memory::new();
    // first entry MUST BE a mask
    for raw in input {
        if raw.starts_with("mask") {
            current_mask = Some(Mask::from(raw))
        } else {
            let (address, value) = parse_into_address_and_value(raw);
            memory.write_with_value_mask(
                address,
                value,
                current_mask.as_ref().expect("no mask was set!"),
            );
        }
    }

    memory.0.values().sum()
}

fn part2(input: &[String]) -> usize {
    let mut current_mask = None;
    let mut memory = Memory::new();
    // first entry MUST BE a mask
    for raw in input {
        if raw.starts_with("mask") {
            current_mask = Some(Mask::from(raw))
        } else {
            let (address, value) = parse_into_address_and_value(raw);
            memory.write_with_address_mask(
                address,
                value,
                current_mask.as_ref().expect("no mask was set!"),
            );
        }
    }

    memory.0.values().sum()
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
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];

        let expected = 165;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "mask = 000000000000000000000000000000X1001X".to_string(),
            "mem[42] = 100".to_string(),
            "mask = 00000000000000000000000000000000X0XX".to_string(),
            "mem[26] = 1".to_string(),
        ];

        let expected = 208;

        assert_eq!(expected, part2(&input))
    }
}
