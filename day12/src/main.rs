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

type Position = (isize, isize);

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
    fn is_rotation(&self) -> bool {
        matches!(self.direction, Direction::Left | Direction::Right)
    }

    fn is_translation(&self) -> bool {
        matches!(
            self.direction,
            Direction::North | Direction::East | Direction::South | Direction::West
        )
    }
}

#[derive(Eq, PartialEq)]
enum NavigationMode {
    Absolute,
    Waypoint,
}

struct Ship {
    position: Position,
    waypoint: Waypoint,
    mode: NavigationMode,
}

impl Ship {
    fn new(waypoint_position: Position, mode: NavigationMode) -> Self {
        Ship {
            position: (0, 0),
            waypoint: Waypoint {
                relative_position: waypoint_position,
            },
            mode,
        }
    }

    fn apply_action(&mut self, action: Action) {
        if action.is_rotation() {
            self.waypoint.apply_rotation(action)
        } else if action.is_translation() {
            if self.mode == NavigationMode::Waypoint {
                self.waypoint.apply_translation(action)
            } else {
                self.apply_self_translation(action)
            }
        } else {
            // it must be forward
            self.move_towards_waypoint(action);
        }
    }

    fn apply_self_translation(&mut self, action: Action) {
        match action.direction {
            Direction::North => self.position.1 += action.magnitude,
            Direction::South => self.position.1 -= action.magnitude,
            Direction::East => self.position.0 += action.magnitude,
            Direction::West => self.position.0 -= action.magnitude,
            _ => unreachable!(),
        }
    }

    fn move_towards_waypoint(&mut self, action: Action) {
        debug_assert_eq!(action.direction, Direction::Forward);
        let (x, y) = self.position;
        let dx = self.waypoint.relative_position.0 * action.magnitude;
        let dy = self.waypoint.relative_position.1 * action.magnitude;
        self.position = (x + dx, y + dy)
    }
}

struct Waypoint {
    relative_position: Position,
}

impl Waypoint {
    fn apply_rotation(&mut self, action: Action) {
        let magnitude = if action.direction == Direction::Right {
            action.magnitude
        } else {
            360 - action.magnitude
        };

        let (x, y) = self.relative_position;
        match magnitude {
            90 => self.relative_position = (y, -x),
            180 => self.relative_position = (-x, -y),
            270 => self.relative_position = (-y, x),
            360 => (),
            v => panic!("invalid rotation - {}", v),
        }
    }

    fn apply_translation(&mut self, action: Action) {
        debug_assert!(action.is_translation());
        match action.direction {
            Direction::North => self.relative_position.1 += action.magnitude,
            Direction::South => self.relative_position.1 -= action.magnitude,
            Direction::East => self.relative_position.0 += action.magnitude,
            Direction::West => self.relative_position.0 -= action.magnitude,
            _ => unreachable!(),
        }
    }
}

fn part1(input: &[String]) -> usize {
    let mut ship = Ship::new((1, 0), NavigationMode::Absolute);

    input
        .iter()
        .map(Action::from)
        .for_each(|action| ship.apply_action(action));

    (ship.position.0.abs() + ship.position.1.abs()) as usize
}

fn part2(input: &[String]) -> usize {
    let mut ship = Ship::new((10, 1), NavigationMode::Waypoint);

    input
        .iter()
        .map(Action::from)
        .for_each(|action| ship.apply_action(action));

    (ship.position.0.abs() + ship.position.1.abs()) as usize
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

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ];

        let expected = 286;

        assert_eq!(expected, part2(&input));
    }
}
