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
use utils::input_read;

struct Group(HashSet<char>);

impl<'a> From<&'a String> for Group {
    fn from(answers: &'a String) -> Self {
        let mut unique_answers = HashSet::new();
        answers
            .split_ascii_whitespace()
            .flat_map(|answer| answer.chars())
            .for_each(|char| {
                unique_answers.insert(char);
            });
        Group(unique_answers)
    }
}

impl Group {
    fn len(&self) -> usize {
        self.0.len()
    }
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|group_answers| Group::from(group_answers).len())
        .sum()
}

fn part2(input: &[String]) -> Option<usize> {
    None
}

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_into_string_groups("input").expect("failed to read input file");
    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2 result is {}", part2_result);
}
