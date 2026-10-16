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

//! The rectangles of `rects` against flush and transfer. Each is in or out
//! by name, and one that is out moves no pixel.

use super::frames::{ask, flush_body, transfer_body};
use super::model::{paint, served, take, COMPOSITOR};
use super::rects::{H, INSIDE, OUTSIDE, W};
use crate::adapter::GUARD;
use crate::constants::CLEAR_COLOR;
use crate::serve::{E_INVAL, OP_FLUSH, OP_TRANSFER_TO_HOST};

#[test]
fn flush_takes_every_rectangle_inside_the_mode_and_refuses_every_one_outside() {
    let mut s = served(W as u16, H as u16);
    take(&mut s);
    paint(&s);
    for (x, y, w, h) in OUTSIDE {
        let status = ask(&mut s.d, COMPOSITOR, OP_FLUSH, &flush_body(x, y, w, h)).0;
        assert_eq!(status, E_INVAL, "({x}, {y}) {w}x{h} flushed");
    }
    let untouched = (0..W * H).all(|i| s.a.fb.wrote32(i as usize * 4) == CLEAR_COLOR);
    assert!(untouched, "a refused flush moved a pixel");
    for (x, y, w, h) in INSIDE {
        let status = ask(&mut s.d, COMPOSITOR, OP_FLUSH, &flush_body(x, y, w, h)).0;
        assert_eq!(status, 0, "({x}, {y}) {w}x{h} refused");
    }
    assert_eq!(s.a.fb.wrote32((W * H * 4) as usize), GUARD, "written past the mapping");
}

#[test]
fn transfer_holds_the_same_edges_at_the_offset_each_rectangle_starts_at() {
    let mut s = served(W as u16, H as u16);
    take(&mut s);
    let at = |x: u32, y: u32| (y as u64 * W as u64 + x as u64) * 4;
    let mut transfer = |x, y, w, h, offset| {
        ask(&mut s.d, COMPOSITOR, OP_TRANSFER_TO_HOST, &transfer_body(x, y, w, h, offset)).0
    };
    for (x, y, w, h) in INSIDE {
        assert_eq!(transfer(x, y, w, h, at(x, y)), 0, "({x}, {y}) {w}x{h} refused");
    }
    for (x, y, w, h) in OUTSIDE {
        for offset in [at(x, y), 0] {
            let status = transfer(x, y, w, h, offset);
            assert_eq!(status, E_INVAL, "({x}, {y}) {w}x{h} at {offset} taken");
        }
    }
}
