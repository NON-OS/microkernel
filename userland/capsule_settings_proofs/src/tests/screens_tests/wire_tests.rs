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

//! The compositor's arrangement read as it writes it, and refused out of
//! range; the bodies Settings sends, and none it must not.

use super::arrange_wire::{encode, Arrangement};
use crate::settings::screens::arrange::Shown;
use crate::settings::screens::wire::{layout, scale, shown};

#[test]
fn the_compositors_answer_reads_as_it_is_written() {
    let a = Arrangement {
        outputs: 3,
        extend: true,
        extended: true,
        scale_q: 8,
        chosen: [None, Some(5), None, Some(16)],
    };
    let s = Shown { outputs: 3, extend: true, extended: true, scale_q: 8, chosen: [0, 5, 0, 16] };
    assert_eq!(shown(&encode(&a)), Some(s));
    let bad = |i: usize, v: u32| {
        let mut b = encode(&a);
        b[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        shown(&b)
    };
    for (i, v) in [(0, 0), (0, 5), (1, 2), (2, 2), (3, 0), (3, 7), (5, 3)] {
        assert_eq!(bad(i, v), None, "word {i} = {v}");
    }
    assert_eq!(shown(&encode(&a)[..28]), None, "short");
}

#[test]
fn bodies_are_the_compositors_and_bad_ones_are_never_sent() {
    assert_eq!(layout(true), 1u32.to_le_bytes());
    let b = scale(1, 6).unwrap();
    assert_eq!((&b[..4], &b[4..]), (&1u32.to_le_bytes()[..], &6u32.to_le_bytes()[..]));
    assert_eq!(scale(4, 6), None, "four displays at most");
    assert_eq!(scale(0, 7), None, "not a step");
}
