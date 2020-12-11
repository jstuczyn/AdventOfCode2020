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
use std::mem;
use std::ops::{Index, IndexMut};
use utils::input_read;

const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';
const FLOOR: char = '.';

#[derive(Debug, Clone, PartialEq, Copy)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl From<char> for Seat {
    fn from(value: char) -> Self {
        match value {
            EMPTY_SEAT => Seat::Empty,
            OCCUPIED_SEAT => Seat::Occupied,
            FLOOR => Seat::Floor,
            v => panic!("invalid seat state {}", v),
        }
    }
}

impl Into<char> for Seat {
    fn into(self) -> char {
        match self {
            Seat::Empty => EMPTY_SEAT,
            Seat::Occupied => OCCUPIED_SEAT,
            Seat::Floor => FLOOR,
        }
    }
}

impl Seat {
    fn swap(&mut self) {
        mem::swap(
            self,
            &mut match self {
                Seat::Empty => Seat::Occupied,
                Seat::Occupied => Seat::Empty,
                Seat::Floor => Seat::Floor,
            },
        );
    }

    fn is_floor(&self) -> bool {
        matches!(self, Seat::Floor)
    }

    fn is_empty(&self) -> bool {
        matches!(self, Seat::Empty)
    }

    fn is_occupied(&self) -> bool {
        matches!(self, Seat::Occupied)
    }
}

type SeatRow = Vec<Seat>;
type SeatPosition = (usize, usize);

#[derive(PartialEq)]
struct SeatGrid(Vec<SeatRow>);

impl Index<SeatPosition> for SeatGrid {
    type Output = Seat;

    fn index(&self, index: SeatPosition) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

impl IndexMut<SeatPosition> for SeatGrid {
    fn index_mut(&mut self, index: SeatPosition) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}

impl From<&[String]> for SeatGrid {
    fn from(raw_rows: &[String]) -> Self {
        Self(
            raw_rows
                .iter()
                .map(|row| row.chars().map(Seat::from).collect())
                .collect(),
        )
    }
}

impl Display for SeatGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            let row_string: String = row
                .iter()
                .map(|&seat| {
                    let char: char = seat.into();
                    char
                })
                .collect();
            writeln!(f, "{}", row_string)?;
        }
        Ok(())
    }
}

impl SeatGrid {
    fn adjacent_seats(&self, position: SeatPosition) -> Vec<Seat> {
        let mut adjacent = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                if let Some(seat) = self.attempt_seat_lookup(position, (i, j)) {
                    adjacent.push(seat)
                }
            }
        }

        adjacent
    }

    fn attempt_seat_lookup(
        &self,
        position: SeatPosition,
        translation: (isize, isize),
    ) -> Option<Seat> {
        let (x, y) = position;
        let (dx, dy) = translation;
        let translated = (x as isize + dx, y as isize + dy);

        if translated.0 < 0
            || translated.0 >= self.0[0].len() as isize
            || translated.1 < 0
            || translated.1 >= self.0.len() as isize
        {
            None
        } else {
            // based on previous checks we know we can safely cast it
            let new_position = (translated.0 as usize, translated.1 as usize);

            Some(self[new_position])
            // if !seat.is_floor() {
            //     // we don't care about floor seats (at least in part 1)
            //     Some(seat)
            // } else {
            //     None
            // }
        }
    }

    fn simulate_step(&self) -> Self {
        let mut new_grid = SeatGrid(self.0.clone());

        self.0.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, seat)| {
                if !seat.is_floor() {
                    let adjacent = self.adjacent_seats((x, y));

                    // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                    if seat.is_empty() && !adjacent.iter().any(|adj| adj.is_occupied()) {
                        new_grid[(x, y)].swap();
                    }
                    // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                    if seat.is_occupied()
                        && adjacent.iter().filter(|seat| seat.is_occupied()).count() >= 4
                    {
                        new_grid[(x, y)].swap();
                    }
                }
            })
        });

        new_grid
    }

    fn occupied_count(&self) -> usize {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .filter(|seat| seat.is_occupied())
            .count()
    }
}

fn part1(input: &[String]) -> usize {
    let mut grid = SeatGrid::from(input);
    loop {
        let next_grid = grid.simulate_step();
        if next_grid == grid {
            break;
        }
        grid = next_grid;
    }
    grid.occupied_count()
}

// fn part2(input: &[String]) -> usize {
//     0
// }

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
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ];

        let expected = 37;

        assert_eq!(expected, part1(&input))
    }
}
