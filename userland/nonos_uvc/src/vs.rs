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
//! What the hub class needs of a USB host controller driver and of the
//! The class-specific VS descriptors read: a format's index and kind, a
//! frame's index, size, bound and default interval.

use alloc::vec::Vec;

use crate::desc::{Format, FormatKind, FrameSize};

/// A format descriptor's body (`d`, from its bLength), or None.
pub(crate) fn format(d: &[u8]) -> Option<Format> {
    let kind = match (d.get(2)?, d.len()) {
        (0x04, 27..) => {
            let fourcc = [d[5], d[6], d[7], d[8]];
            FormatKind::Uncompressed { fourcc, bits: d[21] }
        }
        (0x06, 11..) => FormatKind::Mjpeg,
        _ => return None,
    };
    Some(Format { index: d[3], kind, frames: Vec::new() })
}

/// A frame descriptor's body, uncompressed or MJPEG alike in the fields
/// read here, or None.
pub(crate) fn frame(d: &[u8]) -> Option<FrameSize> {
    if !matches!(d.get(2)?, 0x05 | 0x07) || d.len() < 26 {
        return None;
    }
    let u16_at = |i: usize| u16::from_le_bytes([d[i], d[i + 1]]);
    let u32_at = |i: usize| u32::from_le_bytes([d[i], d[i + 1], d[i + 2], d[i + 3]]);
    let (width, height) = (u16_at(5), u16_at(7));
    if width == 0 || height == 0 {
        return None;
    }
    Some(FrameSize { index: d[3], width, height, max_bytes: u32_at(17), interval: u32_at(21) })
}
