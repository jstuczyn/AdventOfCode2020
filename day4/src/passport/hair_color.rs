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

pub(super) struct HairColor(String);

impl<S: Into<String>> From<S> for HairColor {
    fn from(str: S) -> Self {
        HairColor(str.into())
    }
}

impl HairColor {
    pub(super) fn validate_in_passport(&self) -> bool {
        if !self.0.is_ascii() || self.0.len() != 7 {
            return false;
        }

        for (i, char) in self.0.chars().enumerate() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hair_color_validation() {
        let valid = vec!["#123abc", "#123abf"];
        let invalid = vec!["", "#123abz", "123abcd"];

        for valid in valid {
            assert!(HairColor(valid.to_string()).validate_in_passport())
        }

        for invalid in invalid {
            assert!(!HairColor(invalid.to_string()).validate_in_passport())
        }
    }
}
