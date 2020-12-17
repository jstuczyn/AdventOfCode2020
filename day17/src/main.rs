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

use std::cmp::{max, min};
use std::mem;
use std::ops::{Index, IndexMut, Range};
use utils::input_read;

const ACTIVE_CUBE: char = '#';
const INACTIVE_CUBE: char = '.';

#[derive(Debug, Clone, PartialEq, Copy)]
enum Cube {
    Inactive,
    Active,
}

impl From<char> for Cube {
    fn from(value: char) -> Self {
        match value {
            INACTIVE_CUBE => Cube::Inactive,
            ACTIVE_CUBE => Cube::Active,
            v => panic!("invalid cube state {}", v),
        }
    }
}

impl Into<char> for Cube {
    fn into(self) -> char {
        match self {
            Cube::Inactive => INACTIVE_CUBE,
            Cube::Active => ACTIVE_CUBE,
        }
    }
}

impl Cube {
    fn swap(&mut self) {
        mem::swap(
            self,
            &mut match self {
                Cube::Inactive => Cube::Active,
                Cube::Active => Cube::Inactive,
            },
        );
    }

    fn is_active(&self) -> bool {
        matches!(self, Cube::Active)
    }
}

enum Dimension {
    One(CubeRow),
    Higher(HigherDimension),
}

struct CubeRow {
    // holds values at [0, ...)
    positive_index: Vec<Cube>,
    // holds values at (..., -1]
    negative_index: Vec<Cube>,
}

impl CubeRow {
    fn extend(&mut self, size: usize) {
        self.positive_index
            .extend(std::iter::repeat(Cube::Inactive).take(size));
        self.negative_index
            .extend(std::iter::repeat(Cube::Inactive).take(size));
    }

    fn dimension_range(&self) -> Range<isize> {
        -(self.negative_index.len() as isize)..self.positive_index.len() as isize
    }
}

// defines dimensions 3 and above
struct HigherDimension {
    // helper
    dimension: usize,

    // the reason for spiting each dimension in two parts is to avoid moving all
    // data each time it needs to expands

    // holds values at [0, ...)
    positive_index: Vec<Dimension>,
    // holds values at (..., -1]
    negative_index: Vec<Dimension>,
}

impl HigherDimension {
    fn dimension_range(&self) -> Range<isize> {
        -(self.negative_index.len() as isize)..self.positive_index.len() as isize
    }

    fn combine_ranges(range1: Option<Range<isize>>, range2: Option<Range<isize>>) -> Range<isize> {
        if range1.is_none() && range2.is_none() {
            0..0
        } else if range1.is_none() {
            range2.unwrap()
        } else if range2.is_none() {
            range1.unwrap()
        } else {
            let range1 = range1.unwrap();
            let range2 = range2.unwrap();
            min(range1.start, range2.start)..max(range1.end, range2.end)
        }
    }

    // we're using the assumption that each dimension (i.e. every row or every plane) have the same
    // sizes. It does NOT mean that size(plane) == size(row), only that size(row_a) == size(row_b)
    fn space_range(&self) -> Vec<Range<isize>> {
        let mut range = Vec::with_capacity(self.dimension);
        range.push(self.dimension_range());

        let positive_ranges = match &self.positive_index.get(0) {
            Some(item) => match item {
                Dimension::One(row) => Some(vec![row.dimension_range()]),
                Dimension::Higher(higher) => Some(higher.space_range()),
            },
            None => None,
        };

        let negative_ranges = match &self.negative_index.get(0) {
            Some(item) => match item {
                Dimension::One(row) => Some(vec![row.dimension_range()]),
                Dimension::Higher(higher) => Some(higher.space_range()),
            },
            None => None,
        };

        range.extend(
            positive_ranges
                .into_iter()
                .zip(negative_ranges.into_iter())
                .map(|(pos, neg)| Self::combine_ranges(pos, neg)),
        );

        // this will return (w, z, y, x)
        range
    }
}

// Point contains list of values for each dimension
#[derive(Debug, Eq, PartialEq, Clone)]
struct Point(Vec<isize>);

impl Point {
    fn dimension_adjacent(&self, dim: usize) -> Vec<Point> {
        let mut positively_adjacent = self.clone();
        positively_adjacent.0[dim - 1] += 1;

        let mut negatively_adjacent = self.clone();
        negatively_adjacent.0[dim - 1] -= 1;

        vec![positively_adjacent, negatively_adjacent]
    }

    fn adjacent_points(&self) -> Vec<Point> {
        let mut adjacent = Vec::with_capacity((3usize.pow(self.0.len() as u32 - 1)) as usize);

        for dim in 1..=self.0.len() {
            let mut dim_adjacent = Vec::new();
            for adj in adjacent.iter().chain(std::iter::once(self)) {
                dim_adjacent.append(&mut adj.dimension_adjacent(dim))
            }
            adjacent.extend(dim_adjacent.into_iter().filter(|p| p != self))
        }

        adjacent
    }
}

impl Index<Point> for Dimension {
    type Output = Cube;

    fn index(&self, index: Point) -> &Self::Output {
        self.index(&index.0[..])
    }
}

impl IndexMut<Point> for Dimension {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.index_mut(&index.0[..])
    }
}

impl Index<&[isize]> for Dimension {
    type Output = Cube;

    fn index(&self, index: &[isize]) -> &Self::Output {
        match self {
            Dimension::One(row) => {
                debug_assert_eq!(1, index.len());
                row.index(index[0])
            }
            Dimension::Higher(higher_dim) => higher_dim.index(index),
        }
    }
}

impl IndexMut<&[isize]> for Dimension {
    fn index_mut(&mut self, index: &[isize]) -> &mut Self::Output {
        match self {
            Dimension::One(row) => {
                debug_assert_eq!(1, index.len());
                row.index_mut(index[0])
            }
            Dimension::Higher(higher_dim) => higher_dim.index_mut(index),
        }
    }
}

impl Index<&[isize]> for HigherDimension {
    type Output = Cube;

    fn index(&self, index: &[isize]) -> &Self::Output {
        debug_assert_eq!(index.len(), self.dimension);
        let current_dim = index[self.dimension - 1];
        if current_dim >= 0 {
            self.positive_index[current_dim as usize].index(&index[..self.dimension - 1])
        } else {
            self.negative_index[current_dim.abs() as usize - 1].index(&index[..self.dimension - 1])
        }
    }
}

impl IndexMut<&[isize]> for HigherDimension {
    fn index_mut(&mut self, index: &[isize]) -> &mut Self::Output {
        debug_assert_eq!(index.len(), self.dimension);
        let current_dim = index[self.dimension - 1];
        if current_dim >= 0 {
            self.positive_index[current_dim as usize].index_mut(&index[..self.dimension - 1])
        } else {
            self.negative_index[current_dim.abs() as usize - 1]
                .index_mut(&index[..self.dimension - 1])
        }
    }
}

impl Index<isize> for CubeRow {
    type Output = Cube;

    fn index(&self, index: isize) -> &Self::Output {
        if index >= 0 {
            &self.positive_index[index as usize]
        } else {
            &self.negative_index[index.abs() as usize - 1]
        }
    }
}

impl IndexMut<isize> for CubeRow {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        if index >= 0 {
            &mut self.positive_index[index as usize]
        } else {
            &mut self.negative_index[index.abs() as usize - 1]
        }
    }
}

fn parse_initial_data(input: &[String], dims: usize) -> Dimension {
    // initial input is always 2D. It's just the question of how many outer layers it's going to have
    let rows: Vec<_> = input
        .iter()
        .map(|raw| raw.chars().map(Cube::from).collect::<Vec<_>>())
        .map(|initial_row| {
            Dimension::One(CubeRow {
                positive_index: initial_row,
                negative_index: Vec::new(),
            })
        })
        .collect();

    let mut current_dim = Some(Dimension::Higher(HigherDimension {
        dimension: 2,
        positive_index: rows,
        negative_index: Vec::new(),
    }));
    let mut current_dim_val = 2;
    // add outer dimensions
    while current_dim_val != dims {
        current_dim_val += 1;
        let inner_dim = current_dim.take().unwrap();
        current_dim = Some(Dimension::Higher(HigherDimension {
            dimension: current_dim_val,
            positive_index: vec![inner_dim],
            negative_index: Vec::new(),
        }))
    }

    current_dim.unwrap()
}

impl Dimension {
    fn dimension(&self) -> usize {
        match self {
            Dimension::One(_) => 1,
            Dimension::Higher(higher) => higher.dimension,
        }
    }

    fn adjacent_cubes(&self, point: Point) -> Vec<Cube> {
        let adjacent_points = point.adjacent_points();
        adjacent_points.into_iter().map(|p| self[p]).collect()
    }

    fn space_range(&self) -> Vec<Range<isize>> {
        match self {
            Dimension::One(row) => vec![row.dimension_range()],
            Dimension::Higher(higher) => {
                let mut range = higher.space_range();
                range.reverse();
                range
            }
        }
    }

    fn dimension_range(&self) -> Range<isize> {
        match self {
            Dimension::One(row) => row.dimension_range(),
            Dimension::Higher(higher) => higher.dimension_range(),
        }
    }

    fn iterate(&self) {
        match self {
            Dimension::One(row) => {}
            Dimension::Higher(higher) => {}
        }
    }

    fn simulate_step(&mut self) {
        let mut cubes_to_swap: Vec<&mut Cube> = Vec::new();

        for cube in cubes_to_swap {
            cube.swap()
        }
    }
}

fn part1(input: &[String]) -> usize {
    let data = parse_initial_data(input, 2);

    println!("{:?}", data.space_range());

    // let point = Point(vec![0]);
    // println!("{:?}", point.adjacent_points());
    //
    // let point = Point(vec![0, 0]);
    // println!("{:?}", point.adjacent_points());
    //
    // let point = Point(vec![0, 0, 0]);
    // println!("{:?}", point.adjacent_points());

    0
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
    fn negative_row_indexing() {
        let positive = vec!['#'.into(), '.'.into(), '#'.into()];
        let negative = vec!['.'.into(), '#'.into(), '.'.into()];
        let row = CubeRow {
            positive_index: positive,
            negative_index: negative,
        };

        assert_eq!(row[0], '#'.into());
        assert_eq!(row[1], '.'.into());
        assert_eq!(row[2], '#'.into());

        assert_eq!(row[-1], '.'.into());
        assert_eq!(row[-2], '#'.into());
        assert_eq!(row[-3], '.'.into());
    }

    #[test]
    fn dimension_range_for_row() {
        let positive = vec!['#'.into(), '.'.into(), '#'.into()];
        let negative = vec!['.'.into(), '#'.into(), '.'.into()];
        let row = CubeRow {
            positive_index: positive,
            negative_index: negative,
        };

        assert_eq!(row.dimension_range(), -3..3)
    }

    #[test]
    fn indexing_in_2d() {
        let input = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];
        let dim = parse_initial_data(&input, 2);

        assert_eq!('.', dim[Point(vec![0, 0])].into());
        assert_eq!('#', dim[Point(vec![1, 0])].into());
        assert_eq!('.', dim[Point(vec![2, 0])].into());

        assert_eq!('.', dim[Point(vec![0, 1])].into());
        assert_eq!('.', dim[Point(vec![1, 1])].into());
        assert_eq!('#', dim[Point(vec![2, 1])].into());

        assert_eq!('#', dim[Point(vec![0, 2])].into());
        assert_eq!('#', dim[Point(vec![1, 2])].into());
        assert_eq!('#', dim[Point(vec![2, 2])].into());
    }

    #[test]
    fn part1_sample_input() {
        let input = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];

        let expected = 112;

        assert_eq!(expected, part1(&input))
    }
}
