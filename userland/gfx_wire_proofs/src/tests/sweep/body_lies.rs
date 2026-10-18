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

//! What a driver answers, as the compositor reads it: CAPS, SURFACE, STATE
//! and MODES bodies of every length, each field an edge or noise, held to
//! the test's own reading. A driver is input to the compositor.

use super::frames::{bytes, le16, le32};
use super::rng::Rng;
use crate::body::{caps::Caps, state::State, surface::Surface};

/// A body length near `len` most of the time, else any to 260.
pub fn near(r: &mut Rng, len: usize) -> usize {
    match r.below(4) {
        0 => r.below(261) as usize,
        1 => len.saturating_sub(1 + r.below(4) as usize),
        _ => len + r.below(13) as usize,
    }
}

fn sweep(seed: u64, len: usize, check: impl Fn(&[u8]) -> bool) -> (usize, usize) {
    let mut r = Rng::new(seed);
    let (mut taken, mut refused) = (0, 0);
    for _ in 0..100_000 {
        let n = near(&mut r, len);
        let b = bytes(&mut r, n);
        if check(&b) {
            taken += 1
        } else {
            refused += 1
        }
    }
    (taken, refused)
}

#[test]
fn each_reply_body_is_taken_exactly_as_the_test_reads_it() {
    let (t, f) = sweep(1, 32, |b| {
        let model = b.len() >= 32 && le16(b, 2) != Some(0) && le16(b, 16) != Some(0);
        let got = Caps::decode(b);
        assert_eq!(got.is_ok(), model, "caps {b:02X?}");
        got.is_ok_and(|c| (c.outputs, c.max_width) == (le16(b, 2).unwrap(), le32(b, 8).unwrap()))
    });
    assert!(t >= 10_000 && f >= 10_000, "caps {t} {f}");
    let (t, f) = sweep(2, 32, |b| {
        let w = |i: usize| le32(b, 8 + 4 * i).unwrap_or(0) as u64;
        let model = b.len() >= 32 && w(1) != 0 && w(2) != 0 && w(3) >= 4 * w(1) && w(4) == 1;
        let got = Surface::decode(b);
        assert_eq!(got.is_ok(), model, "surface {b:02X?}");
        got.is_ok_and(|s| s.cursor_offset as u64 == w(5) && s.stride as u64 == w(3))
    });
    assert!(t >= 500 && f >= 10_000, "surface {t} {f}");
    let (t, f) = sweep(3, 28, |b| {
        let got = State::decode(b);
        assert_eq!(got.is_ok(), b.len() >= 28, "state {b:02X?}");
        got.is_ok_and(|s| s.flags == le32(b, 24).unwrap() && s.generation == le32(b, 0).unwrap())
    });
    assert!(t >= 10_000 && f >= 10_000, "state {t} {f}");
}
