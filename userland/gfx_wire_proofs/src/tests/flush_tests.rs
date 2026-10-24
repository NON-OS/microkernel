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

//! FLUSH bodies and the rectangle bound.

use crate::body::flush::Flush;
use crate::body::rect::Rect;
use crate::errno::EINVAL;

fn r(x: u32, y: u32, w: u32, h: u32) -> Rect {
    Rect { x, y, width: w, height: h }
}

#[test]
fn fifteen_rectangles_fit_and_round_trip() {
    let mut f = Flush { serial: 0x0042_0002, len: 15, ..Flush::default() };
    for (i, s) in f.rects.iter_mut().enumerate() {
        *s = r(i as u32, 2, 3, 4);
    }
    let mut out = [0u8; 248];
    assert_eq!(f.encode(&mut out), Ok(248));
    assert_eq!(Flush::decode(&out), Ok(f));
    assert_eq!(Flush::decode(&out[..247]), Err(EINVAL));
}

#[test]
fn zero_or_sixteen_rectangles_are_refused_both_ways() {
    let f = Flush { serial: 1, len: 0, ..Flush::default() };
    assert_eq!(f.encode(&mut [0u8; 256]), Err(EINVAL));
    let mut b = [0u8; 8 + 16 * 16];
    b[4] = 16;
    assert_eq!(Flush::decode(&b), Err(EINVAL));
    b[4] = 0;
    assert_eq!(Flush::decode(&b), Err(EINVAL));
}

#[test]
fn a_rectangle_is_inside_only_when_wholly_inside_and_not_empty() {
    assert!(r(0, 0, 1920, 1080).inside(1920, 1080));
    assert!(r(1919, 1079, 1, 1).inside(1920, 1080));
    assert!(!r(1919, 0, 2, 1).inside(1920, 1080));
    assert!(!r(0, 0, 0, 5).inside(1920, 1080));
    assert!(!r(u32::MAX, 0, 2, 2).inside(u32::MAX, 10), "no wrap");
}
