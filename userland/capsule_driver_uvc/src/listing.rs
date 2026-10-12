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
//! OP_OPEN's answer: the camera's ids, then each format (index u8, kind
//! u8: 1 uncompressed, 2 MJPEG; FourCC 4 bytes, zero for MJPEG; frame
//! count u8) followed by its frames (index u8, width u16, height u16,
//! default interval u32), all little-endian, as many as fit `max` bytes.

use alloc::vec::Vec;

use nonos_uvc::{Camera, FormatKind};

pub fn listing(cam: &Camera, ids: (u16, u16), max: usize) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend(ids.0.to_le_bytes());
    out.extend(ids.1.to_le_bytes());
    out.push(0);
    for f in &cam.formats {
        let (kind, fourcc) = match f.kind {
            FormatKind::Uncompressed { fourcc, .. } => (1, fourcc),
            FormatKind::Mjpeg => (2, [0; 4]),
        };
        let frames = f.frames.len().min(255);
        if out.len() + 7 + 9 * frames > max {
            break;
        }
        out.extend([f.index, kind]);
        out.extend(fourcc);
        out.push(frames as u8);
        for fr in &f.frames[..frames] {
            out.push(fr.index);
            out.extend(fr.width.to_le_bytes());
            out.extend(fr.height.to_le_bytes());
            out.extend(fr.interval.to_le_bytes());
        }
        out[4] += 1;
    }
    out
}
