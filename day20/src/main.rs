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
use std::fmt::{self, Debug, Display, Formatter};
use std::mem;
use utils::input_read;

const ACTIVE_PIXEL: char = '#';
const INACTIVE_PIXEL: char = '.';

#[derive(Debug)]
enum MatchedSide {
    Top,
    Right,
    Bottom,
    Left,
}

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
            Pixel::Active => '■',   //'ACTIVE_PIXEL,
            Pixel::Inactive => '□', //INACTIVE_PIXEL,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    id: usize,
    rows: Vec<Vec<Pixel>>,
}

impl From<&String> for Tile {
    fn from(raw: &String) -> Self {
        let lines: Vec<_> = raw.lines().collect();
        // first line contains id
        let id = lines[0]
            .strip_suffix(':')
            .expect(&*format!("wtf - {}", lines[0]))
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
    fn unset() -> Self {
        Tile {
            id: 0,
            rows: std::iter::repeat(std::iter::repeat(Pixel::Inactive).take(10).collect())
                .take(10)
                .collect(),
        }
    }

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
        self.rows.reverse();
    }

    fn flip_vertically(&mut self) {
        for row in self.rows.iter_mut() {
            row.reverse()
        }
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

    fn rotate_n_clockwise(&mut self, n: usize) {
        for _ in 0..n {
            self.rotate_clockwise();
        }
    }

    fn try_match(&self, other: &mut Self) -> Option<MatchedSide> {
        // println!("trying to match {} with {}", self.id, other.id);
        let self_edges = self.edges();
        let mut other_edges = other.edges();
        if !self_edges.matches(&other_edges) {
            // see if the match is even possible
            return None;
        }

        // and only then determine correct orientation of the other tile

        if let Some(side) = self_edges.direct_match(&other_edges) {
            println!("no rotation on {:?}", side);
            return Some(side);
        }

        // for 90, 180, 270 rotation...
        for i in 1..=3 {
            other_edges.rotate_clockwise();
            if let Some(side) = self_edges.direct_match(&other_edges) {
                println!("with {} rotations on {:?}", i, side);
                other.rotate_n_clockwise(i);
                return Some(side);
            }

            // see if horizontal flip is required
            other_edges.flip_horizontally();
            if let Some(side) = self_edges.direct_match(&other_edges) {
                println!("with {} rotations + H flip on {:?}", i, side);
                other.rotate_n_clockwise(i);
                other.flip_horizontally();
                return Some(side);
            }
            other_edges.flip_horizontally();

            // or a vertical one
            other_edges.flip_vertically();
            if let Some(side) = self_edges.direct_match(&other_edges) {
                println!("with {} rotations + V flip on {:?}", i, side);
                other.rotate_n_clockwise(i);
                other.flip_vertically();
                return Some(side);
            }

            // also vertical + horizontal
            other_edges.flip_horizontally();
            if let Some(side) = self_edges.direct_match(&other_edges) {
                println!("with {} rotations + HV flip on {:?}", i, side);
                other.rotate_n_clockwise(i);
                other.flip_vertically();
                other.flip_horizontally();
                return Some(side);
            }

            // reverse flips
            other_edges.flip_horizontally();
            other_edges.flip_vertically();

            // there's too many combinations but that's fine for now
        }

        unreachable!("we determined a match was possible!")
    }
}

#[derive(Debug, PartialEq)]
struct Edges {
    id: usize,

    top: Vec<Pixel>,
    right: Vec<Pixel>,
    bottom: Vec<Pixel>,
    left: Vec<Pixel>,
}

impl Edges {
    fn flip_horizontally(&mut self) {
        mem::swap(&mut self.top, &mut self.bottom);
        self.right.reverse();
        self.left.reverse();
    }

    fn flip_vertically(&mut self) {
        mem::swap(&mut self.right, &mut self.left);
        self.top.reverse();
        self.bottom.reverse();
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

    fn direct_match(&self, other: &Self) -> Option<MatchedSide> {
        if self.top == other.bottom {
            Some(MatchedSide::Top)
        } else if self.right == other.left {
            Some(MatchedSide::Right)
        } else if self.bottom == other.top {
            Some(MatchedSide::Bottom)
        } else if self.left == other.right {
            Some(MatchedSide::Left)
        } else {
            None
        }
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
    fn insert_tile(
        &mut self,
        relative_to: (usize, usize),
        side: MatchedSide,
        tile: Tile,
    ) -> Option<(usize, usize)> {
        println!("inserting {} {:?} to {:?}", tile.id, side, relative_to);
        let (x, y) = relative_to;
        match side {
            MatchedSide::Top => {
                if y == 0 {
                    self.shift_all_down();
                    debug_assert!(self.tiles[y][x].is_none());
                    self.tiles[y][x] = Some(tile);
                    return Some((0, 1));
                }

                debug_assert!(self.tiles[y - 1][x].is_none());
                self.tiles[y - 1][x] = Some(tile);
                None
            }
            MatchedSide::Right => {
                debug_assert!(self.tiles[y][x + 1].is_none());
                self.tiles[y][x + 1] = Some(tile);
                None
            }
            MatchedSide::Bottom => {
                debug_assert!(self.tiles[y + 1][x].is_none());
                self.tiles[y + 1][x] = Some(tile);
                None
            }
            MatchedSide::Left => {
                if x == 0 {
                    self.shift_all_right();
                    debug_assert!(self.tiles[y][x].is_none());
                    self.tiles[y][x] = Some(tile);
                    return Some((1, 0));
                }

                debug_assert!(self.tiles[y][x - 1].is_none());
                self.tiles[y][x - 1] = Some(tile);
                None
            }
        }
    }

    fn insert_tiles(&mut self, tiles: Vec<Tile>) {
        println!("{}", self);

        // firstly, allow us to take them arbitrarily
        let mut tiles: Vec<Option<Tile>> = tiles.into_iter().map(Some).collect();

        // find two tiles and their orientations such that they don't fit to any other tile

        // start with any tile we have
        self.tiles[0][0] = tiles.pop().unwrap();

        // println!("{}", self);

        loop {
            println!("{}", self);
            if tiles.iter().all(|tile| tile.is_none()) {
                break;
            }

            let mut tiles_to_place = Vec::new();

            for (y, row) in self.tiles.iter().enumerate() {
                for (x, existing_tile) in row.iter().enumerate().filter(|(_, tile)| tile.is_some())
                {
                    let tile = existing_tile.as_ref().unwrap();
                    for unmatched_tile in tiles.iter_mut().filter(|tile| tile.is_some()) {
                        if let Some(matched_side) = tile.try_match(unmatched_tile.as_mut().unwrap())
                        {
                            println!(
                                "{} matched with {}. {:?} to {:?}",
                                unmatched_tile.as_ref().unwrap().id,
                                tile.id,
                                matched_side,
                                (x, y)
                            );
                            tiles_to_place.push((
                                (x, y),
                                matched_side,
                                unmatched_tile.take().unwrap(),
                            ));
                        }
                    }
                }
            }

            let mut relative_delta = (0, 0);
            for (relative_position, matched_side, tile) in tiles_to_place {
                let true_relative = (
                    relative_position.0 + relative_delta.0,
                    relative_position.1 + relative_delta.1,
                );

                println!("gonna place {} at {:?}", tile.id, true_relative);

                if let Some(delta) = self.insert_tile(true_relative, matched_side, tile) {
                    println!("shift happened. delta: {:?}", delta);
                    relative_delta.0 = delta.0;
                    relative_delta.1 = delta.1;
                }
            }
        }

        // let mut tiles_to_place = Vec::new();
        //
        // for (y, row) in self.tiles.iter().enumerate() {
        //     for (x, existing_tile) in row.iter().filter(|tile| tile.is_some()).enumerate() {
        //         let tile = existing_tile.as_ref().unwrap();
        //         for unmatched_tile in tiles.iter_mut().filter(|tile| tile.is_some()) {
        //             if let Some(matched_side) = tile.try_match(unmatched_tile.as_mut().unwrap()) {
        //                 tiles_to_place.push(((x, y), matched_side, unmatched_tile.take().unwrap()));
        //             }
        //         }
        //     }
        // }
        //
        // // FFF if shift happened all is fucked
        // let mut relative_delta = (0, 0);
        // for (relative_position, matched_side, tile) in tiles_to_place {
        //     let true_relative = (
        //         relative_position.0 + relative_delta.0,
        //         relative_position.1 + relative_delta.1,
        //     );
        //     if let Some(delta) = self.insert_tile(true_relative, matched_side, tile) {
        //         relative_delta.0 = delta.0;
        //         relative_delta.1 = delta.1;
        //     }
        // }

        println!("{:?}", self);

        // let edges = self.tiles[0][0].as_ref().unwrap().edges();
        //
        // // for now, don't bother with whole thing, just find corners and multiply ids
        // let mut corner_product = 1;
        // 'outer: for tile1 in &tiles {
        //     let edges1 = tile1.edges();
        //     let mut tile1_matches = 0;
        //     for tile2 in &tiles {
        //         if tile1.id == tile2.id {
        //             continue;
        //         }
        //
        //         let edges2 = tile2.edges();
        //         if edges1.matches(&edges2) {
        //             tile1_matches += 1;
        //         }
        //         if tile1_matches > 2 {
        //             continue 'outer;
        //         }
        //     }
        //     if tile1_matches == 2 {
        //         corner_product *= tile1.id;
        //     }
        //     // rotate
        //     // flip h, flip v
        //     // rotate
        //     // flips
        //     // rotate
        //     // flips
        //
        //     // let other_edges = tile.edges();
        //     // edges.matches(&other_edges);
        //     // a,b,c,d
        //     // flip H:
        //     // flip V:
        //
        //     // rotate 1x:
        //     // flip H 1x:
        //     // flip V 1x:
        //
        //     // rotate 2x:
        // }

        // flip H -> rotate -> flip H
    }
}

impl Display for TileGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.tiles {
            let mut grid_rows = vec!["".to_string(); 10];
            let mut names = "".to_string();
            for tile in row {
                let tile = tile.as_ref().cloned().unwrap_or_else(Tile::unset);
                let tile_rows = tile.rows;
                for (i, actual_tile_row) in tile_rows.iter().enumerate() {
                    grid_rows[i] += &*(" ".to_owned()
                        + &*actual_tile_row
                            .iter()
                            .map(|&pixel| Into::<char>::into(pixel))
                            .collect::<String>())
                }
                // names += &*format!("    {}   ", tile.id);
            }

            // writeln!(f, "{}", names);
            for grid_row in grid_rows {
                writeln!(f, "{}", grid_row)?;
            }
            // writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for TileGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                if let Some(tile) = tile {
                    write!(f, "[{}]", tile.id)?;
                } else {
                    write!(f, "[????]")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
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
        println!("shifting down");
        self.tiles.rotate_right(1);
        // make sure first row is None now!
        assert!(self.tiles[0].iter().all(|tile| tile.is_none()));
    }

    fn shift_all_right(&mut self) {
        println!("shifting right");

        for row in self.tiles.iter_mut() {
            row.rotate_right(1);
            // make sure first element is a None!
            assert!(row[0].is_none());
        }
    }
}

fn part1(input: &[String]) -> usize {
    let mut tiles: Vec<_> = input.iter().map(Tile::from).collect();

    // let mut first = &mut tiles[0];
    //
    // println!("NORMAL: \n{}", first);
    // first.flip_horizontally();
    // println!("FLIP H: \n{}", first);
    // first.flip_horizontally();
    // first.flip_vertically();
    //
    // println!("FLIP V: \n{}", first);
    // first.flip_vertically();
    //
    // first.rotate_clockwise();
    // println!("ROTATE 90: \n{}", first);
    // first.rotate_clockwise();

    let side_len = (tiles.len() as f64).sqrt();
    // make sure it's a perfect square
    assert_eq!((side_len as usize).pow(2), tiles.len());

    let mut tile_grid = TileGrid::new(side_len as usize);
    tile_grid.insert_tiles(tiles);

    tile_grid.tiles[0][0].as_ref().unwrap().id
        * tile_grid.tiles[tile_grid.tiles.len() - 1][0]
            .as_ref()
            .unwrap()
            .id
        * tile_grid.tiles[tile_grid.tiles.len() - 1][tile_grid.tiles[0].len() - 1]
            .as_ref()
            .unwrap()
            .id
        * tile_grid.tiles[0][tile_grid.tiles[0].len() - 1]
            .as_ref()
            .unwrap()
            .id
}

// fn part2(input: &[String]) -> usize {
// 0
// }

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_into_string_groups("input").expect("failed to read input file");

    println!("{}", input[0]);

    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);

    // let part2_result = part2(&input);
    // println!("Part 2 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_tile_edges(edges: &Edges, tile: &Tile) {
        assert_eq!(edges.top, tile.row(0));
        assert_eq!(edges.bottom, tile.row(tile.rows.len() - 1));
        assert_eq!(edges.right, tile.column(tile.rows[0].len() - 1));
        assert_eq!(edges.left, tile.column(0));
    }

    #[test]
    fn edge_rotation_is_tile_consistent() {
        let raw_tile = r#"Tile 2311:
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
            .to_string();
        let mut tile = Tile::from(&raw_tile);
        let mut edges = tile.edges();
        compare_tile_edges(&edges, &tile);

        // for 90, 180, 270 rotation...
        for _ in 0..3 {
            edges.rotate_clockwise();
            tile.rotate_clockwise();
            compare_tile_edges(&edges, &tile);

            edges.flip_horizontally();
            tile.flip_horizontally();
            compare_tile_edges(&edges, &tile);

            edges.flip_horizontally();
            tile.flip_horizontally();
            compare_tile_edges(&edges, &tile);

            edges.flip_vertically();
            tile.flip_vertically();
            compare_tile_edges(&edges, &tile);

            edges.flip_horizontally();
            tile.flip_horizontally();
            compare_tile_edges(&edges, &tile);

            // reverse flips
            edges.flip_horizontally();
            tile.flip_horizontally();
            edges.flip_vertically();
            tile.flip_vertically();
            compare_tile_edges(&edges, &tile);
        }
    }

    #[test]
    fn tile_rotation() {
        let raw_tile = r#"Tile 2311:
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
            .to_string();
        let mut tile = Tile::from(&raw_tile);

        let expected_rotated = r#"Tile 2311:
.#..#####.
.#.####.#.
###...#..#
#..#.##..#
#....#.##.
...##.##.#
.#...#....
#.#.##....
##.###.#.#
#..##.#..."#
            .to_string();
        let rotated = Tile::from(&expected_rotated);

        tile.rotate_clockwise();
        assert_eq!(tile, rotated);
    }

    #[test]
    fn tile_horizontal_flip() {
        let raw_tile = r#"Tile 2311:
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
            .to_string();
        let mut tile = Tile::from(&raw_tile);

        let expected_flipped = r#"Tile 2311:
..###..###
###...#.#.
..#....#..
.#.#.#..##
##...#.###
##.##.###.
####.#...#
#...##..#.
##..#.....
..##.#..#."#
            .to_string();
        let flipped = Tile::from(&expected_flipped);

        tile.flip_horizontally();
        assert_eq!(tile, flipped);
    }

    #[test]
    fn tile_vertical_flip() {
        let raw_tile = r#"Tile 2311:
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
            .to_string();
        let mut tile = Tile::from(&raw_tile);

        let expected_flipped = r#"Tile 2311:
.#..#.##..
.....#..##
.#..##...#
#...#.####
.###.##.##
###.#...##
##..#.#.#.
..#....#..
.#.#...###
###..###.."#
            .to_string();
        let flipped = Tile::from(&expected_flipped);

        tile.flip_vertically();
        assert_eq!(tile, flipped);
    }

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
