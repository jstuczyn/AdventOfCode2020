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

    fn try_next_node(&self) -> Option<Rc<RefCell<Node>>> {
        if self.next.is_none() {
            None
        } else {
            Some(self.next_node())
        }
    }
}

struct NumRingBuffer {
    head: Rc<RefCell<Node>>,
    size: usize,
}

impl NumRingBuffer {
    fn new(mut data: Vec<usize>) -> Self {
        let size = data.len();

        let mut current = Rc::new(RefCell::new(Node {
            data: data.remove(0),
            next: None,
        }));

        let head = Rc::clone(&current);

        for node in data.into_iter().map(|num| {
            Rc::new(RefCell::new(Node {
                data: num,
                next: None,
            }))
        }) {
            current.borrow_mut().next = Some(Rc::clone(&node));
            current = node;
        }

        // last node:
        current.borrow_mut().next = Some(Rc::clone(&head));

        NumRingBuffer { head, size }
    }

    fn move_head(&mut self) {
        let next = self.head.borrow().next_node();
        self.head = next;
    }

    fn read_head(&self) -> usize {
        self.head.borrow().data
    }

    // fn read_head_offset(&self, offset: usize) -> usize {
    //     let mut node = &self.head;
    //     for _ in 0..offset {
    //         node = &node.next.as_ref().unwrap();
    //     }
    //     node.data
    // }

    fn order_as_num(&self) -> usize {
        let mut digits = Vec::with_capacity(self.size);
        // look for '1'
        let mut node = Rc::clone(&self.head);
        while node.borrow().data != 1 {
            let next = node.borrow().next_node();
            node = next;
        }
        // we have a '1' node. progress once
        let mut none_one = node.borrow().next_node();
        while none_one.borrow().data != 1 {
            digits.push(none_one.borrow().data);
            let next = none_one.borrow().next_node();
            none_one = next;
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
        let mut current = Rc::clone(&self.head);

        while current.borrow().data != val {
            let next = current.borrow().next_node();
            current = next;
        }
        // get the node to which our last three should point to
        let tail = current.borrow().next_node();

        let mut nodes_ptr = Rc::clone(&nodes);
        // make the target node point to our subchain
        current.borrow_mut().next = Some(nodes);

        // and finally redirect the subchain to the correct tail
        while nodes_ptr.borrow().next.is_some() {
            let next = nodes_ptr.borrow().next_node();
            nodes_ptr = next;
        }
        nodes_ptr.borrow_mut().next = Some(tail);
    }

    // // TODO: just TAKE IT?
    // fn next_three(&self) -> [usize; 3] {
    //     [
    //         self.read_head_offset(1),
    //         self.read_head_offset(2),
    //         self.read_head_offset(3),
    //     ]
    // }
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

    fn result(&self) -> usize {
        self.buf.order_as_num()
    }

    fn make_n_moves(&mut self, n: usize) {
        for i in 0..n {
            println!("-- move {} --", i + 1);
            self.make_move();
            println!("");
        }
    }

    // highest cup is always 9
    fn sub_one(val: usize) -> usize {
        let res = val - 1;
        if res == 0 {
            9
        } else {
            res
        }
    }

    fn select_destination_cup(&self) -> usize {
        let mut potential = Self::sub_one(self.buf.read_head());
        while potential == self.picked.as_ref().unwrap().1[0]
            || potential == self.picked.as_ref().unwrap().1[1]
            || potential == self.picked.as_ref().unwrap().1[2]
        {
            potential = Self::sub_one(potential)
        }

        potential
    }

    fn print_pickup(&self) {
        let first = Rc::clone(&self.picked.as_ref().unwrap().0);
        let second = first.borrow().next_node();
        let third = second.borrow().next_node();
        debug_assert!(third.borrow().next.is_none());

        println!(
            "pick up: {}, {}, {}",
            first.borrow().data,
            second.borrow().data,
            third.borrow().data
        );
    }

    fn make_move(&mut self) {
        println!("cups: {:?}", self.buf);
        self.picked = Some(self.buf.take_next_three());
        self.print_pickup();
        self.destination = self.select_destination_cup();
        println!("destination: {}", self.destination);

        let (next_nodes, _) = self.picked.take().unwrap();
        self.buf.insert_after(next_nodes, self.destination);
        self.buf.move_head();
    }
}

fn part1(input: usize) -> usize {
    let mut game = CrabGame::new(input);
    game.make_n_moves(100);
    game.result()
}

// fn part2(input: &[usize]) -> usize {
// 0
// }

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

    // let part2_result = part2(&input);
    // println!("Part 2 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_small_sample() {
        let input = 389125467;
        let mut game = CrabGame::new(input);
        game.make_move();
        let res = game.result();

        assert_eq!(54673289, res)
    }

    #[test]
    fn part1_small_sample2() {
        let input = 389125467;
        let mut game = CrabGame::new(input);
        game.make_n_moves(10);
        let res = game.result();

        assert_eq!(92658374, res)
    }

    // #[test]
    // fn part1_sample_input() {
    //     let input = 389125467;
    //     let expected = 67384529;
    //
    //     assert_eq!(expected, part1(input))
    // }
}
