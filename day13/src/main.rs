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

use utils::input_read;

struct Bus {
    id: usize,
}

impl Bus {
    fn new(raw_id: &str) -> Option<Self> {
        match raw_id.parse() {
            Ok(id) => Some(Bus { id }),
            _ => None,
        }
    }

    fn earliest_departure_from(&self, timestamp: usize) -> usize {
        // assume id < timestamp
        let quo = timestamp / self.id;
        let rem = timestamp % self.id;

        let mut n = quo;
        if rem != 0 {
            n += 1;
        }

        self.id * n
    }
}

fn split_into_timestamp_and_buses(input: &str) -> (usize, Vec<Option<Bus>>) {
    let split: Vec<_> = input.split_ascii_whitespace().collect();
    assert_eq!(2, split.len(), "invalid input");

    let timestamp = split[0].parse().expect("failed to parse timestamp");
    let buses = split[1].split(",").map(Bus::new).collect();

    (timestamp, buses)
}

fn part1(input: &str) -> usize {
    let (timestamp, buses) = split_into_timestamp_and_buses(input);
    let (id, departure) = buses
        .into_iter()
        .filter_map(|bus| bus)
        .map(|bus| (bus.id, bus.earliest_departure_from(timestamp)))
        .min_by(|(_, timestamp1), (_, timestamp2)| timestamp1.cmp(timestamp2))
        .unwrap();
    id * (departure - timestamp)
}

// fn part2(input: &[String]) -> usize {
//     0
// }

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_to_string("input").expect("failed to read input file");

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
        let input = r#"939
7,13,x,x,59,x,31,19"#;

        let expected = 295;

        assert_eq!(expected, part1(&input));
    }
}
