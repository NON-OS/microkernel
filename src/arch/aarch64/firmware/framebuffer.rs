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

//! The framebuffer the firmware scans out, at the display mode it already
//! chose (the wiki's frame buffer tags). Asked for at 32 bits per pixel in
//! RGB order; the answers say what the firmware actually set.

use super::call::call;
use super::message::{answer, Tag};
use super::state::mailbox;
use crate::boot::handoff::{pixel_format, FramebufferInfo};

const GET_PHYSICAL_SIZE: u32 = 0x0004_0003;
const SET_PHYSICAL_SIZE: u32 = 0x0004_8003;
const SET_VIRTUAL_SIZE: u32 = 0x0004_8004;
const SET_DEPTH: u32 = 0x0004_8005;
const SET_PIXEL_ORDER: u32 = 0x0004_8006;
const ALLOCATE: u32 = 0x0004_0001;
const GET_PITCH: u32 = 0x0004_0008;
/// Pixel order values: 0 is BGR, 1 is RGB.
const ORDER_RGB: u32 = 1;

/// Ask the firmware for its framebuffer, in CPU terms. `None` without a
/// mailbox, without a display, or when any answer is missing or not 32 bpp.
pub fn framebuffer() -> Option<FramebufferInfo> {
    let size = [Tag { id: GET_PHYSICAL_SIZE, values: &[], room: 2 }];
    let words = call(&size)?;
    let (width, height) = match answer(&size, &words, 0).ok()? {
        [w, h, ..] if *w != 0 && *h != 0 => (*w, *h),
        _ => return None,
    };
    let tags = [
        Tag { id: SET_PHYSICAL_SIZE, values: &[width, height], room: 2 },
        Tag { id: SET_VIRTUAL_SIZE, values: &[width, height], room: 2 },
        Tag { id: SET_DEPTH, values: &[32], room: 1 },
        Tag { id: SET_PIXEL_ORDER, values: &[ORDER_RGB], room: 1 },
        Tag { id: ALLOCATE, values: &[4096], room: 2 },
        Tag { id: GET_PITCH, values: &[], room: 1 },
    ];
    let words = call(&tags)?;
    let one = |i| answer(&tags, &words, i).ok().and_then(|a| a.first().copied());
    let block = answer(&tags, &words, 4).ok()?;
    let (depth, order, pitch) = (one(2)?, one(3)?, one(5)?);
    let (bus, bytes) = (*block.first()?, *block.get(1)?);
    if depth != 32 || bus == 0 || bytes == 0 || pitch == 0 {
        return None;
    }
    let window = mailbox()?.0.dma;
    let offset = (bus as u64).checked_sub(window.bus_base).filter(|o| *o < window.size)?;
    Some(FramebufferInfo {
        ptr: window.cpu_base + offset,
        size: bytes as u64,
        width,
        height,
        stride: pitch,
        pixel_format: if order == ORDER_RGB { pixel_format::RGBX } else { pixel_format::BGRX },
        ..FramebufferInfo::default()
    })
}
