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

use std::ops::RangeInclusive;
use utils::input_read;

#[derive(Debug, Clone)]
struct Category {
    name: String,
    content: Option<usize>,
    // each example and input file consists of 2 ranges. If at some point it changes to some
    // dynamic value, the ranges should instead be put into a vec
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

fn range_from_raw(raw: &str) -> RangeInclusive<usize> {
    let bounds: Vec<_> = raw
        .split('-')
        .map(|bound| bound.parse().expect("failed to parse range bound!"))
        .collect();
    debug_assert_eq!(2, bounds.len());

    RangeInclusive::new(bounds[0], bounds[1])
}

impl From<&str> for Category {
    fn from(raw: &str) -> Self {
        let name_ranges: Vec<_> = raw.split(": ").collect();
        debug_assert_eq!(2, name_ranges.len());

        let ranges: Vec<_> = name_ranges[1].split(" or ").collect();
        debug_assert_eq!(2, ranges.len());

        Category {
            name: name_ranges[0].to_string(),
            content: None,
            range1: range_from_raw(ranges[0]),
            range2: range_from_raw(ranges[1]),
        }
    }
}

impl Category {
    fn is_valid_value(&self, value: usize) -> bool {
        if self.content.is_some() {
            // category is already full
            return false;
        }
        self.range1.contains(&value) || self.range2.contains(&value)
    }
}

#[derive(Debug)]
struct Ticket {
    values: Vec<usize>,
}

impl From<&str> for Ticket {
    fn from(raw: &str) -> Self {
        Ticket {
            values: raw
                .split(',')
                .map(|raw| raw.parse().expect("failed to parse ticket value"))
                .collect(),
        }
    }
}

impl Ticket {
    // get list of values that do not fit into any category
    fn get_invalid_values(&self, categories: &[Category]) -> Vec<usize> {
        let mut invalid = Vec::new();
        for value in &self.values {
            if !categories.iter().any(|cat| cat.is_valid_value(*value)) {
                invalid.push(*value)
            }
        }

        invalid
    }
}

fn parse_into_categories(raw: &str) -> Vec<Category> {
    raw.split('\n').map(Category::from).collect()
}

fn parse_into_tickets(raw: &str) -> Vec<Ticket> {
    // we skip "your ticket:" and "nearby tickets" strings
    raw.split('\n').skip(1).map(Ticket::from).collect()
}

fn part1(input: &[String]) -> usize {
    let categories = parse_into_categories(&input[0]);
    // for part1 we ignore our ticket
    // let our_ticket = parse_into_tickets(&input[1]).pop().unwrap();
    let nearby_tickets = parse_into_tickets(&input[2]);

    let mut sum = 0;
    for ticket in nearby_tickets {
        let invalid_ticket_sum: usize = ticket.get_invalid_values(&categories).into_iter().sum();
        sum += invalid_ticket_sum;
    }

    sum
}

// fn part2(input: &[String]) -> usize {
//     0
// }

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_into_string_groups("input").expect("failed to read input file");
    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);
    //
    // let part2_result = part2(&input);
    // println!("Part 2 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50"
                .to_string(),
            "your ticket:
7,1,14"
                .to_string(),
            "nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
                .to_string(),
        ];

        let expected = 71;

        assert_eq!(expected, part1(&input))
    }
}
