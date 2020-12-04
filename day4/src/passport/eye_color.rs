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

use std::convert::TryFrom;

const COLOR_AMBER_ABBREVIATION: &str = "amb";
const COLOR_BLUE_ABBREVIATION: &str = "blu";
const COLOR_BROWN_ABBREVIATION: &str = "brn";
const COLOR_GRAY_ABBREVIATION: &str = "gry";
const COLOR_GREEN_ABBREVIATION: &str = "grn";
const COLOR_HAZEL_ABBREVIATION: &str = "hzl";
const COLOR_OTHER_ABBREVIATION: &str = "oth";

pub(super) enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

#[derive(Debug)]
pub(super) struct InvalidEyeColor;

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
            _ => Err(InvalidEyeColor),
        }
    }
}
