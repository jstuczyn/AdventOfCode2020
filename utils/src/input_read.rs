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

use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

/// Reads the file as lines, parsing each of them into desired type.
pub fn read_line_input<T, P>(path: P) -> io::Result<Vec<T>>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(path)?;

    let lines = io::BufReader::new(file).lines();
    let size_hint = lines.size_hint();

    // use upper bound size hint if available, otherwise use lower bound
    let mut results = Vec::with_capacity(size_hint.1.unwrap_or(size_hint.0));
    for line in lines {
        // the last one is technically not an io error, but I can't be bothered to create a separate error type just for this
        results.push(line?.parse().map_err(|parse_err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "input could not be parsed into desired type - {:?}",
                    parse_err
                ),
            )
        })?)
    }

    Ok(results)
}

/// Reads the file as a String
pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Reads the file and outputs String groups that were originally separated by an empty line
pub fn read_into_string_groups<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    fs::read_to_string(path)
        .map(|string| string.split("\n\n").map(|split| split.to_owned()).collect())
}
