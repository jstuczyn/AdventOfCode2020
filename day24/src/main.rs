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
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use utils::input_read;

const EAST: char = 'e';
const SOUTH: char = 's';
const WEST: char = 'w';
const NORTH: char = 'n';

const DAYS_TO_SIMULATE: usize = 100;

const EAST_DIR: (isize, isize, isize) = (1, -1, 0);
const SOUTH_EAST_DIR: (isize, isize, isize) = (0, -1, 1);
const SOUTH_WEST_DIR: (isize, isize, isize) = (-1, 0, 1);
const WEST_DIR: (isize, isize, isize) = (-1, 1, 0);
const NORTH_WEST_DIR: (isize, isize, isize) = (0, 1, -1);
const NORTH_EAST_DIR: (isize, isize, isize) = (1, 0, -1);

// represent Hexagon as a 3D point
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Hexagon {
    location: Point,
}

impl From<&String> for Hexagon {
    fn from(raw: &String) -> Self {
        let mut reference = Point::base(3);

        let mut iter = raw.chars();
        while let Some(current) = iter.next() {
            match current {
                EAST => reference += EAST_DIR,
                SOUTH => match iter.next().expect("invalid hex direction") {
                    EAST => reference += SOUTH_EAST_DIR,
                    WEST => reference += SOUTH_WEST_DIR,
                    _ => panic!("invalid hex direction"),
                },
                WEST => reference += WEST_DIR,
                NORTH => match iter.next().expect("invalid hex direction") {
                    WEST => reference += NORTH_WEST_DIR,
                    EAST => reference += NORTH_EAST_DIR,
                    _ => panic!("invalid hex direction"),
                },
                _ => panic!("invalid hex direction"),
            }
        }

        Hexagon {
            location: reference,
        }
    }
}

impl Hexagon {
    fn adjacent_hexes(&self) -> Vec<Hexagon> {
        vec![
            Hexagon {
                location: &self.location + EAST_DIR,
            },
            Hexagon {
                location: &self.location + SOUTH_EAST_DIR,
            },
            Hexagon {
                location: &self.location + SOUTH_WEST_DIR,
            },
            Hexagon {
                location: &self.location + WEST_DIR,
            },
            Hexagon {
                location: &self.location + NORTH_WEST_DIR,
            },
            Hexagon {
                location: &self.location + NORTH_EAST_DIR,
            },
        ]
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

struct SimulatedHexagon {
    hexagon: Hexagon,
    neighbours: Vec<Hexagon>,
    should_deactivate: bool,
}

fn simulate_step_par(active_hexes: &mut HashSet<Hexagon>) {
    let simulated_hexes: Vec<_> = active_hexes
        .par_iter()
        .map(|active_hexagon| {
            let neighbours = active_hexagon.adjacent_hexes();
            let active_neighbours = neighbours
                .par_iter()
                .map(|neighbour| active_hexes.contains(neighbour))
                .filter(|is_active| *is_active)
                .count();

            SimulatedHexagon {
                hexagon: active_hexagon.clone(),
                neighbours,
                should_deactivate: active_neighbours == 0 || active_neighbours > 2,
            }
        })
        .collect();

    let mut all_adjacents = HashMap::new();
    for simulated_hex in simulated_hexes {
        if simulated_hex.should_deactivate {
            active_hexes.remove(&simulated_hex.hexagon);
        }
        for neighbour in simulated_hex.neighbours {
            *all_adjacents.entry(neighbour).or_insert(0) += 1;
        }
    }

    for (adjacent, count) in all_adjacents.into_iter() {
        if count == 2 {
            active_hexes.insert(adjacent);
        }
    }
}

fn part2(input: &[String]) -> usize {
    let mut active = HashSet::new();
    input.iter().map(Hexagon::from).for_each(|hex| {
        if active.contains(&hex) {
            active.remove(&hex);
        } else {
            active.insert(hex);
        }
    });

    for _ in 0..DAYS_TO_SIMULATE {
        simulate_step_par(&mut active);
    }

    active.len()
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

    #[test]
    fn part2_sample_input() {
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

        let expected = 2208;

        assert_eq!(expected, part2(&input))
    }
}
