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

use std::collections::{HashSet, VecDeque};
use utils::input_read;

#[derive(Debug)]
struct Player {
    deck: VecDeque<usize>,
}

impl From<&String> for Player {
    fn from(raw: &String) -> Self {
        // skip the "Player 1:" line
        let deck = raw
            .lines()
            .skip(1)
            .map(|card| card.parse().expect("failed to parse the card"))
            .collect();

        Player { deck }
    }
}

impl Player {
    fn play_card(&mut self) -> usize {
        self.deck
            .pop_front()
            .expect("tried to play a card from an empty deck!")
    }

    fn peek_next(&self) -> Option<&usize> {
        self.deck.get(0)
    }

    fn cards_left(&self) -> usize {
        self.deck.len()
    }

    fn play_round(&mut self, other: &mut Self) -> Option<bool> {
        if self.peek_next().is_none() {
            // player self lost
            return Some(false);
        }
        if other.peek_next().is_none() {
            // player self won
            return Some(true);
        }

        let played1 = self.play_card();
        let played2 = other.play_card();

        if played1 > played2 {
            self.insert_won((played1, played2));
        } else {
            other.insert_won((played2, played1));
        }

        None
    }

    fn insert_won(&mut self, cards: (usize, usize)) {
        self.deck.push_back(cards.0);
        self.deck.push_back(cards.1);
    }

    fn calculate_final_score(&self) -> usize {
        self.deck
            .iter()
            .rev()
            .enumerate()
            .map(|(i, card)| (i + 1) * *card)
            .sum()
    }
}

struct RecursiveGame {
    player1: Player,
    player2: Player,

    // if there was a previous round in this game that had
    // exactly the same cards in the same order in the same players' decks,
    // the game instantly ends in a win for player 1.
    previously_played: HashSet<(usize, usize)>,
    // If both players have at least as many cards
    // remaining in their deck as the value of the card they just drew,
    // the winner of the round is determined by playing a new game of Recursive Combat (see below).

    // Otherwise, at least one player must not have enough cards left in their deck to recurse;
    // the winner of the round is the player with the higher-value card.
}

fn part1(input: &[String]) -> usize {
    let mut player1 = Player::from(&input[0]);
    let mut player2 = Player::from(&input[1]);

    loop {
        if let Some(result) = player1.play_round(&mut player2) {
            return if result {
                player1.calculate_final_score()
            } else {
                player2.calculate_final_score()
            };
        }
    }
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
            r#"Player 1:
9
2
6
3
1"#
            .to_string(),
            r#"Player 2:
5
8
4
7
10"#
            .to_string(),
        ];

        let expected = 306;

        assert_eq!(expected, part1(&input));
    }
}
