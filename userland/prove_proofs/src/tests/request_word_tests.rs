// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The nonce and the root are field words: each word p or above is refused,
//! in each of the four places, and p - 1 is read.

use crate::assemble::error::Refusal;
use crate::assemble::request::parse_request;
use crate::fixture::{field_p, request, NONCE, VERIFIER, WINDOW};

fn good() -> Vec<u8> {
    request(VERIFIER, WINDOW, &NONCE, &[0x17; 32])
}

/// p and every word above it is refused in each word of the nonce and the
/// root; p - 1 is a word.
#[test]
fn a_word_at_or_above_p_is_refused() {
    let p = field_p();
    for (at, why) in [(16, Refusal::RequestNonceWord), (48, Refusal::RequestRootWord)] {
        for word in 0..4 {
            for v in [p, p + 1, u64::MAX] {
                let mut r = good();
                r[at + 8 * word..at + 8 * word + 8].copy_from_slice(&v.to_le_bytes());
                assert_eq!(parse_request(&r), Err(why.clone()), "word {word} at {at}: {v:#x}");
            }
            let mut r = good();
            r[at + 8 * word..at + 8 * word + 8].copy_from_slice(&(p - 1).to_le_bytes());
            assert!(parse_request(&r).is_ok(), "p - 1 is a word");
        }
    }
}
