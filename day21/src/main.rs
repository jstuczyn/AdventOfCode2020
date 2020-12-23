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

use std::collections::HashMap;
use utils::input_read;

type Ingredient = String;
type Allergen = String;

struct Allergens(HashMap<Ingredient, Allergen>);

fn split_into_ingredients_and_allergens(raw: &str) -> (Vec<&str>, Vec<&str>) {
    let split: Vec<_> = raw.split(" (contains ").collect();
    assert_eq!(2, split.len());
    let ingredients = split[0].split_ascii_whitespace().collect();
    let allergens = split[1].strip_suffix(')').unwrap().split(", ").collect();

    (ingredients, allergens)
}

fn find_allergen(
    possible_allergen_sources: &mut HashMap<&str, HashMap<&str, usize>>,
    known_allergens: &mut HashMap<Ingredient, Allergen>,
) {
    let mut discovered = Vec::new();
    for (allergen, possible_sources) in possible_allergen_sources.iter() {
        let mut most_common = vec![("", 0)];
        for (possible_source, count) in possible_sources.iter() {
            if known_allergens.contains_key(*possible_source) {
                // we already know it has different thing
                continue;
            }
            match *count {
                n if n > most_common[0].1 => most_common = vec![(possible_source, *count)],
                n if n == most_common[0].1 => most_common.push((possible_source, *count)),
                _ => (),
            }
        }

        if most_common.len() == 1 {
            discovered.push(allergen.to_string());
            known_allergens.insert(most_common[0].0.to_string(), allergen.to_string());
        }
    }
    if discovered.is_empty() {
        panic!("o-oh, we didn't find anything!");
    }
    for discovered_allergen in discovered {
        possible_allergen_sources.remove(&*discovered_allergen);
    }
}

impl From<&[(Vec<&str>, Vec<&str>)]> for Allergens {
    fn from(ingredient_allergens: &[(Vec<&str>, Vec<&str>)]) -> Self {
        let mut possible_allergen_sources = HashMap::new();
        for (ingredients, allergens) in ingredient_allergens {
            for allergen in allergens.iter() {
                for ingredient in ingredients.iter() {
                    let sources = possible_allergen_sources
                        .entry(*allergen)
                        .or_insert_with(HashMap::new);
                    *sources.entry(*ingredient).or_insert(0) += 1;
                }
            }
        }

        let mut known_allergens = HashMap::new();

        // let's pray to the old gods and the new that this doesn't get stuck in infinite loop
        loop {
            find_allergen(&mut possible_allergen_sources, &mut known_allergens);
            if possible_allergen_sources.is_empty() {
                break;
            }
        }

        Allergens(known_allergens)
    }
}

fn part1(input: &[String]) -> usize {
    let ingredient_allergens: Vec<_> = input
        .iter()
        .map(|raw| split_into_ingredients_and_allergens(raw))
        .collect();
    let allergens = Allergens::from(&*ingredient_allergens);

    let mut safe_ingredients = 0;
    for (ingredients, _) in ingredient_allergens {
        for ingredient in ingredients {
            if !allergens.0.contains_key(ingredient) {
                safe_ingredients += 1
            }
        }
    }

    safe_ingredients
}

fn part2(input: &[String]) -> String {
    let ingredient_allergens: Vec<_> = input
        .iter()
        .map(|raw| split_into_ingredients_and_allergens(raw))
        .collect();
    let allergens = Allergens::from(&*ingredient_allergens);

    let mut ingredients: Vec<_> = allergens.0.into_iter().collect();

    ingredients.sort_by(|(_, a1), (_, a2)| a1.cmp(a2));
    ingredients
        .into_iter()
        .map(|(k, _)| k)
        .collect::<Vec<_>>()
        .join(",")
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
    fn part1_sample_input() {
        let input = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];

        let expected = 5;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];

        let expected = "mxmxvkd,sqjhc,fvjkl";

        assert_eq!(expected, part2(&input))
    }
}
