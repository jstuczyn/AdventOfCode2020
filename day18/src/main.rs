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

use std::collections::{HashMap, VecDeque};
use std::fmt::{self, Display, Formatter};
use utils::input_read;

struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { inner: Vec::new() }
    }

    fn push(&mut self, val: T) {
        self.inner.push(val)
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    fn peek(&self) -> Option<&T> {
        if self.inner.is_empty() {
            None
        } else {
            Some(&self.inner[self.inner.len() - 1])
        }
    }
}

fn digits_to_number(digits: &[usize]) -> usize {
    digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, digit)| acc + 10usize.pow(idx as u32) * digit)
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().filter(|c| !c.is_whitespace()).peekable();

    let mut current_number_digits = Vec::new();
    while let Some(current) = iter.next() {
        let next = iter.peek();
        if current.is_numeric() {
            current_number_digits
                .push(current.to_digit(10).expect("failed to parse numeric!") as usize);
            if !match next {
                Some(next) => next.is_numeric(),
                None => false,
            } {
                let number = digits_to_number(&current_number_digits);
                current_number_digits = Vec::new();
                tokens.push(Token::Operand(number))
            }
        } else {
            tokens.push(Token::Operator(Operator::from(current)))
        };
    }

    tokens
}

fn shunting_yard(input: &str, custom_precedence: &HashMap<Operator, usize>) -> VecDeque<Token> {
    let mut queue = VecDeque::new();
    let mut stack: Stack<Operator> = Stack::new();

    let tokens = tokenize(input);
    for token in tokens {
        match token {
            Token::Operand(_) => queue.push_back(token),
            Token::Operator(operator) => match operator {
                Operator::LeftParen => stack.push(Operator::LeftParen),
                Operator::RightParen => {
                    while let Some(top) = stack.pop() {
                        if top == Operator::LeftParen {
                            break;
                        } else {
                            queue.push_back(Token::Operator(top))
                        }
                    }
                }
                _ => {
                    let precedence = custom_precedence
                        .get(&operator)
                        .copied()
                        .unwrap_or_else(|| operator.default_precedence());
                    while let Some(top) = stack.peek() {
                        let top_precedence = custom_precedence
                            .get(&top)
                            .copied()
                            .unwrap_or_else(|| top.default_precedence());

                        if top_precedence >= precedence {
                            let popped = stack.pop().unwrap();
                            queue.push_back(Token::Operator(popped))
                        } else {
                            break;
                        }
                    }
                    stack.push(operator)
                }
            },
        }
    }

    while let Some(operator) = stack.pop() {
        queue.push_back(Token::Operator(operator))
    }

    queue
}

fn calculate(mut rpn_queue: VecDeque<Token>) -> usize {
    // in our case we only have binary operators
    let mut stack = Vec::with_capacity(2);

    while let Some(token) = rpn_queue.pop_front() {
        match token {
            Token::Operand(num) => stack.push(num),
            Token::Operator(operator) => {
                if let Some(y) = stack.pop() {
                    if let Some(x) = stack.pop() {
                        stack.push(operator.apply(x, y));
                    }
                }
            }
        }
    }

    stack.pop().unwrap()
}

#[derive(Debug)]
enum Token {
    Operator(Operator),
    Operand(usize),
}

// no other operators are required by the specs
#[derive(Debug, PartialEq, Eq, Hash)]
enum Operator {
    Addition,
    Multiplication,
    LeftParen,
    RightParen,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Addition => write!(f, "+"),
            Operator::Multiplication => write!(f, "*"),
            Operator::LeftParen => write!(f, "("),
            Operator::RightParen => write!(f, ")"),
        }
    }
}

impl Operator {
    fn default_precedence(&self) -> usize {
        match self {
            Operator::LeftParen | Operator::RightParen => 0,
            Operator::Addition => 1,
            Operator::Multiplication => 2,
        }
    }

    fn apply(&self, x: usize, y: usize) -> usize {
        match self {
            Operator::Addition => x + y,
            Operator::Multiplication => x * y,
            _ => panic!("tried to apply parentheses operator!"),
        }
    }
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '+' => Operator::Addition,
            '*' => Operator::Multiplication,
            '(' => Operator::LeftParen,
            ')' => Operator::RightParen,
            v => panic!("invalid operator {}", v),
        }
    }
}

fn part1(input: &[String]) -> usize {
    let mut precedence = HashMap::new();
    precedence.insert(Operator::Multiplication, 1);

    input
        .iter()
        .map(|raw| shunting_yard(raw, &precedence))
        .map(calculate)
        .sum()
}

fn part2(input: &[String]) -> usize {
    let mut precedence = HashMap::new();
    precedence.insert(Operator::Multiplication, 1);
    precedence.insert(Operator::Addition, 2);

    input
        .iter()
        .map(|raw| shunting_yard(raw, &precedence))
        .map(calculate)
        .sum()
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
    fn digits_to_number() {
        assert_eq!(super::digits_to_number(&[1, 2, 3]), 123);
        assert_eq!(super::digits_to_number(&[1]), 1);
    }

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "2 * 3 + (4 * 5)".to_string(),
            "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(),
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(),
        ];

        let expected = 26335;

        assert_eq!(expected, part1(&input));
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "2 * 3 + (4 * 5)".to_string(),
            "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(),
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(),
        ];

        let expected = 693891;

        assert_eq!(expected, part2(&input));
    }
}
