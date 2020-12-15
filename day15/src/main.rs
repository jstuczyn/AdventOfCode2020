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
use std::vec;

struct VanEckSequence {
    initial_sequence: Vec<<Self as IntoIterator>::Item>,
}

impl VanEckSequence {
    fn new(initial_sequence: Vec<<Self as IntoIterator>::Item>) -> Self {
        VanEckSequence { initial_sequence }
    }
}

impl IntoIterator for VanEckSequence {
    type Item = usize;
    type IntoIter = VanEckSequenceIterator;

    fn into_iter(self) -> Self::IntoIter {
        let mut initial_sequence = self.initial_sequence.into_iter();

        let first = initial_sequence
            .next()
            .expect("initial sequence was empty!");

        VanEckSequenceIterator {
            initial_sequence,
            last_seen: Default::default(),
            current_epoch: 0,
            current_value: first,
        }
    }
}

struct VanEckSequenceIterator {
    initial_sequence: vec::IntoIter<<Self as Iterator>::Item>,

    // map between number and the epoch when it was last seen
    last_seen: HashMap<usize, usize>,
    current_value: usize,
    current_epoch: usize,
}

impl Iterator for VanEckSequenceIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current_value;

        // firstly, consume initial sequence
        if let Some(initial) = self.initial_sequence.next() {
            self.current_value = initial
        } else {
            // If that was the first time the number has been spoken, the current player says 0.
            // Otherwise, the number had been spoken before;
            // the current player announces how many turns apart the number is from when it was previously spoken.
            if let Some(last_seen) = self.last_seen.get(&self.current_value) {
                self.current_value = self.current_epoch - last_seen;
            } else {
                self.current_value = 0;
            }
        }

        self.last_seen.insert(next, self.current_epoch);
        self.current_epoch += 1;

        Some(next)
    }
}

fn part1(input: &[usize]) -> usize {
    VanEckSequence::new(input.to_vec())
        .into_iter()
        .nth(2019)
        .unwrap()
}

// fn part2(input: &[usize]) -> usize {
//     0
// }

#[cfg(not(tarpaulin))]
fn main() {
    let input = vec![18, 8, 0, 5, 4, 1, 20];

    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);
    //
    //     let part2_result = part2(&input);
    //     println!("Part 2 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_sequence() {
        let init = vec![0, 3, 6];

        let expected_10 = vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0];

        assert_eq!(
            expected_10,
            VanEckSequence::new(init)
                .into_iter()
                .take(10)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn part1_sample_input1() {
        let input = vec![1, 3, 2];

        let expected = 1;

        assert_eq!(expected, part1(&input));
    }

    #[test]
    fn part1_sample_input2() {
        let input = vec![2, 1, 3];

        let expected = 10;

        assert_eq!(expected, part1(&input));
    }

    #[test]
    fn part1_sample_input3() {
        let input = vec![1, 2, 3];

        let expected = 27;

        assert_eq!(expected, part1(&input));
    }

    #[test]
    fn part1_sample_input4() {
        let input = vec![2, 3, 1];

        let expected = 78;

        assert_eq!(expected, part1(&input));
    }

    #[test]
    fn part1_sample_input5() {
        let input = vec![3, 2, 1];

        let expected = 438;

        assert_eq!(expected, part1(&input));
    }

    #[test]
    fn part1_sample_input6() {
        let input = vec![3, 1, 2];

        let expected = 1836;

        assert_eq!(expected, part1(&input));
    }
}
