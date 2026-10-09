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

use super::super::sfnt_header::sfnt_header;

/// The sum of a table's big-endian words, zero padded (OpenType 5.2).
fn checksum(data: &[u8]) -> u32 {
    data.chunks(4).fold(0u32, |sum, w| {
        let mut word = [0u8; 4];
        word[..w.len()].copy_from_slice(w);
        sum.wrapping_add(u32::from_be_bytes(word))
    })
}

/// Lay out an sfnt from finished tables: the directory sorted by tag,
/// the data in the order given, each table padded to four bytes, and the
/// head's checkSumAdjustment set over the whole font.
pub(super) fn assemble(
    flavor: [u8; 4],
    tables: &mut [([u8; 4], Vec<u8>)],
    max: usize,
) -> Option<Vec<u8>> {
    let n = tables.len();
    let mut offset = 12 + 16 * n;
    let mut entries: Vec<([u8; 4], u32, u32, u32)> = Vec::new();
    entries.try_reserve_exact(n).ok()?;
    for (tag, data) in tables.iter_mut() {
        if tag == b"head" {
            data.get_mut(8..12)?.fill(0);
        }
        entries.push((*tag, checksum(data), offset as u32, data.len() as u32));
        offset = offset.checked_add(data.len().next_multiple_of(4))?;
    }
    if offset > max {
        return None;
    }
    entries.sort_unstable_by_key(|e| e.0);
    let mut out = sfnt_header(&flavor, n);
    out.try_reserve_exact(offset - out.len()).ok()?;
    for (tag, sum, at, len) in &entries {
        out.extend_from_slice(tag);
        [sum, at, len].iter().for_each(|v| out.extend_from_slice(&v.to_be_bytes()));
    }
    let mut head = None;
    for (tag, data) in tables.iter() {
        if tag == b"head" {
            head = Some(out.len());
        }
        out.extend_from_slice(data);
        out.resize(out.len().next_multiple_of(4), 0);
    }
    if let Some(at) = head {
        let adjust = 0xb1b0_afbau32.wrapping_sub(checksum(&out));
        out[at + 8..at + 12].copy_from_slice(&adjust.to_be_bytes());
    }
    Some(out)
}
