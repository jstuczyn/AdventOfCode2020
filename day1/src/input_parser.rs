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
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{io, path::Path};

pub(crate) fn read_input_file<T, P>(path: P) -> io::Result<Vec<T>>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(path)?;

    let lines = io::BufReader::new(file).lines();
    let size_hint = lines.size_hint();

    // use upper bound size hint if available, otherwise use lower bound
    let mut results = Vec::with_capacity(size_hint.1.unwrap_or_else(|| size_hint.0));
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
