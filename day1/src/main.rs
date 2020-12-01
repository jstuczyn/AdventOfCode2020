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

mod input_parser;

fn part1(input: &[usize]) -> Option<usize> {
    // if you really want to be fancy about it, you could sort the whole thing first,
    // then be smart about choosing second value, like if v1 + v2 > 2020, don't bother
    // checking anything above v2. But current approach works well enough
    // and cutting edge performance is not a requirement.

    for pair in input.iter().tuple_combinations::<(_, _)>() {
        if pair.0 + pair.1 == 2020 {
            return Some(pair.0 * pair.1);
        }
    }

    None
}

fn part2(input: &[usize]) -> Option<usize> {
    for triplet in input.iter().tuple_combinations::<(_, _, _)>() {
        if triplet.0 + triplet.1 + triplet.2 == 2020 {
            return Some(triplet.0 * triplet.1 * triplet.2);
        }
    }

    None
}

fn main() {
    let input = input_parser::read_input_file("input").expect("failed to read input file");
    let part1_result = part1(&input).expect("failed to solve part1");
    println!("Part 1 result is {}", part1_result);

    let part2_result = part2(&input).expect("failed to solve part2");
    println!("Part 1 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let expected = 514579;

        assert_eq!(expected, part1(&input).unwrap())
    }
}
