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

use super::bits::Bools;

/// The token partitions: 1, 2, 4 or 8, all but the last sized by 3-byte
/// little-endian lengths ahead of them.
pub(super) fn partitions<'a>(br: &mut Bools, buf: &'a [u8]) -> Option<Vec<&'a [u8]>> {
    let n = 1usize << br.literal(2);
    let sizes = buf.get(..3 * (n - 1))?;
    let mut at = 3 * (n - 1);
    let mut parts = Vec::with_capacity(n);
    for s in sizes.chunks(3) {
        let len =
            (s[0] as usize | (s[1] as usize) << 8 | (s[2] as usize) << 16).min(buf.len() - at);
        parts.push(&buf[at..at + len]);
        at += len;
    }
    parts.push(&buf[at..]);
    Some(parts)
}
