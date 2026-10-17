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

/* Calldata read back: every call as itself, and each shape the encoder
 * never writes refused. */

use super::fixture::at;
use super::kinds::every;
use crate::intent::{decode, encode, Publisher};

pub(crate) fn built(p: &Publisher) -> Vec<u8> {
    encode(p, at).unwrap()
}

/// `p`'s calldata with byte `b` of word `w` (after the selector) set to 1.
fn dirty(p: &Publisher, w: usize, b: usize) -> Option<Publisher> {
    let mut cd = built(p);
    cd[4 + 32 * w + b] = 1;
    decode(p.kind(), &cd, at)
}

#[test]
fn every_call_reads_back_as_itself() {
    for p in every() {
        assert_eq!(decode(p.kind(), &built(&p), at), Some(p.clone()), "{p:?}");
    }
}

#[test]
fn a_wrong_selector_or_length_is_refused() {
    for p in every() {
        let cd = built(&p);
        let no = |d: &[u8]| decode(p.kind(), d, at).is_none();
        let mut sel = cd.clone();
        sel[3] ^= 1;
        assert!(no(&sel) && no(&cd[..3]) && no(&[]), "{p:?}");
        assert!(no(&cd[..cd.len() - 1]) && no(&cd[..cd.len() - 32]), "short {p:?}");
        assert!(no(&[&cd[..], &[0; 32]].concat()) && no(&[&cd[..], &[0]].concat()), "{p:?}");
    }
    let credit = built(&every()[8]);
    for kind in [17, 18, 9, 20, 0] {
        assert_eq!(decode(kind, &credit, at), None, "kind {kind}");
    }
    assert_eq!(decode(5, &built(&every()[3]), at), None, "requestUnbond is not withdraw");
}

#[test]
fn a_word_out_of_its_type_is_refused() {
    let e = every();
    for (p, w) in [(&e[0], 0), (&e[2], 1), (&e[8], 0)] {
        assert!(dirty(p, w, 0).is_none() && dirty(p, w, 11).is_none(), "dirty address {p:?}");
    }
    assert!(dirty(&e[7], 1, 30).is_none() && dirty(&e[7], 1, 0).is_none(), "uint8 above 255");
    for p in [&e[0], &e[1]] {
        assert_eq!(dirty(p, 1, 15), None, "an amount above u128 {p:?}");
        let mut zero = built(p);
        zero[4 + 32..].fill(0);
        assert_eq!(decode(p.kind(), &zero, at), None, "an amount of 0 {p:?}");
    }
}
