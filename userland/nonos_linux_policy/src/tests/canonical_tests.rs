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

//! One policy, one encoding: any single bit flipped anywhere either is
//! refused or decodes to a policy that encodes back to exactly the flipped
//! bytes. Two encodings of one policy would give it two hashes. So for
//! version 2, whose name has one spelling, zero padded.

use super::sample::{policy, served};
use crate::{decode, policy_hash, LEN};

#[test]
fn every_flip_is_refused_or_reencodes_to_itself() {
    for base in [policy().encode(), served().encode()] {
        for at in 0..LEN {
            for bit in 0..8 {
                let mut b = base;
                b[at] ^= 1 << bit;
                if let Ok(p) = decode(&b) {
                    assert_eq!(p.encode(), b, "byte {at} bit {bit}");
                    assert_ne!(policy_hash(&b), policy_hash(&base));
                }
            }
        }
    }
}

#[test]
fn reencoding_a_decoded_policy_is_the_identity() {
    for b in [policy().encode(), served().encode()] {
        assert_eq!(decode(&b).unwrap().encode(), b);
    }
}
