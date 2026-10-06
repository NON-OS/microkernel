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

use super::current::current_framebuffer_info;
use super::usable::mode_usable;
use crate::display::gop::pick::{self, Choice, Offered};
use crate::display::gop::{preferred_mode, read_edid};
use crate::handoff::types::FramebufferInfo;
use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::BootServices;

// Reached only when the splash latched nothing. The current mode is taken if
// the kernel can use it; otherwise the mode is chosen by the splash's own
// rule (EDID native first, then the largest), never simply the first usable
// one, which on most firmware is 640x480.
pub fn try_gop_handle(bs: &BootServices, handle: Handle, _idx: usize) -> Option<FramebufferInfo> {
    let edid = read_edid(bs, handle);
    let mut gop = bs.open_protocol_exclusive::<GraphicsOutput>(handle).ok()?;
    let phys_mm = edid.map_or(0, |e| pick::pack_mm(e.width_mm, e.height_mm));
    if let Some(info) = current_framebuffer_info(&mut gop, phys_mm) {
        return Some(info);
    }
    let mut offered = Vec::new();
    for idx in 0..gop.modes().len() as u32 {
        let Ok(mode) = gop.query_mode(idx) else { continue };
        if mode_usable(mode.info()).is_none() {
            continue;
        }
        let (w, h) = mode.info().resolution();
        offered.push(Offered { index: idx, width: w as u32, height: h as u32 });
    }
    let native = edid.map(|e| (e.width, e.height));
    let pinned = preferred_mode().map(|(w, h)| (w as u32, h as u32));
    let Some((Choice::Set(idx), _)) = pick::choose(&offered, None, native, pinned) else {
        return None;
    };
    let mode = gop.query_mode(idx).ok()?;
    gop.set_mode(&mode).ok()?;
    current_framebuffer_info(&mut gop, phys_mm)
}
