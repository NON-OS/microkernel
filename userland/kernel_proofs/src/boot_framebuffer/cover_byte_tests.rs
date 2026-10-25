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

//! The cover rule against the page-by-page model at the grain of one byte
//! (L79). The other sweeps step by a quarter or half page, where a rule
//! that drops a span's last byte, ignores a one-byte BAR or reads an end as
//! inclusive gives the same answers; here the floors count the cases whose
//! answer one byte decides, so such a rule is met and fails.

use super::cover_byte_gen::{bar, frame};
use super::cover_find::MMIO;
use super::cover_model::{bits, touched, Lcg};
use super::cover_rule::{holds, pages};

/// What the sweep met: held, not held, held by a one-byte BAR, and held by
/// the one byte at the BAR's end, the BAR's start, the frame's end and the
/// frame's start, with that byte's span shorter by it not held.
const FLOORS: [(&str, u32); 7] = [
    ("held", 60_000),
    ("not held", 60_000),
    ("one-byte BAR", 4_000),
    ("BAR's last byte", 12_000),
    ("BAR's first byte", 12_000),
    ("frame's last byte", 6_500),
    ("frame's first byte", 6_000),
];

#[test]
fn byte_grained_spans_are_held_as_their_pages_say() {
    let mut g = Lcg(0x0079_B17E_0000_0001);
    let mut seen = [0u32; 7];
    for case in 0..200_000 {
        let fb = frame(&mut g);
        let (b, s) = bar(&mut g, fb);
        let (fm, bm) = (touched(fb.0, fb.1), touched(b, s));
        assert_eq!(bits(pages(fb.0, fb.1)), fm, "case {case}: frame {fb:x?}");
        assert_eq!(bits(pages(b, s)), bm, "case {case}: BAR {b:#x}+{s:#x}");
        let want = fm & bm != 0;
        let got = holds(fb, &[(b, s, MMIO)], 1, MMIO);
        assert_eq!(got, want, "case {case}: frame {fb:x?}, BAR {b:#x}+{s:#x}");
        let alone = |shorter: u64, other: u64| want && shorter & other == 0;
        let met = [
            want,
            !want,
            want && s == 1,
            alone(touched(b, s - 1), fm),
            alone(touched(b + 1, s - 1), fm),
            alone(touched(fb.0, fb.1 - 1), bm),
            alone(touched(fb.0 + 1, fb.1 - 1), bm),
        ];
        for (n, m) in seen.iter_mut().zip(met) {
            *n += u32::from(m);
        }
    }
    for ((name, floor), n) in FLOORS.iter().zip(seen) {
        assert!(n >= *floor, "{name}: met {n} times, below {floor}");
    }
}
