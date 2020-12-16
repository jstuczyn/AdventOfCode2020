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
use std::ops::RangeInclusive;
use utils::input_read;

#[derive(Debug, Clone)]
struct Category {
    name: String,
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
            range1: range_from_raw(ranges[0]),
            range2: range_from_raw(ranges[1]),
        }
    }
}

impl Category {
    fn is_valid_value(&self, value: usize) -> bool {
        self.range1.contains(&value) || self.range2.contains(&value)
    }
}

#[derive(Debug, Clone)]
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

    fn is_valid(&self, categories: &[Category]) -> bool {
        for value in &self.values {
            if !categories.iter().any(|cat| cat.is_valid_value(*value)) {
                return false;
            }
        }
        true
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

// see if there are any categories such that they can only accept a single set of values
// it takes categories by value and returns a new vec rather than do everything by reference
// to decrease the search space and increase performance by that sweet 10% bringing it all
// [on my machine] below 1ms
fn try_get_fixed_categories(
    category_values: &mut HashMap<usize, Vec<usize>>,
    categories: Vec<Category>,
) -> (HashMap<usize, Category>, Vec<Category>) {
    let mut matched = HashMap::new();
    let mut remaining = Vec::new();
    for category in categories.into_iter() {
        let mut is_unique = true;
        let mut good_index = usize::max_value();
        for (category_idx, values) in category_values.iter() {
            if values.iter().all(|val| category.is_valid_value(*val)) {
                if good_index != usize::max_value() {
                    is_unique = false;
                    break;
                } else {
                    good_index = *category_idx
                }
            }
        }
        if is_unique && good_index != usize::max_value() {
            matched.insert(good_index, category);
        } else {
            remaining.push(category);
        }
    }

    for idx in matched.keys() {
        category_values.remove(idx);
    }

    (matched, remaining)
}

fn part2(input: &[String]) -> usize {
    let mut categories = parse_into_categories(&input[0]);
    let our_ticket = parse_into_tickets(&input[1]).pop().unwrap();
    let nearby_valid_tickets: Vec<_> = parse_into_tickets(&input[2])
        .into_iter()
        .filter(|ticket| ticket.is_valid(&categories))
        .collect();

    let num_tickets = nearby_valid_tickets.len() + 1;

    let mut category_values = HashMap::with_capacity(categories.len());

    for ticket in nearby_valid_tickets
        .into_iter()
        .chain(std::iter::once(our_ticket.clone()))
    {
        for (i, value) in ticket.values.iter().enumerate() {
            let valid_values = category_values
                .entry(i)
                .or_insert_with(|| Vec::with_capacity(num_tickets));
            valid_values.push(*value);
        }
    }

    let mut category_map = HashMap::new();
    loop {
        let (matched, remaining_categories) =
            try_get_fixed_categories(&mut category_values, categories);
        categories = remaining_categories;

        if matched.is_empty() {
            break;
        } else {
            category_map.extend(matched);
        }
    }

    if !categories.is_empty() {
        // according to some smarter person, it is provable that the input must be cheese-able
        // since there's a unique solution
        panic!("our input was not cheese-able : (")
    }

    let mut final_product = 1;
    for (idx, category) in category_map {
        if category.name.starts_with("departure") {
            final_product *= our_ticket.values[idx]
        }
    }

    final_product
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

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "departure time: 0-1 or 4-19
departure station: 0-5 or 8-19
seat: 0-13 or 16-19"
                .to_string(),
            "your ticket:
11,12,13"
                .to_string(),
            "nearby tickets:
3,9,18
15,1,5
5,14,9"
                .to_string(),
        ];

        let expected = 132;

        assert_eq!(expected, part2(&input))
    }
}
