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

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;
use utils::input_read;

const EMPTY_BAG: &str = "no other";
const TARGET_BAG: &str = "shiny gold";

struct Part1Traversal<'a> {
    visited: HashSet<String>,
    graph: &'a BagGraph,
}

impl<'a> Part1Traversal<'a> {
    fn new(graph: &'a BagGraph) -> Self {
        Part1Traversal {
            visited: HashSet::new(),
            graph,
        }
    }

    fn visit_entry(&mut self, entry: &Rc<RefCell<Bag>>) {
        for parent in &entry.borrow().parents {
            if self.visited.insert(parent.borrow().name.clone()) {
                self.visit_entry(parent);
            }
        }
    }

    fn get_unique_parents(&mut self, bag_name: &str) -> HashSet<String> {
        let entry = self.graph.nodes.get(bag_name).unwrap();

        for parent in &entry.borrow().parents {
            self.visited.insert(parent.borrow().name.clone());
            self.visit_entry(parent)
        }

        self.visited.clone()
    }
}

struct Part2Traversal<'a> {
    graph: &'a BagGraph,
}

impl<'a> Part2Traversal<'a> {
    fn new(graph: &'a BagGraph) -> Self {
        Part2Traversal { graph }
    }

    fn visit_entry(&mut self, entry: &Rc<RefCell<Bag>>) -> usize {
        let mut branch_bags = 1;

        for child in &entry.borrow().inner {
            branch_bags += child.count * self.visit_entry(&child.bag);
        }

        branch_bags
    }

    fn get_total_bags(&mut self, bag_name: &str) -> usize {
        let entry = self.graph.nodes.get(bag_name).unwrap();
        self.visit_entry(entry) - 1
    }
}

#[derive(Default)]
struct BagGraph {
    nodes: HashMap<String, Rc<RefCell<Bag>>>,
}

impl BagGraph {
    fn new() -> Self {
        Default::default()
    }

    fn insert_rule(&mut self, bag_name: String, inner: Vec<BagInnerRaw>) {
        let mut inner_bags = Vec::with_capacity(inner.len());
        let mut inserted_inner_bags = Vec::with_capacity(inner.len());

        for (inner_bag_count, inner_bag_name) in inner {
            // inner rcs
            let node = self
                .nodes
                .entry(inner_bag_name.clone())
                .or_insert_with(|| Rc::new(RefCell::new(Bag::new(inner_bag_name))));

            inner_bags.push(BagInner {
                count: inner_bag_count,
                bag: Rc::clone(&node),
            });

            inserted_inner_bags.push(Rc::clone(&node));
        }

        let parent = self
            .nodes
            .entry(bag_name.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Bag::new(bag_name.clone()))));

        // it can't have been set before
        debug_assert!(parent.borrow().inner.is_empty());
        parent.borrow_mut().set_inner(inner_bags);

        for inner_bag in inserted_inner_bags {
            inner_bag.borrow_mut().add_parent(Rc::clone(&parent))
        }
    }
}

impl Debug for BagGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "BAG GRAPH")?;
        for (name, children) in self.nodes.iter() {
            writeln!(f, "{}:", name)?;
            for child in &children.borrow().inner {
                writeln!(f, "\t{} x {}", child.count, child.bag.borrow().name)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Bag {
    // each bag has a name/colour
    name: String,
    // and can have multiple other bags inside it
    inner: Vec<BagInner>,
    // and can have multiple parents
    parents: Vec<Rc<RefCell<Bag>>>,
}

impl Bag {
    fn new(name: String) -> Self {
        Bag {
            name,
            inner: Vec::new(),
            parents: Vec::new(),
        }
    }

    fn add_parent(&mut self, parent: Rc<RefCell<Bag>>) {
        self.parents.push(parent)
    }

    fn set_inner(&mut self, inner: Vec<BagInner>) {
        self.inner = inner
    }
}

type BagInnerRaw = (usize, String);

struct BagInner {
    count: usize,
    bag: Rc<RefCell<Bag>>,
}

impl Debug for BagInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} x {}", self.count, self.bag.borrow().name)
    }
}

fn into_count_and_name(raw_bag: &str) -> Option<BagInnerRaw> {
    if raw_bag.starts_with(EMPTY_BAG) {
        return None;
    }
    let whitespace_split: Vec<_> = raw_bag.split_ascii_whitespace().collect();
    // we must at least have a count and name
    assert!(whitespace_split.len() >= 2);
    let count = whitespace_split[0]
        .parse()
        .expect("failed to parse bag count!");

    // final chunk is word 'bag' or 'bags' and we don't care about it
    let name = whitespace_split[1..whitespace_split.len() - 1].join(" ");
    Some((count, name))
}

fn parse_rule(rule: &str) -> (String, Vec<BagInnerRaw>) {
    let split: Vec<_> = rule.split("contain").collect();
    let bag_name = split[0].trim().trim_end_matches(" bags").to_owned();

    let inner_bags = split[1].split(|c| c == ',' || c == '.');
    let raw_inner = inner_bags
        .map(str::trim)
        .filter(|bag| !bag.is_empty())
        .map(into_count_and_name)
        .filter_map(|entry| entry)
        .collect();

    (bag_name, raw_inner)
}

fn part1(input: &[String]) -> usize {
    let mut graph = BagGraph::new();
    input
        .iter()
        .map(|rule| parse_rule(rule))
        .for_each(|rule| graph.insert_rule(rule.0, rule.1));

    Part1Traversal::new(&graph)
        .get_unique_parents(TARGET_BAG)
        .len()
}

fn part2(input: &[String]) -> usize {
    let mut graph = BagGraph::new();
    input
        .iter()
        .map(|rule| parse_rule(rule))
        .for_each(|rule| graph.insert_rule(rule.0, rule.1));

    Part2Traversal::new(&graph).get_total_bags(TARGET_BAG)
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
    fn sample_part1_input() {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];

        let expected = 4;

        assert_eq!(expected, part1(&input));
    }

    #[test]
    fn sample_part2_input1() {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];

        let expected = 32;

        assert_eq!(expected, part2(&input));
    }

    #[test]
    fn sample_part2_input2() {
        let input = vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ];

        let expected = 126;

        assert_eq!(expected, part2(&input));
    }
}
