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

const MISSING_FIELD_STR: &str = "MISSING";
const MISSING_FIELD_NUM: isize = -1;

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

const HEIGHT_METRIC_UNIT: &str = "cm";
const HEIGHT_IMPERIAL_UNIT: &str = "in";

const COLOR_AMBER_ABBREVIATION: &str = "amb";
const COLOR_BLUE_ABBREVIATION: &str = "blu";
const COLOR_BROWN_ABBREVIATION: &str = "brn";
const COLOR_GRAY_ABBREVIATION: &str = "gry";
const COLOR_GREEN_ABBREVIATION: &str = "grn";
const COLOR_HAZEL_ABBREVIATION: &str = "hzl";
const COLOR_OTHER_ABBREVIATION: &str = "oth";

#[derive(Debug)]
struct InvalidHeight;

enum Height {
    Metric(usize),
    Imperial(usize),
}

impl<'a> TryFrom<&'a str> for Height {
    type Error = InvalidHeight;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // we need at very least 3 characters - 2 for unit and one for value
        if value.len() < 3 {
            return Err(InvalidHeight);
        }

        let mut chars: Vec<_> = value.chars().collect();
        let unit = vec![chars.pop().unwrap(), chars.pop().unwrap()].iter().rev().collect::<String>();
        let value = chars.iter().collect::<String>().parse().map_err(|_| InvalidHeight)?;

        match &*unit {
            HEIGHT_METRIC_UNIT => Ok(Height::Metric(value)),
            HEIGHT_IMPERIAL_UNIT => Ok(Height::Imperial(value)),
            _ => Err(InvalidHeight)
        }
    }
}

impl Height {
    fn value(&self) -> usize {
        match self {
            Height::Metric(value) => *value,
            Height::Imperial(value) => *value,
        }
    }

    fn is_metric(&self) -> bool {
        matches!(self, Height::Metric(_))
    }

    fn is_imperial(&self) -> bool {
        matches!(self, Height::Imperial(_))
    }
}

enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

#[derive(Debug)]
struct InvalidEyeColor;

impl<'a> TryFrom<&'a str> for EyeColor {
    type Error = InvalidEyeColor;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            COLOR_AMBER_ABBREVIATION => Ok(Self::Amber),
            COLOR_BLUE_ABBREVIATION => Ok(Self::Blue),
            COLOR_BROWN_ABBREVIATION => Ok(Self::Brown),
            COLOR_GRAY_ABBREVIATION => Ok(Self::Gray),
            COLOR_GREEN_ABBREVIATION => Ok(Self::Green),
            COLOR_HAZEL_ABBREVIATION => Ok(Self::Hazel),
            COLOR_OTHER_ABBREVIATION => Ok(Self::Other),
            _ => Err(InvalidEyeColor)
        }
    }
}

#[derive(Debug)]
enum MalformedPassport {
    MissingField(Vec<&'static str>),
    MalformedData(String),
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

struct Passport {
    birth_year: isize,
    issue_year: isize,
    expiration_year: isize,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<isize>,
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
                    ));
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

    fn validate_hair_color(color: &str) -> bool {
        if !color.is_ascii() || color.len() != 7 {
            return false;
        }

        for (i, char) in color.chars().enumerate() {
            if i == 0 {
                if char != '#' {
                    return false;
                } else {
                    continue;
                }
            } else {
                match char {
                    '0'..='9' | 'a'..='f' => continue,
                    _ => return false,
                }
            }
        }
        true
    }

    fn validate_passport_id(passport_id: &str) -> bool {
        // it must be numeric of size 9
        if !passport_id.is_ascii() || passport_id.len() != 9 {
            return false;
        }
        for char in passport_id.chars() {
            if !char.is_numeric() {
                return false;
            }
        }
        true
    }

    fn validate(&self) -> bool {
        if self.birth_year < 1920 || self.birth_year > 2002 {
            return false;
        }

        if self.issue_year < 2010 || self.issue_year > 2020 {
            return false;
        }

        if self.expiration_year < 2020 || self.expiration_year > 2030 {
            return false;
        }

        let height = match Height::try_from(&*self.height) {
            Ok(height) => height,
            Err(_) => return false
        };
        let height_value = height.value();
        if height.is_metric() && (height_value < 150 || height_value > 193) {
            return false;
        }
        if height.is_imperial() && (height_value < 59 || height_value > 76) {
            return false;
        }

        if !Self::validate_hair_color(&*self.hair_color) {
            return false;
        }

        if let Err(_) = EyeColor::try_from(&*self.eye_color) {
            return false;
        }

        if !Self::validate_passport_id(&*self.passport_id) {
            return false;
        }

        true
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

fn part2(input: &str) -> usize {
    try_parse_passports(input)
        .into_iter()
        .filter(Result::is_ok)
        .filter(|passport| passport.as_ref().unwrap().validate())
        .count()
}

fn main() {
    let input = input_read::read_to_string("input").expect("failed to read input file");
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

    #[test]
    fn height_parsing() {
        let height = Height::try_from("60in").unwrap();
        assert!(height.is_imperial());
        assert_eq!(60, height.value());

        let height = Height::try_from("60cm").unwrap();
        assert!(height.is_metric());
        assert_eq!(60, height.value());

        assert!(Height::try_from("60c").is_err());
        assert!(Height::try_from("60inch").is_err());
        assert!(Height::try_from("a60cm").is_err());
    }

    #[test]
    fn part2_sample_input() {
        let input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#
            .to_string();

        let expected = 4;

        assert_eq!(expected, part2(&input))
    }
}
