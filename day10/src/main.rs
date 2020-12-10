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

use itertools::Itertools;
use std::collections::HashMap;
use utils::input_read;

fn part1(input: &[usize]) -> usize {
    // for larger input it might have been more efficient to temporarily store it in a HashSet,
    // but considering the input has less than 100 values, the performance penalty of cloning and
    // sorting the input slice is negligible

    // adding value of 0 indicating the jolt value of the outlet
    let mut adapters = input
        .iter()
        .copied()
        .chain(std::iter::once(0))
        .collect_vec();
    adapters.sort_unstable();

    let mut one_jolt_diffs = 0;
    let mut three_jolt_diffs = 1; // there's the final 3 jolt difference to the device
    adapters.iter().tuple_windows().for_each(|(a, b)| {
        if *b == a + 1 {
            one_jolt_diffs += 1;
        }

        if *b == a + 3 {
            three_jolt_diffs += 1
        }
    });

    one_jolt_diffs * three_jolt_diffs
}

#[inline]
fn path_cost(
    node: usize,
    connection_map: &HashMap<usize, Vec<usize>>,
    cost_map: &mut HashMap<usize, usize>,
) -> usize {
    // if we have value in cache - return it
    if let Some(cost) = cost_map.get(&node) {
        return *cost;
    }
    // if it has any children, it's the sum of the cost of the children
    if let Some(children) = connection_map.get(&node) {
        let cost = children
            .iter()
            .map(|child| path_cost(*child, connection_map, cost_map))
            .sum();
        cost_map.insert(node, cost);
        cost
    } else {
        1
    }
}

fn part2(input: &[usize]) -> usize {
    // note that device always only has a single valid parent, i.e. highest adapter
    let mut valid_adapters = input
        .iter()
        .copied()
        .chain(std::iter::once(0))
        .collect_vec();
    valid_adapters.sort_unstable();

    let mut connection_map = HashMap::with_capacity(valid_adapters.len() - 1);

    // for each adapter
    'outer: for (i, adapter) in valid_adapters.iter().enumerate() {
        // find it's children, i.e. other adapters that can be connected to it
        for potential_child in &valid_adapters[i + 1..] {
            if *adapter + 3 >= *potential_child {
                let children = connection_map.entry(*adapter).or_insert_with(Vec::new);
                children.push(*potential_child);
            } else {
                // vec is sorted so we won't find anything there
                continue 'outer;
            }
        }
    }

    let mut cost_map = HashMap::with_capacity(connection_map.len());

    path_cost(0, &connection_map, &mut cost_map)
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
    fn part1_sample_input1() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let expected = 35;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part1_sample_input2() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        let expected = 220;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input1() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let expected = 8;

        assert_eq!(expected, part2(&input))
    }

    #[test]
    fn part2_sample_input2() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        let expected = 19208;

        assert_eq!(expected, part2(&input))
    }
}
