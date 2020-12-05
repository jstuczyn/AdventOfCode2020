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

use std::convert::TryFrom;

use utils::input_read;

struct MalformedPolicy;

#[derive(Debug)]
struct Policy {
    lower_bound: usize,
    upper_bound: usize,
    character: char,
}

impl TryFrom<String> for Policy {
    type Error = MalformedPolicy;

    fn try_from(mut raw_policy: String) -> Result<Self, Self::Error> {
        // final character is a `:` so we can discard it
        // if this fails, it means provided string was empty
        raw_policy.pop().ok_or(MalformedPolicy)?;

        // we are left with `lowerbound-upperbound character`
        let split: Vec<_> = raw_policy.split_whitespace().collect();
        if split.len() != 2 {
            return Err(MalformedPolicy);
        }

        let chars_raw: Vec<_> = split[1].chars().collect();
        if chars_raw.len() != 1 {
            return Err(MalformedPolicy);
        }

        let character = chars_raw.first().ok_or(MalformedPolicy)?.to_owned();

        let bound_split: Vec<_> = split[0].split('-').collect();
        if bound_split.len() != 2 {
            return Err(MalformedPolicy);
        }

        let lower_bound = bound_split[0].parse().map_err(|_| MalformedPolicy)?;
        let upper_bound = bound_split[1].parse().map_err(|_| MalformedPolicy)?;

        Ok(Policy {
            lower_bound,
            upper_bound,
            character,
        })
    }
}

impl Policy {
    fn verify_password_part1(&self, password: &Password) -> bool {
        let chars = password.chars();
        let count = chars.filter(|c| c == &self.character).count();
        count >= self.lower_bound && count <= self.upper_bound
    }

    fn verify_password_part2(&self, password: &Password) -> bool {
        let chars: Vec<_> = password.chars().collect();

        let first_char = chars[self.lower_bound - 1];
        let second_char = chars[self.upper_bound - 1];

        (first_char == self.character) ^ (second_char == self.character)
    }
}

type Password<'a> = &'a str;

// input is formatted as follows:
// `lowerbound-upperbound character: password`
// for example: `1-3 a: abcde`
// note that final space separates policy from password
fn parse_into_policy_password(input_line: &str) -> Option<(Policy, Password)> {
    let split: Vec<_> = input_line.split_whitespace().collect();

    if split.len() != 3 {
        return None;
    }

    // we know there will be 2 chunks in policy due to the described structure
    let policy_raw = vec![split[0], split[1]].join(" ");
    let password = split[2];
    let policy = Policy::try_from(policy_raw).ok()?;

    Some((policy, password))
}

fn part1(input: &[String]) -> Option<usize> {
    let mut valid_count = 0;
    for policy_password in input.iter().map(|input| parse_into_policy_password(input)) {
        let (policy, password) = policy_password?;
        if policy.verify_password_part1(&password) {
            valid_count += 1;
        }
    }
    Some(valid_count)
}

fn part2(input: &[String]) -> Option<usize> {
    let mut valid_count = 0;
    for policy_password in input.iter().map(|input| parse_into_policy_password(input)) {
        let (policy, password) = policy_password?;
        if policy.verify_password_part2(&password) {
            valid_count += 1;
        }
    }
    Some(valid_count)
}

fn main() {
    let input = input_read::read_line_input("input").expect("failed to read input file");
    let part1_result = part1(&input).expect("failed to solve part1");
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
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        let expected = 2;

        assert_eq!(expected, part1(&input).unwrap())
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        let expected = 1;

        assert_eq!(expected, part2(&input).unwrap())
    }
}
