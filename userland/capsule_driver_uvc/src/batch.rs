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
//! A collected batch read into its payloads: each interval's length from
//! its slot, the bytes packed after the slots in order. A batch that
//! claims more bytes than it carries is read no further.

pub const SLOTS: usize = 31;

/// The payloads in `data` (an OP_ISOCH_IN reply's data), in order.
pub fn payloads(data: &[u8]) -> impl Iterator<Item = &[u8]> {
    let count = data.get(1).map_or(0, |&n| (n as usize).min(SLOTS));
    let lens = data.get(4..4 + 2 * SLOTS).unwrap_or(&[]);
    let mut at = 4 + 2 * SLOTS;
    (0..count).map_while(move |i| {
        let n = u16::from_le_bytes([*lens.get(2 * i)?, *lens.get(2 * i + 1)?]) as usize;
        let p = data.get(at..at + n)?;
        at += n;
        Some(p)
    })
}
