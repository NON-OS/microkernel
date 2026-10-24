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

//! The cursor plane (SKL PRM Vol 2c, CUR_CTL, CUR_BASE, CUR_POS): a 64 x
//! 64 ARGB image the engine blends over the pipe. CUR_CTL and CUR_POS are
//! double-buffered and armed by a write of CUR_BASE, so every update ends
//! with one, even when only the position moved.

use super::access::Mmio;
use super::error::Error;
use super::pipe::Pipe;
use super::plane_regs as r;

/// CUR_CTL Cursor Mode Select, 5:0: 100111b is 64 x 64 32bpp ARGB, 0 is
/// off (the mode encoding is the PRM's; unverified against the volume).
pub const MODE_64_ARGB: u32 = 0x27;
pub const MODE_MASK: u32 = 0x3F;

/// CUR_POS: sign in bit 31 (Y) and bit 15 (X), magnitudes in 27:16 and
/// 11:0. The image's top left may sit off the pipe's top or left edge.
pub fn position(x: i32, y: i32) -> u32 {
    let mag = |v: i32| v.unsigned_abs().min(0xFFF);
    let sign = |v: i32| (v < 0) as u32;
    (sign(y) << 31) | (mag(y) << 16) | (sign(x) << 15) | mag(x)
}

/// Show the cursor image at graphics address `base` with its top left at
/// (x, y), or hide it when `base` is None.
pub fn set(m: &mut dyn Mmio, p: Pipe, base: Option<u32>, x: i32, y: i32) -> Result<(), Error> {
    let ctl = m.read(r::cur_ctl(p)) & !MODE_MASK;
    match base {
        Some(b) if !b.is_multiple_of(4096) => Err(Error::Argument),
        Some(b) => {
            m.write(r::cur_pos(p), position(x, y));
            m.write(r::cur_ctl(p), ctl | MODE_64_ARGB);
            m.write(r::cur_base(p), b);
            Ok(())
        }
        None => {
            let b = m.read(r::cur_base(p));
            m.write(r::cur_ctl(p), ctl);
            m.write(r::cur_base(p), b);
            Ok(())
        }
    }
}
