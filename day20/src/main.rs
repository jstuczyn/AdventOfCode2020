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

use std::collections::VecDeque;
use std::fmt::{self, Display, Formatter};
use std::mem;
use utils::input_read;

const ACTIVE_PIXEL: char = '#';
const INACTIVE_PIXEL: char = '.';

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pixel {
    Active,
    Inactive,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            ACTIVE_PIXEL => Pixel::Active,
            INACTIVE_PIXEL => Pixel::Inactive,
            _ => panic!("invalid pixel {}", c),
        }
    }
}

impl Into<char> for Pixel {
    fn into(self) -> char {
        match self {
            Pixel::Active => ACTIVE_PIXEL,
            Pixel::Inactive => INACTIVE_PIXEL,
        }
    }
}

#[derive(Clone)]
struct Tile {
    id: usize,
    rows: Vec<Vec<Pixel>>,
}

impl From<&String> for Tile {
    fn from(raw: &String) -> Self {
        let lines: Vec<_> = raw.split('\n').collect();
        // first line contains id
        let id = lines[0]
            .strip_suffix(':')
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .parse()
            .expect("failed to parse tile id");

        let rows = lines[1..]
            .iter()
            .map(|row| row.chars().map(Pixel::from).collect())
            .collect();

        Tile { id, rows }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for row in self.rows.iter() {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|&pixel| Into::<char>::into(pixel))
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl Tile {
    fn rotate_clockwise(&mut self) {
        let columns: Vec<_> = (0..self.rows.len())
            .map(|i| {
                let mut column = self.column(i);
                column.reverse();
                column
            })
            .collect();
        self.rows = columns
    }

    fn flip_horizontally(&mut self) {
        for row in self.rows.iter_mut() {
            row.reverse()
        }
    }

    fn flip_vertically(&mut self) {
        self.rows.reverse();
    }

    fn column(&self, idx: usize) -> Vec<Pixel> {
        self.rows.iter().map(|row| row[idx]).collect()
    }

    fn row(&self, idx: usize) -> &[Pixel] {
        &self.rows[idx]
    }

    fn top_row(&self) -> &[Pixel] {
        self.row(0)
    }

    fn bottom_row(&self) -> &[Pixel] {
        self.row(self.rows.len() - 1)
    }

    fn left_column(&self) -> Vec<Pixel> {
        self.column(0)
    }

    fn right_column(&self) -> Vec<Pixel> {
        self.column(self.rows[0].len() - 1)
    }

    fn edges(&self) -> Edges {
        Edges {
            id: self.id,

            top: self.top_row().to_vec(),
            right: self.right_column(),
            bottom: self.bottom_row().to_vec(),
            left: self.left_column(),
        }
    }
}

#[derive(Debug)]
struct Edges {
    id: usize,

    top: Vec<Pixel>,
    right: Vec<Pixel>,
    bottom: Vec<Pixel>,
    left: Vec<Pixel>,
}

impl Edges {
    fn flip_horizontally(&mut self) {
        mem::swap(&mut self.right, &mut self.left)
    }

    fn flip_vertically(&mut self) {
        mem::swap(&mut self.top, &mut self.bottom)
    }

    fn rotate_clockwise(&mut self) {
        // start: {top, right, bottom, left}

        // {right, top, bottom, left}
        mem::swap(&mut self.top, &mut self.right);

        // {bottom, top, right, left}
        mem::swap(&mut self.top, &mut self.bottom);

        // {left, top, right, bottom}
        mem::swap(&mut self.top, &mut self.left);

        self.top.reverse();
        self.bottom.reverse();
    }

    fn matches(&self, other: &Self) -> bool {
        // TODO: just keep it on hand
        let self_reversed = self.reversed();
        let all_self = std::iter::once(&self.top)
            .chain(std::iter::once(&self.right))
            .chain(std::iter::once(&self.bottom))
            .chain(std::iter::once(&self.left))
            .chain(std::iter::once(&self_reversed.top))
            .chain(std::iter::once(&self_reversed.right))
            .chain(std::iter::once(&self_reversed.bottom))
            .chain(std::iter::once(&self_reversed.left));

        for self_edge in all_self {
            for other_edge in std::iter::once(&other.top)
                .chain(std::iter::once(&other.right))
                .chain(std::iter::once(&other.bottom))
                .chain(std::iter::once(&other.left))
            {
                if self_edge == other_edge {
                    return true;
                }
            }
        }

        false
    }

    fn reversed(&self) -> Self {
        Edges {
            id: self.id,
            top: self.top.iter().cloned().rev().collect(),
            right: self.right.iter().cloned().rev().collect(),
            bottom: self.bottom.iter().cloned().rev().collect(),
            left: self.left.iter().cloned().rev().collect(),
        }
    }
}

struct TileGrid {
    tiles: VecDeque<VecDeque<Option<Tile>>>,
}

impl TileGrid {
    fn insert_tiles(&mut self, mut tiles: Vec<Tile>) -> usize {
        // find two tiles and their orientations such that they don't fit to any other tile

        // start with any tile we have
        // self.tiles[0][0] = tiles.pop();
        //
        // let edges = self.tiles[0][0].as_ref().unwrap().edges();

        // for now, don't bother with whole thing, just find corners and multiply ids
        let mut corner_product = 1;
        'outer: for tile1 in &tiles {
            let edges1 = tile1.edges();
            let mut tile1_matches = 0;
            for tile2 in &tiles {
                if tile1.id == tile2.id {
                    continue;
                }

                let edges2 = tile2.edges();
                if edges1.matches(&edges2) {
                    tile1_matches += 1;
                }
                if tile1_matches > 2 {
                    continue 'outer;
                }
            }
            if tile1_matches == 2 {
                corner_product *= tile1.id;
            }
            // rotate
            // flip h, flip v
            // rotate
            // flips
            // rotate
            // flips

            // let other_edges = tile.edges();
            // edges.matches(&other_edges);
            // a,b,c,d
            // flip H:
            // flip V:

            // rotate 1x:
            // flip H 1x:
            // flip V 1x:

            // rotate 2x:
        }

        // flip H -> rotate -> flip H

        return corner_product;
    }
}

impl TileGrid {
    fn new(size: usize) -> Self {
        let tiles = std::iter::repeat(std::iter::repeat(None).take(size).collect())
            .take(size)
            .collect();

        TileGrid { tiles }
    }

    fn shift_all_down(&mut self) {
        // remove last row (and make sure it's all empty!)
        assert!(self
            .tiles
            .pop_back()
            .unwrap()
            .iter()
            .all(|tile| tile.is_none()));
        // push empty row to the front
        self.tiles
            .push_front(std::iter::repeat(None).take(self.tiles[0].len()).collect())
    }

    fn shift_all_right(&mut self) {
        for row in self.tiles.iter_mut() {
            // pop first element from each row (making sure it was actually empty)
            assert!(row.pop_front().unwrap().is_none());
            // and push an empty one to the back
            row.push_back(None);
        }
    }
}

fn part1(input: &[String]) -> usize {
    let mut tiles: Vec<_> = input.iter().map(Tile::from).collect();

    let side_len = (tiles.len() as f64).sqrt();
    // make sure it's a perfect square
    assert_eq!((side_len as usize).pow(2), tiles.len());

    let mut tile_grid = TileGrid::new(side_len as usize);
    tile_grid.insert_tiles(tiles)
}

// fn part2(input: &[String]) -> usize {
// 0
// }

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_into_string_groups("input").expect("failed to read input file");

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
            r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###"#
                .to_string(),
            r#"Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#.."#
                .to_string(),
            r#"Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##..."#
                .to_string(),
            r#"Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#."#
                .to_string(),
            r#"Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#.."#
                .to_string(),
            r#"Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#."#
                .to_string(),
            r#"Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#"#
                .to_string(),
            r#"Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##."#
                .to_string(),
            r#"Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#
                .to_string(),
        ];

        let expected = 20899048083289;

        assert_eq!(expected, part1(&input));
    }
}
