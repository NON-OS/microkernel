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

use super::pick;
use uefi::proto::console::gop::{ModeInfo, PixelFormat};

/// Some(true) for a linear mode laid out B,G,R,X in memory, Some(false) for
/// R,G,B,X, None for a mode with no linear framebuffer (PixelBltOnly) or a
/// PixelBitMask layout that is not one of those two with 32 bits a pixel.
pub(crate) fn linear_bgr(info: &ModeInfo) -> Option<bool> {
    match info.pixel_format() {
        PixelFormat::Rgb => Some(false),
        PixelFormat::Bgr => Some(true),
        PixelFormat::Bitmask => {
            let m = info.pixel_bitmask()?;
            pick::bitmask_bgr(m.red, m.green, m.blue, m.reserved)
        }
        PixelFormat::BltOnly => None,
    }
}

pub(super) fn fb_covers_mode(fb_size: usize, stride: usize, height: usize) -> bool {
    stride
        .checked_mul(height)
        .and_then(|px| px.checked_mul(core::mem::size_of::<u32>()))
        .map_or(false, |needed| fb_size >= needed)
}
