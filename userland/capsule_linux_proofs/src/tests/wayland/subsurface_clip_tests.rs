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

//! C8u2: whatever a subsurface's size and place, edges and all, it changes
//! nothing of the window's frame outside itself or past the window's width:
//! a client never draws outside its own window.

use super::super::random::Regs;
use crate::wayland_sub::blend::{lay, Plane};

#[test]
fn a_subsurface_never_paints_outside_itself_or_past_the_window() {
    let mut r = Regs::new(0x5B5);
    let fp = Plane { w: 13, h: 9, stride: 13 * 4 + 12 };
    for _ in 0..2000 {
        let ls = Plane { w: r.small(40) as u32, h: r.small(40) as u32, stride: 0 };
        let ls = Plane { stride: ls.w * 4 + r.small(3) as u32 * 4, ..ls };
        let edge = |r: &mut Regs| [i32::MIN, i32::MAX, -40, 40][r.small(4) as usize];
        let x = if r.small(5) == 0 { edge(&mut r) } else { r.small(60) as i32 - 30 };
        let y = if r.small(5) == 0 { edge(&mut r) } else { r.small(60) as i32 - 30 };
        let layer: Vec<u8> = (0..ls.stride * ls.h).map(|_| r.any() as u8 | 1).collect();
        let before: Vec<u8> = (0..fp.stride * fp.h).map(|_| r.any() as u8).collect();
        let mut frame = before.clone();
        lay(&mut frame, fp, &layer, ls, (x, y), r.small(2) == 0);
        for (i, (a, b)) in frame.iter().zip(&before).enumerate() {
            let (row, col) = ((i as u32 / fp.stride) as i64, (i as u32 % fp.stride / 4) as i64);
            let (x, y) = (i64::from(x), i64::from(y));
            let inside = col < i64::from(fp.w)
                && (x..x + i64::from(ls.w)).contains(&col)
                && (y..y + i64::from(ls.h)).contains(&row);
            assert!(inside || a == b, "byte {i} at {x},{y} for {}x{}", ls.w, ls.h);
        }
    }
}
