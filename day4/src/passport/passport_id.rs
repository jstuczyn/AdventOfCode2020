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

pub(super) struct PassportId(String);

impl<S: Into<String>> From<S> for PassportId {
    fn from(str: S) -> Self {
        PassportId(str.into())
    }
}

impl PassportId {
    pub(super) fn validate_in_passport(&self) -> bool {
        // it must be numeric of size 9
        if !self.0.is_ascii() || self.0.len() != 9 {
            return false;
        }
        for char in self.0.chars() {
            if !char.is_numeric() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passport_id_validation() {
        let valid = vec!["000000001", "600304001"];
        let invalid = vec!["", "0123456789", "00000001", "aaaaaaaaa"];

        for valid in valid {
            assert!(PassportId(valid.to_string()).validate_in_passport())
        }

        for invalid in invalid {
            assert!(!PassportId(invalid.to_string()).validate_in_passport())
        }
    }
}
