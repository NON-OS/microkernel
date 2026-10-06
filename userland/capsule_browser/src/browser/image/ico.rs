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

pub(super) use super::ico_dib::decode_dib;

/// The picture an ICO or CUR file holds: its largest entry (the deepest
/// colour among equal sizes), as (width, height, image bytes). The bytes
/// are a whole PNG file or a headerless DIB.
pub(super) fn pick(b: &[u8]) -> Option<(u32, u32, &[u8])> {
    let le16 = |i: usize| Some(u16::from_le_bytes(b.get(i..i + 2)?.try_into().ok()?) as u32);
    let le32 = |i: usize| Some(u32::from_le_bytes(b.get(i..i + 4)?.try_into().ok()?) as usize);
    if le16(0)? != 0 || !matches!(le16(2)?, 1 | 2) {
        return None;
    }
    let mut best: Option<(u32, u32, u32, &[u8])> = None;
    for i in 0..le16(4)?.min(256) as usize {
        let e = 6 + 16 * i;
        let side = |v: u8| if v == 0 { 256 } else { v as u32 };
        let (w, h) = (side(*b.get(e)?), side(*b.get(e + 1)?));
        let (len, off) = (le32(e + 8)?, le32(e + 12)?);
        let Some(data) = b.get(off..off.checked_add(len)?) else { continue };
        let depth = if data.starts_with(b"\x89PNG") { 32 } else { le16(off + 14)? };
        let better = best.is_none_or(|(bw, bh, bd, _)| (w * h, depth) > (bw * bh, bd));
        if better {
            best = Some((w, h, depth, data));
        }
    }
    best.map(|(w, h, _, data)| (w, h, data))
}

/// Whether the bytes open like an icon or cursor directory.
pub(super) fn is_ico(b: &[u8]) -> bool {
    b.len() >= 22 && b[..2] == [0, 0] && matches!(b[2..4], [1, 0] | [2, 0]) && b[4..6] != [0, 0]
}
