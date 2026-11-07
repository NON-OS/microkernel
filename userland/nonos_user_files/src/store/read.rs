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

//! Reading an object range whole.

use crate::errno::Errno;
use crate::host::Objects;

/// Fills `buf` from `offset`, stopping early only at the object's end.
/// Returns how much was filled.
pub fn read_full(
    o: &mut impl Objects,
    id: u64,
    offset: u64,
    buf: &mut [u8],
) -> Result<usize, Errno> {
    let mut done = 0;
    while done < buf.len() {
        let got = o.read(id, offset + done as u64, &mut buf[done..])?;
        if got == 0 {
            break;
        }
        done += got;
    }
    Ok(done)
}
