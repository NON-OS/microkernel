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

//! The start-up mode as section 12 and the driver's own limits say it must
//! be, from the register file alone. Firmware's mode is kept when DISPI
//! shows one: enabled with the linear framebuffer, 32 bits, from the start
//! of video memory, a size, and its lines (the virtual width, no narrower
//! than the width: narrower lines overlap, which no copy draws) fitting
//! both video memory and the mapping. Otherwise the panel's
//! preferred mode when it fits them, a width a multiple of 8, 640x480 to
//! the largest side; else 1024x768. A mode past video memory or past the
//! mapping is refused before anything is written, and a mode the adapter
//! does not show back is not taken.

use super::file::File;
use crate::error::BgaError;

#[derive(Debug, PartialEq, Eq)]
pub enum Want {
    /// Firmware's mode, kept: (width, height, stride), nothing written.
    Kept(u32, u32, u32),
    /// A mode set and shown: (width, height).
    Set(u32, u32),
    /// Refused before a register was written.
    Refused(BgaError),
    /// Written, and not shown back.
    NotTaken,
}

pub fn want(f: &File, preferred: Option<(u32, u32)>, mapped: u64, lie: bool) -> Want {
    let video = u64::from(f[9]) * 0x1_0000;
    let room = video.min(mapped);
    let (w, h) = (u32::from(f[0]), u32::from(f[1]));
    let showing = f[3] & 0x41 == 0x41 && f[2] == 32 && f[7] == 0 && f[8] == 0 && w > 0 && h > 0;
    let showing = showing && u32::from(f[5]) >= w;
    let stride = u32::from(f[5]) * 4;
    if showing && u64::from(stride) * u64::from(h) <= room {
        return Want::Kept(w, h, stride);
    }
    let (w, h) = preferred.filter(|&(w, h)| makes(room, w, h)).unwrap_or((1024, 768));
    let bytes = u64::from(w) * u64::from(h) * 4;
    if bytes > video {
        return Want::Refused(BgaError::VideoMemoryTooSmall);
    }
    if bytes > mapped {
        return Want::Refused(BgaError::ModeTooLarge);
    }
    if lie {
        Want::NotTaken
    } else {
        Want::Set(w, h)
    }
}

/// Whether this adapter makes a `w` x `h` mode in `room` bytes: sides from
/// 640x480 to 4096, a width a multiple of 8, four bytes a pixel.
pub fn makes(room: u64, w: u32, h: u32) -> bool {
    let sides = (640..=4096).contains(&w) && (480..=4096).contains(&h);
    sides && w.is_multiple_of(8) && u64::from(w) * u64::from(h) * 4 <= room
}
