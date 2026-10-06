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

use crate::browser::image::shrink::fit;
use crate::browser::image::store::Decoded;

/// The chunks a still WebP is drawn from.
#[derive(Default)]
struct Chunks<'a> {
    vp8: Option<&'a [u8]>,
    vp8l: Option<&'a [u8]>,
    alph: Option<&'a [u8]>,
}

/// Decode a WebP file, simple or extended (VP8X), to `size` (never larger
/// than natural): a lossless VP8L image, or a lossy VP8 key frame with its
/// ALPH plane when the file carries one. Animations are not decoded.
pub fn decode_webp(bytes: &[u8], size: (u32, u32)) -> Option<Decoded> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WEBP" {
        return None;
    }
    let mut c = Chunks::default();
    let mut off = 12usize;
    while off + 8 <= bytes.len() {
        let fourcc = &bytes[off..off + 4];
        let len = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().ok()?) as usize;
        let end = off.checked_add(8)?.checked_add(len)?;
        let payload = bytes.get(off + 8..end.min(bytes.len()))?;
        match fourcc {
            b"VP8L" => c.vp8l = c.vp8l.or(Some(payload)),
            b"VP8 " => c.vp8 = c.vp8.or(Some(payload)),
            b"ALPH" => c.alph = c.alph.or(Some(payload)),
            b"ANIM" | b"ANMF" => return None,
            _ => {}
        }
        /* Chunk payloads are padded to an even length. */
        off = end + (len & 1);
    }
    match (c.vp8l, c.vp8) {
        (Some(p), _) => fit(super::vp8l::decode_vp8l(p)?, size).ok(),
        (None, Some(p)) => super::vp8::decode_vp8(p, c.alph, size),
        (None, None) => None,
    }
}
