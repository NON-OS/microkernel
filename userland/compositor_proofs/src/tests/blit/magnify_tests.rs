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

//! The magnify blit: every screen pixel shows the canvas pixel under its
//! centre through the lens, and nothing outside the screen or the drawn
//! rectangle is touched. A damaged rectangle: magnify_damage_tests.

use crate::damage::Rect;
use crate::display::magnifier_rule::Lens;
use crate::sw_blitter::{magnify, Surface};

const CW: u32 = 24;
const CH: u32 = 16;
pub const SW: u32 = 30;
pub const SH: u32 = 20;
pub const OUT: u32 = 0x1234_5678;

fn surface(px: &mut [u32], w: u32, h: u32) -> Surface {
    Surface {
        base_va: px.as_mut_ptr() as u64,
        stride: w * 4,
        width: w,
        height: h,
        byte_len: px.len() as u64 * 4,
    }
}

pub fn shown(rect: Rect, l: Lens) -> (Vec<u32>, Option<Rect>) {
    let mut canvas: Vec<u32> = (0..CW * CH).map(|i| 0xFF00_0000 | i).collect();
    let mut screen = vec![OUT; (SW * SH) as usize];
    let r = magnify(surface(&mut canvas, CW, CH), surface(&mut screen, SW, SH), rect, l);
    (screen, r)
}

fn want(sx: u32, sy: u32, l: Lens) -> u32 {
    let (cx, cy) = (l.ox + (4 * sx + 2) / l.z, l.oy + (4 * sy + 2) / l.z);
    if cx < CW && cy < CH {
        0xFF00_0000 | (cy * CW + cx)
    } else {
        0xFF00_0000
    }
}

#[test]
fn the_whole_canvas_fills_the_screen_through_the_lens() {
    for l in
        [Lens { z: 8, ox: 5, oy: 3 }, Lens { z: 6, ox: 0, oy: 0 }, Lens { z: 13, ox: 17, oy: 11 }]
    {
        let (screen, r) = shown(Rect { x: 0, y: 0, width: CW, height: CH }, l);
        let r = r.unwrap();
        for sy in 0..SH {
            for sx in 0..SW {
                let inside = sx < r.x + r.width && sy < r.y + r.height;
                let got = screen[(sy * SW + sx) as usize];
                assert_eq!(got, if inside { want(sx, sy, l) } else { OUT }, "{l:?} at {sx},{sy}");
            }
        }
    }
}
