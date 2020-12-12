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

// Note: I've started using 'from' rather than 'try_from' as I'm making assumption that
// provided inputs must not be malformed.

use std::fmt::{self, Display, Formatter};
use utils::input_read;

const NORTH_DIRECTION: char = 'N';
const SOUTH_DIRECTION: char = 'S';
const EAST_DIRECTION: char = 'E';
const WEST_DIRECTION: char = 'W';
const LEFT_DIRECTION: char = 'L';
const RIGHT_DIRECTION: char = 'R';
const FORWARD_DIRECTION: char = 'F';

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

// as per specs, actions are "single-character"
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            NORTH_DIRECTION => Direction::North,
            SOUTH_DIRECTION => Direction::South,
            EAST_DIRECTION => Direction::East,
            WEST_DIRECTION => Direction::West,
            LEFT_DIRECTION => Direction::Left,
            RIGHT_DIRECTION => Direction::Right,
            FORWARD_DIRECTION => Direction::Forward,
            v => panic!("unknown direction - {}", v),
        }
    }
}

#[derive(Copy, Clone)]
struct Action {
    direction: Direction,
    magnitude: isize,
}

impl From<&String> for Action {
    fn from(raw: &String) -> Self {
        if !raw.is_ascii() {
            panic!("received non-ascii input")
        }
        let (raw_direction, raw_magnitude) = raw.split_at(1);

        let direction = Direction::from(
            raw_direction
                .chars()
                .next()
                .expect("failed to recover direction"),
        );
        let magnitude = raw_magnitude.parse().expect("failed to parse magnitude");
        Action {
            direction,
            magnitude,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.direction, self.magnitude)
    }
}

impl Action {
    fn is_rotate(&self) -> bool {
        matches!(self.direction, Direction::Left | Direction::Right)
    }
}

struct Ship {
    position: (isize, isize),
    facing_direction: Direction,
}

impl Ship {
    fn new() -> Self {
        Ship {
            position: (0, 0),
            facing_direction: Direction::East,
        }
    }

    fn apply_action(&mut self, action: Action) {
        if action.is_rotate() {
            self.apply_rotation(action)
        } else {
            self.apply_movement(action)
        }
    }

    fn apply_rotation(&mut self, action: Action) {
        let directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let mut current_idx = 0;
        for (i, dir) in directions.iter().enumerate() {
            if self.facing_direction == *dir {
                current_idx = i
            }
        }

        let magnitude = (action.magnitude / 90) as usize;

        match action.direction {
            Direction::Right => self.facing_direction = directions[((current_idx + magnitude) % 4)],
            Direction::Left => {
                self.facing_direction = directions[((current_idx + 4 - magnitude) % 4)]
            }
            _ => unreachable!(),
        }
    }

    fn apply_movement(&mut self, action: Action) {
        match action.direction {
            Direction::North => self.position.1 += action.magnitude,
            Direction::South => self.position.1 -= action.magnitude,
            Direction::East => self.position.0 += action.magnitude,
            Direction::West => self.position.0 -= action.magnitude,
            Direction::Forward => self.apply_movement(Action {
                direction: self.facing_direction,
                magnitude: action.magnitude,
            }),
            _ => unreachable!(),
        }
    }
}

fn part1(input: &[String]) -> usize {
    let mut ship = Ship::new();
    input
        .iter()
        .map(Action::from)
        .for_each(|action| ship.apply_action(action));

    (ship.position.0.abs() + ship.position.1.abs()) as usize
}

// fn part2(input: &[String]) -> usize {
//     0
// }

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_line_input("input").expect("failed to read input file");

    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ];

        let expected = 25;

        assert_eq!(expected, part1(&input));
    }
}
