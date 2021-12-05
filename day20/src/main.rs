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

impl From<Pixel> for char {
    fn from(pixel: Pixel) -> Self {
        match pixel {
            // use those unicode characters instead of the original ones
            // for way better readability
            Pixel::Active => '■',   //'ACTIVE_PIXEL,
            Pixel::Inactive => '□', //INACTIVE_PIXEL,
        }
    }
}

impl Pixel {
    fn is_active(&self) -> bool {
        matches!(self, Pixel::Active)
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
        if n == 0 {
            return;
        }
        for _ in 0..n {
            self.rotate_clockwise();
        }
    }

    fn try_match(&self, other: &mut Self) -> Option<MatchedSide> {
        let self_edges = self.edges();
        let mut other_edges = other.edges();
        if !self_edges.matches(&other_edges) {
            // see if the match is even possible
            return None;
        }

        // and only then determine correct orientation of the other tile

        if let Some(side) = self_edges.direct_match(&other_edges) {
            // println!("no rotation on {:?}", side);
            return Some(side);
        }

        // for 90, 180, 270 rotation...
        for i in 1..=3 {
            other_edges.rotate_clockwise();
            if let Some(side) = self_edges.direct_match(&other_edges) {
                other.rotate_n_clockwise(i);
                return Some(side);
            }
        }
        // reset
        other_edges.rotate_clockwise();

        other_edges.flip_horizontally();
        if let Some(side) = self_edges.direct_match(&other_edges) {
            other.flip_horizontally();
            return Some(side);
        }

        for i in 1..=3 {
            other_edges.rotate_clockwise();
            if let Some(side) = self_edges.direct_match(&other_edges) {
                other.flip_horizontally();
                other.rotate_n_clockwise(i);
                return Some(side);
            }
        }

        unreachable!("we determined a match was possible!")
    }

    fn remove_border(self) -> Vec<Vec<Pixel>> {
        let num_rows = self.rows.len();
        self.rows
            .into_iter()
            .skip(1)
            .take(num_rows - 2)
            .map(|row| row.into_iter().skip(1).take(num_rows - 2).collect())
            .collect()
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
        // firstly, allow us to take them arbitrarily
        let mut tiles: Vec<Option<Tile>> = tiles.into_iter().map(Some).collect();

        // find two tiles and their orientations such that they don't fit to any other tile

        // start with any tile we have
        self.tiles[0][0] = tiles.pop().unwrap();

        loop {
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

                if let Some(delta) = self.insert_tile(true_relative, matched_side, tile) {
                    relative_delta.0 = delta.0;
                    relative_delta.1 = delta.1;
                }
            }
        }
    }
}

impl Display for TileGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.tiles {
            let mut grid_rows = vec![" ".to_string(); 10];
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
            }

            for grid_row in grid_rows {
                writeln!(f, "{}", grid_row)?;
            }
            writeln!(f)?;
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
        self.tiles.rotate_right(1);
        // make sure first row is None now!
        assert!(self.tiles[0].iter().all(|tile| tile.is_none()));
    }

    fn shift_all_right(&mut self) {
        for row in self.tiles.iter_mut() {
            row.rotate_right(1);
            // make sure first element is a None!
            assert!(row[0].is_none());
        }
    }
}

struct Image {
    rows: Vec<Vec<Pixel>>,
}

impl From<TileGrid> for Image {
    fn from(grid: TileGrid) -> Self {
        let per_tile_rows = grid.tiles[0][0].as_ref().unwrap().rows.len() - 2;
        let num_rows = grid.tiles.len() * (per_tile_rows);
        let mut rows = Vec::with_capacity(num_rows);
        for row in grid.tiles.into_iter() {
            let mut grid_rows = vec![Vec::with_capacity(num_rows); per_tile_rows];
            for tile in row.into_iter().map(|tile| tile.unwrap()) {
                let borderless = tile.remove_border();
                for (i, mut tile_row) in borderless.into_iter().enumerate() {
                    grid_rows[i].append(&mut tile_row)
                }
            }
            rows.append(&mut grid_rows);
        }

        Image { rows }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl Image {
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

    fn column(&self, idx: usize) -> Vec<Pixel> {
        self.rows.iter().map(|row| row[idx]).collect()
    }

    fn check_for_monster(&self, base: (usize, usize)) -> Option<bool> {
        let monster = monster();

        for (y, row) in monster.iter().enumerate() {
            for x in row.iter() {
                if base.1 + y >= self.rows.len() || base.0 + *x >= self.rows.len() {
                    return None;
                }
                if !self.rows[base.1 + y][base.0 + *x].is_active() {
                    return Some(false);
                }
            }
        }

        Some(true)
    }

    fn find_monsters_in_orientation(&self) -> usize {
        let mut monsters = 0;
        // right now just completely ignore Nones

        for (y, row) in self.rows.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if let Some(res) = self.check_for_monster((x, y)) {
                    if res {
                        monsters += 1;
                    }
                } else {
                    break;
                }
            }
        }

        monsters
    }

    fn find_monsters(&mut self) -> usize {
        let monsters = self.find_monsters_in_orientation();
        if monsters != 0 {
            return monsters;
        }

        // for 90, 180, 270 rotation...
        for _ in 1..=3 {
            self.rotate_clockwise();
            let monsters = self.find_monsters_in_orientation();
            if monsters != 0 {
                return monsters;
            }
        }
        // reset
        self.rotate_clockwise();

        self.flip_horizontally();

        let monsters = self.find_monsters_in_orientation();
        if monsters != 0 {
            return monsters;
        }
        for _ in 1..=3 {
            self.rotate_clockwise();
            let monsters = self.find_monsters_in_orientation();
            if monsters != 0 {
                return monsters;
            }
        }

        unreachable!("there are no monsters in the image!!")
    }

    fn active_count(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.iter().filter(|pixel| pixel.is_active()).count())
            .sum()
    }
}

#[inline]
fn monster() -> [Vec<usize>; 3] {
    /*
                      #
    #    ##    ##    ###
     #  #  #  #  #  #
        */

    // row1: 18
    // row2: 0,5 ,6, 11, 12, 17, 18, 19
    // row3: 1,4 7, 10, 13, 16
    [
        vec![18],
        vec![0, 5, 6, 11, 12, 17, 18, 19],
        vec![1, 4, 7, 10, 13, 16],
    ]
}

fn part1(input: &[String]) -> usize {
    let tiles: Vec<_> = input.iter().map(Tile::from).collect();

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

fn part2(input: &[String]) -> usize {
    let tiles: Vec<_> = input.iter().map(Tile::from).collect();

    let side_len = (tiles.len() as f64).sqrt();
    // make sure it's a perfect square
    assert_eq!((side_len as usize).pow(2), tiles.len());

    let mut tile_grid = TileGrid::new(side_len as usize);
    tile_grid.insert_tiles(tiles);

    let mut image = Image::from(tile_grid);
    let monsters = image.find_monsters();

    // 15 is the number of tiles used by a single monster
    image.active_count() - 15 * monsters
}

#[cfg(not(tarpaulin))]
fn main() {
    let input = input_read::read_into_string_groups("input").expect("failed to read input file");

    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);

    let part2_result = part2(&input);
    println!("Part 2 result is {}", part2_result);
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

            edges.flip_horizontally();
            tile.flip_horizontally();
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

    #[test]
    fn part2_sample_input() {
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

        let expected = 273;

        assert_eq!(expected, part2(&input));
    }
}
