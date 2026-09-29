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

use super::bits::Bits;
use super::out::Out;
use super::types::End;

/// A stored block: byte-aligned LEN, its complement NLEN, then LEN bytes
/// copied as they are.
pub fn stored(b: &mut Bits, out: &mut Out) -> Result<(), End> {
    let at = b.align_to_bytes();
    let d = b.input();
    let Some(h) = d.get(at..at + 4) else {
        return Err(End::Truncated);
    };
    let len = u16::from_le_bytes([h[0], h[1]]);
    if len != !u16::from_le_bytes([h[2], h[3]]) {
        return Err(End::Corrupt);
    }
    let len = usize::from(len);
    let data = &d[at + 4..];
    let want = len.min(data.len());
    let n = out.extend(&data[..want]);
    b.skip_to(at + 4 + n);
    if n == len {
        Ok(())
    } else if n < want {
        Err(End::Capped)
    } else {
        Err(End::Truncated)
    }
}
