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


//! The TLS 1.2 PRF against a reference written in Python.

use super::expect::{hex, lines};
use crate::tls12::prf::{prf, Hash};

#[test]
fn prf_over_sha256_and_sha384_at_every_length_used() {
    for (tag, hash) in [("prf256", Hash::Sha256), ("prf384", Hash::Sha384)] {
        let all = lines(tag);
        assert_eq!(all.len(), 4);
        for f in all {
            let n: usize = f[4].parse().unwrap();
            let mut out = std::vec![0u8; n];
            prf(hash, &hex(f[1]), &hex(f[2]), &hex(f[3]), &mut out).unwrap();
            assert_eq!(out, hex(f[5]), "{tag} {n}");
        }
    }
}

#[test]
fn the_suites_pick_their_hash() {
    assert_eq!(Hash::of(0xCCA8), Hash::Sha256);
    assert_eq!(Hash::of(0xC030), Hash::Sha384);
}
