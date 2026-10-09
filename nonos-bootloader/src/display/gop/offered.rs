// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::mode::linear_bgr;
use super::pick::Offered;
use super::state::{NO_FB_BAD_GEOMETRY, NO_FB_BITMASK, NO_FB_BLT_ONLY};
use alloc::vec::Vec;
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};

/// The linear modes a GOP offers, and what the rest were.
pub(super) struct Modes {
    pub offered: Vec<Offered>,
    pub saw_blt_only: bool,
    pub saw_bitmask: bool,
}

pub(super) fn offered_modes(gop: &GraphicsOutput) -> Modes {
    let mut offered = Vec::new();
    let mut saw_blt_only = false;
    let mut saw_bitmask = false;
    for idx in 0..gop.modes().len() as u32 {
        let Ok(mode) = gop.query_mode(idx) else { continue };
        let info = mode.info();
        match info.pixel_format() {
            PixelFormat::BltOnly => saw_blt_only = true,
            PixelFormat::Bitmask if linear_bgr(info).is_none() => saw_bitmask = true,
            _ => {}
        }
        if linear_bgr(info).is_none() {
            continue;
        }
        let (w, ht) = info.resolution();
        offered.push(Offered { index: idx, width: w as u32, height: ht as u32 });
    }
    Modes { offered, saw_blt_only, saw_bitmask }
}

impl Modes {
    /// Why no framebuffer was latched from this GOP, one of the NO_FB_* values.
    pub fn failure(&self) -> u8 {
        if !self.offered.is_empty() {
            NO_FB_BAD_GEOMETRY
        } else if self.saw_bitmask {
            NO_FB_BITMASK
        } else if self.saw_blt_only {
            NO_FB_BLT_ONLY
        } else {
            NO_FB_BAD_GEOMETRY
        }
    }
}
