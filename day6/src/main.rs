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
use utils::input_read;

struct Group {
    size: usize,
    answers: HashMap<char, usize>,
}

impl<'a> From<&'a String> for Group {
    fn from(answers_raw: &'a String) -> Self {
        let mut answers = HashMap::new();

        let mut size = 0;

        answers_raw
            .split_ascii_whitespace()
            .flat_map(|answer| {
                size += 1;
                answer.chars()
            })
            .for_each(|char| {
                let count = answers.entry(char).or_insert(0);
                *count += 1;
            });

        Group { size, answers }
    }
}

impl Group {
    fn len(&self) -> usize {
        self.answers.len()
    }

    fn all_yes(&self) -> usize {
        self.answers
            .iter()
            .filter(|(_, &count)| count == self.size)
            .count()
    }
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|group_answers| Group::from(group_answers).len())
        .sum()
}

fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|group_answers| Group::from(group_answers).all_yes())
        .sum()
}

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_into_string_groups("input").expect("failed to read input file");
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
            "abc".to_string(),
            "a
b
c"
            .to_string(),
            "ab
ac"
            .to_string(),
            "a
a
a
a"
            .to_string(),
            "b".to_string(),
        ];

        let expected = 11;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "abc".to_string(),
            "a
b
c"
            .to_string(),
            "ab
ac"
            .to_string(),
            "a
a
a
a"
            .to_string(),
            "b".to_string(),
        ];

        let expected = 6;

        assert_eq!(expected, part2(&input))
    }
}
