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

use super::consts::{PIXEL_FORMAT_BGRX, PIXEL_FORMAT_RGBX};
use crate::display::gop::linear_bgr;
use uefi::proto::console::gop::ModeInfo;

// The handoff's pixel format for a mode the kernel can scan out from, by the
// same rule the splash latched with: RGB, BGR, or a PixelBitMask mode whose
// masks are one of those two at 32 bits a pixel. PixelBltOnly has no linear
// framebuffer and is never usable.
pub(super) fn mode_usable(info: &ModeInfo) -> Option<u32> {
    let (width, height) = info.resolution();
    if width == 0 || height == 0 || info.stride() < width {
        return None;
    }
    Some(if linear_bgr(info)? { PIXEL_FORMAT_BGRX } else { PIXEL_FORMAT_RGBX })
}
