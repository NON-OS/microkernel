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

//! Whether a table entry is one NONOS wrote: its name is one of the four
//! this writer gives its partitions. Entry 0 of a disk written before the
//! store had a partition of its own is `NONOS-ESP`, which counts too.

use crate::layout::Region;

/// `entry` is one 128-byte GPT entry as it sits on the disk.
pub fn written_by_nonos(entry: &[u8]) -> bool {
    if entry.len() < 128 {
        return false;
    }
    let units = entry[56..128].chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]]));
    let name = units.take_while(|&u| u != 0);
    Region::ALL.iter().any(|r| name.clone().eq(r.name().encode_utf16()))
}
