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

use super::bytes::{le16, le32};
use super::probe::{Format, Probe};

/* WebP dimensions from the first chunk after the RIFF/WEBP header. VP8X
 * carries a 24-bit canvas size, VP8L packs 14-bit dims after a signature
 * byte, and lossy VP8 reads them from the keyframe header. */
pub(super) fn webp_dims(b: &[u8]) -> Option<Probe> {
    let fourcc = b.get(12..16)?;
    match fourcc {
        b"VP8X" => {
            let w = 1 + (le16(b, 24)? | ((b.get(26).copied()? as u32) << 16));
            let h = 1 + (le16(b, 27)? | ((b.get(29).copied()? as u32) << 16));
            Some(Probe { format: Format::Webp, w, h })
        }
        b"VP8L" => {
            let bits = le32(b, 21)?;
            let w = (bits & 0x3FFF) + 1;
            let h = ((bits >> 14) & 0x3FFF) + 1;
            Some(Probe { format: Format::Webp, w, h })
        }
        b"VP8 " => {
            /* Skip the 10-byte frame tag and the 3-byte start code to the
             * 14-bit width and height words. */
            let w = le16(b, 26)? & 0x3FFF;
            let h = le16(b, 28)? & 0x3FFF;
            Some(Probe { format: Format::Webp, w, h })
        }
        _ => None,
    }
}
