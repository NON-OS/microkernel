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

//! The most bytes a committed stream's frame may take. The device names a
//! bound twice, in its frame descriptor and in its probe answer
//! (dwMaxVideoFrameSize), and frames are gathered up to it; so it is held
//! to what the frame it agreed to can be, four bytes a pixel at most (YUY2
//! takes two, MJPEG less), and to what the driver's heap can hold.

use crate::desc::Camera;
use crate::probe::Probe;

const BYTES_A_PIXEL: u64 = 4;
/// The heap in quarters: the frame being gathered and the last whole one
/// take one each, and half is left for all else the driver holds.
const HEAP_SHARES: u64 = 4;

/// Why a bound was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// A format or frame the camera never described, or a bound larger
    /// than that frame can be.
    Undescribed,
    /// A frame larger than the driver's heap holds (`frame_cap`).
    TooLarge,
}

/// The most bytes a frame may take in a driver whose heap is `heap` bytes.
pub const fn frame_cap(heap: u64) -> u64 {
    heap / HEAP_SHARES
}

/// The bound for `agreed`, at most `cap` bytes.
pub fn frame_bound(cam: &Camera, agreed: &Probe, cap: u64) -> Result<u32, Refused> {
    let f = cam.formats.iter().find(|f| f.index == agreed.format);
    let fr = f.and_then(|f| f.frames.iter().find(|x| x.index == agreed.frame));
    let fr = fr.ok_or(Refused::Undescribed)?;
    let most = u64::from(fr.width) * u64::from(fr.height) * BYTES_A_PIXEL;
    let named = [agreed.max_frame, fr.max_bytes].into_iter().find(|&b| b != 0);
    let bound = named.map_or(most, u64::from);
    if bound > most {
        Err(Refused::Undescribed)
    } else if bound > cap.min(u64::from(u32::MAX)) {
        Err(Refused::TooLarge)
    } else {
        Ok(bound as u32)
    }
}
