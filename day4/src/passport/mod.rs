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

use self::eye_color::{EyeColor, InvalidEyeColor};
use self::hair_color::HairColor;
use self::height::{Height, InvalidHeight};
use self::passport_id::PassportId;
use std::collections::HashMap;
use std::convert::TryFrom;

mod eye_color;
mod hair_color;
mod height;
mod passport_id;

const BIRTH_YEAR_ABBREVIATION: &str = "byr";
const ISSUE_YEAR_ABBREVIATION: &str = "iyr";
const EXPIRATION_YEAR_ABBREVIATION: &str = "eyr";
const HEIGHT_ABBREVIATION: &str = "hgt";
const HAIR_COLOR_ABBREVIATION: &str = "hcl";
const EYE_COLOR_ABBREVIATION: &str = "ecl";
const PASSPORT_ID_ABBREVIATION: &str = "pid";
const COUNTRY_ID_ABBREVIATION: &str = "cid";

#[derive(Debug)]
pub(crate) enum MalformedPassport {
    MissingFields,
    MalformedData(String),
}

impl From<InvalidHeight> for MalformedPassport {
    fn from(err: InvalidHeight) -> Self {
        MalformedPassport::MalformedData(format!("invalid height - {:?}", err))
    }
}

impl From<InvalidEyeColor> for MalformedPassport {
    fn from(err: InvalidEyeColor) -> Self {
        MalformedPassport::MalformedData(format!("invalid height - {:?}", err))
    }
}

pub(crate) struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: Height,
    hair_color: HairColor,
    #[allow(dead_code)] // we don't use this field for anything
    eye_color: EyeColor,
    passport_id: PassportId,
    #[allow(dead_code)] // we don't use this field for anything
    country_id: Option<usize>,
}

impl TryFrom<RawPassport> for Passport {
    type Error = MalformedPassport;

    fn try_from(value: RawPassport) -> Result<Self, Self::Error> {
        Ok(Passport {
            birth_year: value.birth_year,
            issue_year: value.issue_year,
            expiration_year: value.expiration_year,
            height: Height::try_from(&*value.height)?,
            hair_color: HairColor::from(value.hair_color),
            eye_color: EyeColor::try_from(&*value.eye_color)?,
            passport_id: PassportId::from(value.passport_id),
            country_id: value.country_id,
        })
    }
}

impl Passport {
    pub(crate) fn validate(&self) -> bool {
        if self.birth_year < 1920 || self.birth_year > 2002 {
            return false;
        }

        if self.issue_year < 2010 || self.issue_year > 2020 {
            return false;
        }

        if self.expiration_year < 2020 || self.expiration_year > 2030 {
            return false;
        }

        if !self.height.validate_in_passport() {
            return false;
        }

        if !self.hair_color.validate_in_passport() {
            return false;
        }

        if !self.passport_id.validate_in_passport() {
            return false;
        }

        true
    }
}

pub(crate) struct RawPassport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<usize>,
}

impl<'a> TryFrom<&'a str> for RawPassport {
    type Error = MalformedPassport;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let fields = value.split_ascii_whitespace();

        let mut mandatory_passport_fields = HashMap::new();
        let mut country_id = None;

        for field in fields {
            let name_value = field.split(':').collect::<Vec<_>>();
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
                    mandatory_passport_fields.insert(BIRTH_YEAR_ABBREVIATION, value);
                }
                ISSUE_YEAR_ABBREVIATION => {
                    mandatory_passport_fields.insert(ISSUE_YEAR_ABBREVIATION, value);
                }
                EXPIRATION_YEAR_ABBREVIATION => {
                    mandatory_passport_fields.insert(EXPIRATION_YEAR_ABBREVIATION, value);
                }
                HEIGHT_ABBREVIATION => {
                    mandatory_passport_fields.insert(HEIGHT_ABBREVIATION, value);
                }
                HAIR_COLOR_ABBREVIATION => {
                    mandatory_passport_fields.insert(HAIR_COLOR_ABBREVIATION, value);
                }
                EYE_COLOR_ABBREVIATION => {
                    mandatory_passport_fields.insert(EYE_COLOR_ABBREVIATION, value);
                }
                PASSPORT_ID_ABBREVIATION => {
                    mandatory_passport_fields.insert(PASSPORT_ID_ABBREVIATION, value);
                }
                COUNTRY_ID_ABBREVIATION => country_id = Some(value),
                _ => {
                    return Err(MalformedPassport::MalformedData(
                        "unknown field".to_string(),
                    ));
                }
            }
        }

        if mandatory_passport_fields.len() != 7 {
            return Err(MalformedPassport::MissingFields);
        }

        Ok(RawPassport {
            birth_year: mandatory_passport_fields
                .get(BIRTH_YEAR_ABBREVIATION)
                .unwrap()
                .parse()
                .map_err(|_| MalformedPassport::MissingFields)?,
            issue_year: mandatory_passport_fields
                .get(ISSUE_YEAR_ABBREVIATION)
                .unwrap()
                .parse()
                .map_err(|_| MalformedPassport::MissingFields)?,
            expiration_year: mandatory_passport_fields
                .get(EXPIRATION_YEAR_ABBREVIATION)
                .unwrap()
                .parse()
                .map_err(|_| MalformedPassport::MissingFields)?,
            height: mandatory_passport_fields
                .get(HEIGHT_ABBREVIATION)
                .unwrap()
                .parse()
                .map_err(|_| MalformedPassport::MissingFields)?,
            hair_color: mandatory_passport_fields
                .get(HAIR_COLOR_ABBREVIATION)
                .unwrap()
                .parse()
                .map_err(|_| MalformedPassport::MissingFields)?,
            eye_color: mandatory_passport_fields
                .get(EYE_COLOR_ABBREVIATION)
                .unwrap()
                .parse()
                .map_err(|_| MalformedPassport::MissingFields)?,
            passport_id: mandatory_passport_fields
                .get(PASSPORT_ID_ABBREVIATION)
                .unwrap()
                .parse()
                .map_err(|_| MalformedPassport::MissingFields)?,
            country_id: country_id.map(|id| id.parse().ok()).flatten(),
        })
    }
}
