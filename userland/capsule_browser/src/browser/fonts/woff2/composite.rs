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

use alloc::vec::Vec;

use super::streams::Streams;

/// Append a composite glyph (WOFF2 5.1 steps 1a-3a): its stored box,
/// the component records as they came, and instructions if any asked.
pub(super) fn composite(s: &mut Streams, bbox: &[u8], out: &mut Vec<u8>) -> Option<i16> {
    out.extend_from_slice(&[0xff, 0xff]);
    out.extend_from_slice(bbox);
    let mut code = false;
    loop {
        let flags = s.composites.u16()?;
        code |= flags & 0x100 != 0;
        let args = if flags & 1 != 0 { 4 } else { 2 };
        let scale = [(8, 2), (0x40, 4), (0x80, 8)].iter().find(|(b, _)| flags & b != 0);
        out.extend_from_slice(&flags.to_be_bytes());
        out.extend_from_slice(s.composites.bytes(2 + args + scale.map_or(0, |x| x.1))?);
        if flags & 0x20 == 0 {
            break;
        }
    }
    if code {
        let len = s.glyphs.u255()?;
        out.extend_from_slice(&len.to_be_bytes());
        out.extend_from_slice(s.code.bytes(len as usize)?);
    }
    Some(i16::from_be_bytes([bbox[0], bbox[1]]))
}
