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

use std::ops::{Add, AddAssign};

// Point contains list of values for each dimension
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point(pub Vec<isize>);

impl Point {
    pub fn base(dims: usize) -> Point {
        Point(vec![0; dims])
    }

    pub fn dimension_adjacent(&self, dim: usize) -> Vec<Point> {
        let mut positively_adjacent = self.clone();
        positively_adjacent.0[dim - 1] += 1;

        let mut negatively_adjacent = self.clone();
        negatively_adjacent.0[dim - 1] -= 1;

        vec![positively_adjacent, negatively_adjacent]
    }

    pub fn adjacent_points(&self) -> Vec<Point> {
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

impl Add<(isize, isize, isize)> for &Point {
    type Output = Point;

    fn add(self, rhs: (isize, isize, isize)) -> Self::Output {
        assert_eq!(3, self.0.len());

        Point(vec![
            self.0[0] + rhs.0,
            self.0[1] + rhs.1,
            self.0[2] + rhs.2,
        ])
    }
}

impl AddAssign<(isize, isize, isize)> for Point {
    fn add_assign(&mut self, rhs: (isize, isize, isize)) {
        assert_eq!(3, self.0.len());

        self.0[0] += rhs.0;
        self.0[1] += rhs.1;
        self.0[2] += rhs.2;
    }
}
