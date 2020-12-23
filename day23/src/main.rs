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
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

#[derive(Default)]
struct Node {
    data: usize,
    // it can only be a `None` if it got detached while moving it to new location
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    // assumes next MUST exist
    fn next_node(&self) -> Rc<RefCell<Node>> {
        Rc::clone(self.next.as_ref().unwrap())
    }
}

struct NumRingBuffer {
    head: Rc<RefCell<Node>>,
    lookup_cache: HashMap<usize, Rc<RefCell<Node>>>,
    size: usize,
}

impl NumRingBuffer {
    fn new(mut data: Vec<usize>) -> Self {
        let size = data.len();
        let mut lookup_cache = HashMap::new();

        let mut current = Rc::new(RefCell::new(Node {
            data: data.remove(0),
            next: None,
        }));

        lookup_cache.insert(current.borrow().data, Rc::clone(&current));

        let head = Rc::clone(&current);

        for node in data.into_iter().map(|num| {
            Rc::new(RefCell::new(Node {
                data: num,
                next: None,
            }))
        }) {
            current.borrow_mut().next = Some(Rc::clone(&node));
            lookup_cache.insert(node.borrow().data, Rc::clone(&node));
            current = node;
        }

        // last node:
        current.borrow_mut().next = Some(Rc::clone(&head));

        NumRingBuffer {
            head,
            size,
            lookup_cache,
        }
    }

    fn new_million(mut init_data: Vec<usize>) -> Self {
        let size = 1_000_000;
        let mut lookup_cache = HashMap::new();

        let mut current = Rc::new(RefCell::new(Node {
            data: init_data.remove(0),
            next: None,
        }));

        lookup_cache.insert(current.borrow().data, Rc::clone(&current));

        let head = Rc::clone(&current);

        // note: we're taking 999_999 elements as we've already consumed one
        for node in init_data
            .into_iter()
            .chain(std::iter::successors(Some(10), |first| Some(*first + 1)))
            .take(999999)
            .map(|num| {
                Rc::new(RefCell::new(Node {
                    data: num,
                    next: None,
                }))
            })
        {
            current.borrow_mut().next = Some(Rc::clone(&node));
            lookup_cache.insert(node.borrow().data, Rc::clone(&node));
            current = node;
        }

        // last node:
        current.borrow_mut().next = Some(Rc::clone(&head));

        NumRingBuffer {
            head,
            size,
            lookup_cache,
        }
    }

    fn move_head(&mut self) {
        let next = self.head.borrow().next_node();
        self.head = next;
    }

    fn read_head(&self) -> usize {
        self.head.borrow().data
    }

    fn find_one_node(&self) -> Rc<RefCell<Node>> {
        Rc::clone(self.lookup_cache.get(&1).unwrap())
    }

    fn part1_result(&self) -> usize {
        let mut digits = Vec::with_capacity(self.size);
        // look for '1'
        let one_node = self.find_one_node();

        // we have a '1' node. progress once
        let mut not_one = one_node.borrow().next_node();
        while not_one.borrow().data != 1 {
            digits.push(not_one.borrow().data);
            let next = not_one.borrow().next_node();
            not_one = next;
        }

        digits
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, digit)| acc + 10usize.pow(idx as u32) * digit)
    }

    fn take_next_three(&mut self) -> (Rc<RefCell<Node>>, [usize; 3]) {
        let next = self.head.borrow_mut().next.take().unwrap();
        let mut picked_values = [0; 3];

        let mut current = Rc::clone(&next);

        // determine the fourth element
        // ignore clippy as I can't be bothered to implement iterator for the nodes
        #[allow(clippy::needless_range_loop)]
        for i in 0..3 {
            picked_values[i] = current.borrow().data;
            let next = current.borrow().next_node();
            if i == 2 {
                // clear the 'next' of the third element we're removing
                current.borrow_mut().next = None;
            }
            current = next;
        }

        // point to the fourth element
        self.head.borrow_mut().next = Some(current);

        (next, picked_values)
    }

    fn insert_after(&mut self, nodes: Rc<RefCell<Node>>, val: usize) {
        let insertion_target = self.lookup_cache.get(&val).unwrap();

        // get the node to which our last three should point to
        let tail = insertion_target.borrow().next_node();

        let mut nodes_ptr = Rc::clone(&nodes);
        // make the target node point to our subchain
        insertion_target.borrow_mut().next = Some(nodes);

        // and finally redirect the subchain to the correct tail
        while nodes_ptr.borrow().next.is_some() {
            let next = nodes_ptr.borrow().next_node();
            nodes_ptr = next;
        }
        nodes_ptr.borrow_mut().next = Some(tail);
    }
}

impl Debug for NumRingBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let head = self.head.borrow().data;
        let mut current = Rc::clone(&self.head);
        let mut values = vec![current.borrow().data];

        let next = current.borrow().next_node();
        current = next;

        while current.borrow().data != head {
            values.push(current.borrow().data);
            let next = current.borrow().next_node();
            current = next;
        }

        write!(f, "head: {}, {:?}", self.head.borrow().data, values)
    }
}

struct CrabGame {
    buf: NumRingBuffer,
    picked: Option<(Rc<RefCell<Node>>, [usize; 3])>,
    destination: usize,
}

impl CrabGame {
    fn new(input: usize) -> CrabGame {
        let split_input = split_into_digits(input);
        let buf = NumRingBuffer::new(split_input);

        CrabGame {
            buf,
            picked: None,
            destination: 0,
        }
    }

    fn new_million(input: usize) -> CrabGame {
        let split_input = split_into_digits(input);
        let buf = NumRingBuffer::new_million(split_input);

        CrabGame {
            buf,
            picked: None,
            destination: 0,
        }
    }

    fn part1_result(&self) -> usize {
        self.buf.part1_result()
    }

    fn part2_result(&self) -> usize {
        let one_node = self.buf.find_one_node();
        let next = one_node.borrow().next_node();
        let label1 = next.borrow().data;
        let label2 = next.borrow().next_node().borrow().data;
        label1 * label2
    }

    fn make_n_moves(&mut self, n: usize) {
        for _ in 0..n {
            self.make_move();
        }
    }

    fn sub_one(&self, val: usize) -> usize {
        let res = val - 1;
        if res == 0 {
            self.buf.size
        } else {
            res
        }
    }

    fn select_destination_cup(&self) -> usize {
        let mut potential = self.sub_one(self.buf.read_head());
        while potential == self.picked.as_ref().unwrap().1[0]
            || potential == self.picked.as_ref().unwrap().1[1]
            || potential == self.picked.as_ref().unwrap().1[2]
        {
            potential = self.sub_one(potential)
        }

        potential
    }

    fn make_move(&mut self) {
        self.picked = Some(self.buf.take_next_three());
        self.destination = self.select_destination_cup();

        let (next_nodes, _) = self.picked.take().unwrap();
        self.buf.insert_after(next_nodes, self.destination);
        self.buf.move_head();
    }
}

fn part1(input: usize) -> usize {
    let mut game = CrabGame::new(input);
    game.make_n_moves(100);
    game.part1_result()
}

// this is not included in coverage for the same reason as the part2 test
#[cfg(not(tarpaulin))]
fn part2(input: usize) -> usize {
    let mut game = CrabGame::new_million(input);
    game.make_n_moves(10_000_000);
    game.part2_result()
}

fn split_into_digits(number: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    let mut n = number;
    while n > 9 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

#[cfg(not(tarpaulin))]
fn main() {
    let input = 364289715;

    let part1_result = part1(input);
    println!("Part 1 result is {}", part1_result);

    let part2_result = part2(input);
    println!("Part 2 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_small_sample() {
        let input = 389125467;
        let mut game = CrabGame::new(input);
        game.make_move();
        let res = game.part1_result();

        assert_eq!(54673289, res)
    }

    #[test]
    fn part1_small_sample2() {
        let input = 389125467;
        let mut game = CrabGame::new(input);
        game.make_n_moves(10);
        let res = game.part1_result();

        assert_eq!(92658374, res)
    }

    #[test]
    fn part1_sample_input() {
        let input = 389125467;
        let expected = 67384529;

        assert_eq!(expected, part1(input))
    }

    // the below test passes, however, it is not committed as because is it run under
    // `debug` release profile (and I can't be bothered to change that) it take too long to
    // complete

    // #[test]
    // fn part2_sample_input() {
    //     let input = 389125467;
    //     let expected = 149245887792;
    //
    //     assert_eq!(expected, part2(input))
    // }
}
