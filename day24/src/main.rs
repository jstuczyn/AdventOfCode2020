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

use day17::Point;
use std::collections::HashSet;
use utils::input_read;

const EAST: char = 'e';
const SOUTH: char = 's';
const WEST: char = 'w';
const NORTH: char = 'n';

// represent Hexagon as a 3D point
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Hexagon {
    location: Point,
}

impl From<&String> for Hexagon {
    fn from(raw: &String) -> Self {
        let mut base = Point::base(3);

        // east =>          (1, -1, 0)
        // south-east =>    (0, -1, 1)
        // south-west =>    (-1, 0, 1)
        // west =>          (-1, 1, 0)
        // north-west =>    (0, 1, -1)
        // north-east =>    (1, 0, -1)

        // e, se, sw, w, nw, ne

        let mut iter = raw.chars();
        while let Some(current) = iter.next() {
            match current {
                EAST => base += (1, -1, 0),
                SOUTH => match iter.next().expect("invalid hex direction") {
                    EAST => base += (0, -1, 1),
                    WEST => base += (-1, 0, 1),
                    _ => panic!("invalid hex direction"),
                },
                WEST => base += (-1, 1, 0),
                NORTH => match iter.next().expect("invalid hex direction") {
                    WEST => base += (0, 1, -1),
                    EAST => base += (1, 0, -1),
                    _ => panic!("invalid hex direction"),
                },
                _ => panic!("invalid hex direction"),
            }
        }

        Hexagon { location: base }
    }
}

fn part1(input: &[String]) -> usize {
    let mut active = HashSet::new();
    input.iter().map(Hexagon::from).for_each(|hex| {
        if active.contains(&hex) {
            active.remove(&hex);
        } else {
            active.insert(hex);
        }
    });

    active.len()
}

fn part2(input: &[String]) -> usize {
    0
}

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_line_input("input").expect("failed to read input file");

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
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string(),
        ];

        let expected = 10;

        assert_eq!(expected, part1(&input))
    }
}
