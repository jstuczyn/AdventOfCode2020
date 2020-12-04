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
use std::convert::TryFrom;
use utils::input_read;

const BIRTH_YEAR_ABBREVIATION: &str = "byr";
const ISSUE_YEAR_ABBREVIATION: &str = "iyr";
const EXPIRATION_YEAR_ABBREVIATION: &str = "eyr";
const HEIGHT_ABBREVIATION: &str = "hgt";
const HAIR_COLOR_ABBREVIATION: &str = "hcl";
const EYE_COLOR_ABBREVIATION: &str = "ecl";
const PASSPORT_ID_ABBREVIATION: &str = "pid";
const COUNTRY_ID_ABBREVIATION: &str = "cid";

const MANDATORY_FIELDS: [&str; 7] = [
    BIRTH_YEAR_ABBREVIATION,
    ISSUE_YEAR_ABBREVIATION,
    EXPIRATION_YEAR_ABBREVIATION,
    HEIGHT_ABBREVIATION,
    HAIR_COLOR_ABBREVIATION,
    EYE_COLOR_ABBREVIATION,
    PASSPORT_ID_ABBREVIATION,
];

const MISSING_FIELD_STR: &str = "MISSING";
const MISSING_FIELD_NUM: isize = -1;

#[derive(Debug)]
enum MalformedPassport {
    MissingField(Vec<&'static str>),
    MalformedData(String),
}

struct Passport {
    birth_year: isize,
    issue_year: isize,
    expiration_year: isize,
    height: String,
    hair_color: String,
    eye_color: String,
    // my input data has one case of `pid:192cm`
    country_id: Option<isize>,
    passport_id: String,
}

impl Default for Passport {
    fn default() -> Self {
        Passport {
            birth_year: MISSING_FIELD_NUM,
            issue_year: MISSING_FIELD_NUM,
            expiration_year: MISSING_FIELD_NUM,
            height: MISSING_FIELD_STR.to_string(),
            hair_color: MISSING_FIELD_STR.to_string(),
            eye_color: MISSING_FIELD_STR.to_string(),
            passport_id: MISSING_FIELD_STR.to_string(),
            country_id: None,
        }
    }
}

impl<'a> TryFrom<&'a str> for Passport {
    type Error = MalformedPassport;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let fields = value.split_ascii_whitespace();

        let mut default_passport = Passport::default();

        for field in fields {
            let name_value = field.split(":").collect::<Vec<_>>();
            if name_value.len() != 2 {
                return Err(MalformedPassport::MalformedData(format!(
                    "invalid field: {}",
                    field
                )));
            }

            let name = name_value[0];
            let value = name_value[1];

            match name {
                BIRTH_YEAR_ABBREVIATION => {
                    default_passport.birth_year = value
                        .parse()
                        .map_err(|_| MalformedPassport::MalformedData("birth year".to_string()))?
                }
                ISSUE_YEAR_ABBREVIATION => {
                    default_passport.issue_year = value
                        .parse()
                        .map_err(|_| MalformedPassport::MalformedData("issue year".to_string()))?
                }
                EXPIRATION_YEAR_ABBREVIATION => {
                    default_passport.expiration_year = value.parse().map_err(|_| {
                        MalformedPassport::MalformedData("expiration year".to_string())
                    })?
                }
                HEIGHT_ABBREVIATION => default_passport.height = value.to_string(),
                HAIR_COLOR_ABBREVIATION => default_passport.hair_color = value.to_string(),
                EYE_COLOR_ABBREVIATION => default_passport.eye_color = value.to_string(),
                PASSPORT_ID_ABBREVIATION => default_passport.passport_id = value.to_string(),
                COUNTRY_ID_ABBREVIATION => {
                    default_passport.country_id =
                        Some(value.parse().map_err(|_| {
                            MalformedPassport::MalformedData("country id".to_string())
                        })?)
                }
                _ => {
                    return Err(MalformedPassport::MalformedData(
                        "unknown field".to_string(),
                    ))
                }
            }
        }

        // check if all mandatory fields were set
        if let Some(missing) = default_passport.check_missing_fields() {
            return Err(MalformedPassport::MissingField(missing));
        }

        Ok(default_passport)
    }
}

impl Passport {
    fn check_missing_fields(&self) -> Option<Vec<&'static str>> {
        let mut missing_fields = Vec::new();
        if self.birth_year == MISSING_FIELD_NUM {
            missing_fields.push(BIRTH_YEAR_ABBREVIATION)
        }
        if self.issue_year == MISSING_FIELD_NUM {
            missing_fields.push(ISSUE_YEAR_ABBREVIATION)
        }
        if self.expiration_year == MISSING_FIELD_NUM {
            missing_fields.push(EXPIRATION_YEAR_ABBREVIATION)
        }
        if self.height == MISSING_FIELD_STR {
            missing_fields.push(HEIGHT_ABBREVIATION)
        }
        if self.hair_color == MISSING_FIELD_STR {
            missing_fields.push(HAIR_COLOR_ABBREVIATION)
        }
        if self.eye_color == MISSING_FIELD_STR {
            missing_fields.push(EYE_COLOR_ABBREVIATION)
        }
        if self.passport_id == MISSING_FIELD_STR {
            missing_fields.push(PASSPORT_ID_ABBREVIATION)
        }

        if missing_fields.is_empty() {
            None
        } else {
            Some(missing_fields)
        }
    }
}

fn try_parse_passports(raw_data: &str) -> Vec<Result<Passport, MalformedPassport>> {
    raw_data.split("\n\n").map(Passport::try_from).collect()
}

fn part1(input: &str) -> usize {
    try_parse_passports(input)
        .into_iter()
        .filter(Result::is_ok)
        .count()
}

fn part2(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = input_read::read_to_string("input").expect("failed to read input file");
    let part1_result = part1(&input);
    println!("Part 1 result is {}", part1_result);

    // let part2_result = part2(&input).expect("failed to solve part2");
    // println!("Part 2 result is {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#
        .to_string();

        let expected = 2;

        assert_eq!(expected, part1(&input))
    }

//     #[test]
//     fn part2_sample_input() {
//         let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm
//
// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
// hcl:#cfa07d byr:1929
//
// hcl:#ae17e1 iyr:2013
// eyr:2024
// ecl:brn pid:760753108 byr:1931
// hgt:179cm
//
// hcl:#cfa07d eyr:2025 pid:166559648
// iyr:2011 ecl:brn hgt:59in
// "#
//         .to_string();
//
//         let expected = 336;
//
//         assert_eq!(expected, part2(&input).unwrap())
//     }
}
