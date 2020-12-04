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

const HEIGHT_METRIC_UNIT: &str = "cm";
const HEIGHT_IMPERIAL_UNIT: &str = "in";

#[derive(Debug)]
pub(super) struct InvalidHeight;

pub(super) enum Height {
    Metric(usize),
    Imperial(usize),
}

impl<'a> TryFrom<&'a str> for Height {
    type Error = InvalidHeight;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // we need at very least 3 characters - 2 for unit and one for value
        if !value.is_ascii() || value.len() < 3 {
            return Err(InvalidHeight);
        }

        let mut chars: Vec<_> = value.chars().collect();
        let unit = vec![chars.pop().unwrap(), chars.pop().unwrap()]
            .iter()
            .rev()
            .collect::<String>();
        let value = chars
            .iter()
            .collect::<String>()
            .parse()
            .map_err(|_| InvalidHeight)?;

        match &*unit {
            HEIGHT_METRIC_UNIT => Ok(Height::Metric(value)),
            HEIGHT_IMPERIAL_UNIT => Ok(Height::Imperial(value)),
            _ => Err(InvalidHeight),
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

    pub(super) fn validate_in_passport(&self) -> bool {
        let value = self.value();
        if self.is_metric() && (value < 150 || value > 193) {
            return false;
        }
        if self.is_imperial() && (value < 59 || value > 76) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(Height::try_from("60").is_err());
    }
}
