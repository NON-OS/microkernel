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
//! to what the frame it agreed to can be: four bytes a pixel at most (YUY2
//! takes two, MJPEG less), and never past MAX_FRAME_BYTES.

use crate::desc::Camera;
use crate::probe::Probe;

const BYTES_A_PIXEL: u64 = 4;
/// A 3840x2160 frame at four bytes a pixel: the largest this driver holds.
pub const MAX_FRAME_BYTES: u64 = 3840 * 2160 * 4;

/// The bound for `agreed`, or None when it names a format or frame the
/// camera never described, or a bound larger than that frame can be.
pub fn frame_bound(cam: &Camera, agreed: &Probe) -> Option<u32> {
    let f = cam.formats.iter().find(|f| f.index == agreed.format)?;
    let fr = f.frames.iter().find(|x| x.index == agreed.frame)?;
    let pixels = u64::from(fr.width) * u64::from(fr.height);
    let most = (pixels * BYTES_A_PIXEL).min(MAX_FRAME_BYTES);
    let named = [agreed.max_frame, fr.max_bytes].into_iter().find(|&b| b != 0);
    let bound = named.map_or(most, u64::from);
    (bound <= most).then_some(bound as u32)
}
