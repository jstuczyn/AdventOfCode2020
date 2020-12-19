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

use utils::input_read;

#[derive(Debug)]
struct Grammar {
    rules: Vec<Option<Rule>>,
}

impl From<&str> for Grammar {
    fn from(rules: &str) -> Self {
        let unsorted_rules: Vec<_> = rules
            .split('\n')
            .map(|raw_rule| {
                let id_rule: Vec<_> = raw_rule.split(": ").collect();
                let rule_id: usize = id_rule[0].parse().expect("failed to parse rule id");
                let rule = Rule::from(id_rule[1]);
                (rule_id, rule)
            })
            .collect();

        let mut rules = vec![None; unsorted_rules.iter().map(|(id, _)| *id).max().unwrap() + 1];

        for (id, rule) in unsorted_rules.into_iter() {
            rules[id] = Some(rule)
        }

        Grammar { rules }
    }
}

impl Grammar {
    // perform recursive descent parsing
    // returns number of characters consumed by the rule on the word
    fn check_word_rule(&self, chars: &[char], input_rule: usize) -> Vec<usize> {
        if chars.is_empty() {
            return Vec::new();
        }

        match &self.rules[input_rule]
            .as_ref()
            .expect("the rule does not exist!")
        {
            Rule::Terminal(c) => {
                if &chars[0] == c {
                    vec![1]
                } else {
                    Vec::new()
                }
            }
            Rule::Nonterminal(rule) => {
                let mut used_by_subrules = Vec::new();

                for subrule in rule.subrules.iter() {
                    if subrule.len() > chars.len() {
                        // due to how substitutions work here, we must have at least n characters
                        // for n rules left
                        continue;
                    }
                    let mut subrule_consumed = vec![0];
                    for rule in subrule.iter() {
                        let mut new_thing = Vec::new();
                        for consumed in subrule_consumed {
                            let consumed_possibilities =
                                self.check_word_rule(&chars[consumed..], *rule);
                            if consumed_possibilities.is_empty() {
                                continue;
                            }
                            for consumed_pos in consumed_possibilities {
                                new_thing.push(consumed + consumed_pos)
                            }
                        }
                        subrule_consumed = new_thing;
                    }

                    // if we didn't consume anything, it means the subrule was invalid
                    if !subrule_consumed.is_empty() {
                        used_by_subrules.append(&mut subrule_consumed)
                    }
                }
                used_by_subrules
            }
        }
    }

    fn check_word(&self, word: &str) -> bool {
        let chars: Vec<_> = word.chars().collect();
        let num_consumed = self.check_word_rule(&chars, 0);
        if num_consumed.is_empty() {
            false
        } else {
            num_consumed[0] == chars.len()
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Terminal(char),
    Nonterminal(NonterminalRule),
}

type Subrule = Vec<usize>;

#[derive(Debug, Clone)]
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

fn do_part2_grammar_change(grammar: &mut Grammar) {
    grammar.rules[8] = Some(Rule::Nonterminal(NonterminalRule {
        subrules: vec![vec![42], vec![42, 8]],
    }));
    grammar.rules[11] = Some(Rule::Nonterminal(NonterminalRule {
        subrules: vec![vec![42, 31], vec![42, 11, 31]],
    }));
}

fn part2(input: &[String]) -> usize {
    let mut grammar = Grammar::from(&*input[0]);
    do_part2_grammar_change(&mut grammar);

    input[1]
        .split('\n')
        .filter(|word| grammar.check_word(word))
        .count()
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

    #[test]
    fn part2_sample_input() {
        let input = vec![
            r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"#
                .to_string(),
            r#"abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
                .to_string(),
        ];

        let expected_p1 = 3;
        let expected_p2 = 12;

        assert_eq!(expected_p1, part1(&input));
        assert_eq!(expected_p2, part2(&input));
    }
}
