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

use std::time::Instant;

const ORDER: usize = 20201227;
const GENERATOR: usize = 7;

type PrivateKey = usize;
type PublicKey = usize;

// perform (base ** exp) % modulus
// basically https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
#[inline]
fn mod_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

// sanity check to see if I understood the task
#[inline]
fn derive_public_key(secret: PrivateKey) -> PublicKey {
    mod_pow(GENERATOR, secret, ORDER)
}

// it's the last day of the advent of code so
// let's be lazy about it and since the values are small, just brute-force it.
// could I have implemented something fancier like baby-step giant-step?
// yes. did the task require it? no.
#[inline]
fn reverse_private_key(public: PublicKey) -> PrivateKey {
    let mut candidate = 1;

    loop {
        if derive_public_key(candidate) == public {
            return candidate;
        }
        candidate += 1;
    }
}

#[inline]
fn diffie_hellman_ish_thing(local_secret: PrivateKey, remote_public: PublicKey) -> PublicKey {
    mod_pow(remote_public, local_secret, ORDER)
}

fn part1(pub_keys: (PublicKey, PublicKey)) -> usize {
    // just reverse a single key
    let private = reverse_private_key(pub_keys.0);
    diffie_hellman_ish_thing(private, pub_keys.1)
}

#[cfg(not(tarpaulin))]
fn main() {
    let input = (14788856, 19316454);

    let part1_result = part1(input);
    println!("Part 1 result is {}", part1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_key_derivation() {
        assert_eq!(5764801, derive_public_key(8));
        assert_eq!(17807724, derive_public_key(11));
    }

    #[test]
    fn reversing_private_key() {
        assert_eq!(8, reverse_private_key(5764801));
        assert_eq!(11, reverse_private_key(17807724));
    }

    #[test]
    fn diffie_hellman_ish() {
        assert_eq!(14897079, diffie_hellman_ish_thing(8, 17807724));
        assert_eq!(14897079, diffie_hellman_ish_thing(11, 5764801));
    }

    #[test]
    fn part1_sample_input() {
        let input = (5764801, 17807724);

        let expected = 14897079;

        assert_eq!(expected, part1(input))
    }
}
