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

// Since our rules represent a finite regular language, we can either
// - enumerate all words since it's finite
// - use a regex since it's regular (or a FSM)
// - or do the simplest recursive descent parsing

use utils::input_read;

#[derive(Debug)]
struct Grammar {
    rules: Vec<Rule>,
}

impl From<&str> for Grammar {
    fn from(rules: &str) -> Self {
        let mut unsorted_rules: Vec<_> = rules
            .split('\n')
            .map(|raw_rule| {
                let id_rule: Vec<_> = raw_rule.split(": ").collect();
                let rule_id: usize = id_rule[0].parse().expect("failed to parse rule id");
                let rule = Rule::from(id_rule[1]);
                (rule_id, rule)
            })
            .collect();

        unsorted_rules.sort_by(|(id1, _), (id2, _)| id1.cmp(id2));

        Grammar {
            rules: unsorted_rules.into_iter().map(|(_, rule)| rule).collect(),
        }
    }
}

impl Grammar {
    // perform recursive descent parsing
    // returns whether the word is valid for particular rule and number of characters consumed
    fn check_word_rule(&self, chars: &[char], rule: usize) -> (bool, usize) {
        // println!(
        //     "checking rule {} ({:?}) for {:?}",
        //     rule, self.rules[rule], chars
        // );
        match &self.rules[rule] {
            Rule::Terminal(c) => (&chars[0] == c, 1),
            Rule::Nonterminal(rule) => {
                // rule.subrules.iter().any(|subrule| subrule.iter().enumerate().all(|(i, rule)| self.check_word(&chars[i..], *rule)));

                for subrule in rule.subrules.iter() {
                    let mut subrule_consumed = 0;
                    for rule in subrule.iter() {
                        let (valid, consumed) =
                            self.check_word_rule(&chars[subrule_consumed..], *rule);
                        if valid {
                            subrule_consumed += consumed
                        } else {
                            // all rules must be valid
                            subrule_consumed = 0;
                            break;
                        }
                    }

                    // if we didn't consume anything, it means the subrule was invalid
                    if subrule_consumed != 0 {
                        return (true, subrule_consumed);
                    }
                }
                // no subrule is valid
                (false, 0)
            }
        }
    }

    fn check_word(&self, word: &str) -> bool {
        let chars: Vec<_> = word.chars().collect();
        let (valid, num_consumed) = self.check_word_rule(&chars, 0);
        valid && num_consumed == chars.len()
    }
}

#[derive(Debug)]
enum Rule {
    Terminal(char),
    Nonterminal(NonterminalRule),
}

type Subrule = Vec<usize>;

#[derive(Debug)]
struct NonterminalRule {
    subrules: Vec<Subrule>,
}

impl From<&str> for Rule {
    fn from(raw: &str) -> Self {
        // at most there are 2 subrules
        let mut subrules = Vec::with_capacity(2);
        for subrule in raw.split(" | ") {
            let mut rules = Vec::new();
            for rule in subrule.split_ascii_whitespace() {
                if let Ok(rule_id) = rule.parse() {
                    rules.push(rule_id)
                } else {
                    return Rule::Terminal(rule.chars().nth(1).unwrap());
                }
            }
            subrules.push(rules)
        }

        Rule::Nonterminal(NonterminalRule { subrules })
    }
}

fn part1(input: &[String]) -> usize {
    let grammar = Grammar::from(&*input[0]);

    input[1]
        .split('\n')
        .filter(|word| grammar.check_word(word))
        .count()
}

// fn part2(input: &[String]) -> usize {
//     0
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
    fn part1_small_sample() {
        let input = vec![
            r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b""#
                .to_string(),
            r#"aab
aba"#
                .to_string(),
        ];

        let expected = 2;

        assert_eq!(expected, part1(&input));
    }

    #[test]
    fn part1_sample_input() {
        let input = vec![
            r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#
                .to_string(),
            r#"ababbb
bababa
abbbab
aaabbb
aaaabbb"#
                .to_string(),
        ];

        let expected = 2;

        assert_eq!(expected, part1(&input));
    }
}
