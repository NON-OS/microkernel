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

use alloc::vec::Vec;

use super::points::Point;

/// Append the flags and coordinates of a simple glyph the way the
/// reference decoder writes them: a flag per point with runs folded into
/// a repeat count, then x steps, then y steps, a byte each when they fit.
/// Coordinates keep their low 16 bits, as the glyf format does. Flag
/// bits: 1 on curve, 2 and 4 short x and y, 8 repeat, 16 and 32 x and y
/// same (or positive when short), 64 overlapping contours.
pub(super) fn store_points(pts: &[Point], overlap: bool, out: &mut Vec<u8>) {
    let (mut xs, mut ys) = (Vec::new(), Vec::new());
    let (mut last, mut last_flag, mut repeat) = ((0, 0), None, 0u8);
    for (i, p) in pts.iter().enumerate() {
        let mut flag = p.on as u8;
        if overlap && i == 0 {
            flag |= 0x40;
        }
        flag |= step(p.x - last.0, 0x02, 0x10, &mut xs);
        flag |= step(p.y - last.1, 0x04, 0x20, &mut ys);
        last = (p.x, p.y);
        match out.last_mut() {
            Some(prev) if last_flag == Some(flag) && repeat != 255 => {
                *prev |= 0x08;
                repeat += 1;
            }
            _ => {
                if repeat != 0 {
                    out.push(repeat);
                }
                out.push(flag);
                repeat = 0;
            }
        }
        last_flag = Some(flag);
    }
    if repeat != 0 {
        out.push(repeat);
    }
    out.extend_from_slice(&xs);
    out.extend_from_slice(&ys);
}

/// Code one coordinate step into `bytes`, returning its flag bits.
fn step(d: i32, short: u8, same: u8, bytes: &mut Vec<u8>) -> u8 {
    match d {
        0 => same,
        -255..=255 => {
            bytes.push(d.unsigned_abs() as u8);
            short | if d > 0 { same } else { 0 }
        }
        _ => {
            bytes.extend_from_slice(&(d as i16).to_be_bytes());
            0
        }
    }
}
