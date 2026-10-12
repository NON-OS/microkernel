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

//! What a pipe's primary plane holds, read back (SKL PRM Vol 2c, PLANE_*;
//! BDW PRM Vol 2c, DSP* at the same offsets, but no size register: the
//! plane shows PIPESRC's picture).

use super::access::{field, Mmio};
use super::gen::PlaneKind;
use super::pipe::Pipe;
use super::plane_regs as r;
use super::regs;

/// PIPESRC puts the width high, PLANE_SIZE the height.
fn wh(v: u32, w: (u32, u32), h: (u32, u32)) -> (u32, u32) {
    (field(v, w.0, w.1) + 1, field(v, h.0, h.1) + 1)
}

pub const ENABLE: u32 = 1 << 31;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneState {
    pub kind: PlaneKind,
    pub ctl: u32,
    pub stride: u32, // bytes
    pub width: u32,
    pub height: u32,
    /// The armed surface address, and the one scanned out.
    pub surf: u32,
    pub live: u32,
}

impl PlaneState {
    pub fn read(m: &mut dyn Mmio, kind: PlaneKind, p: Pipe) -> PlaneState {
        let (width, height) = match kind {
            PlaneKind::Primary => wh(m.read(regs::pipe_src(p)), (28, 16), (11, 0)),
            _ => wh(m.read(r::plane_size(p, 1)), (12, 0), (27, 16)),
        };
        PlaneState {
            kind,
            ctl: m.read(r::plane_ctl(p, 1)),
            stride: kind.stride_bytes(m.read(r::plane_stride(p, 1))),
            width,
            height,
            surf: m.read(r::plane_surf(p, 1)) & !0xFFF,
            live: m.read(r::plane_surflive(p, 1)) & !0xFFF,
        }
    }

    pub fn enabled(&self) -> bool {
        self.ctl & ENABLE != 0
    }

    /// Linear, unrotated 32-bit RGB (either order): a layout to flip.
    pub fn flippable(&self) -> bool {
        let k = self.kind;
        let format = self.ctl & k.format_mask();
        let rgb = format == k.format_argb() || (k == PlaneKind::Primary && format == 0xE << 26);
        rgb && self.ctl & k.layout_mask() == 0
    }
}
