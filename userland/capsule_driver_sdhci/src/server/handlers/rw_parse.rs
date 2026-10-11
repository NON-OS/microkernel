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

//! The head of a READ or WRITE, held to the disk and to one request.

use crate::protocol::{E_INVAL, E_MSGSIZE, E_NXIO, RW_HEADER_LEN};

/// The first sector and the count, or the status refusing them: a short
/// head, no sectors, more than one request moves, or a run past the end.
pub fn parse(body: &[u8], capacity: u64, max: u32) -> Result<(u64, u32), i32> {
    let head = body.get(..RW_HEADER_LEN).ok_or(E_MSGSIZE)?;
    let lba = u64::from_le_bytes(head[0..8].try_into().map_err(|_| E_MSGSIZE)?);
    let n = u32::from_le_bytes(head[8..12].try_into().map_err(|_| E_MSGSIZE)?);
    if n == 0 {
        return Err(E_INVAL);
    }
    if n > max {
        return Err(E_MSGSIZE);
    }
    let end = lba.checked_add(n as u64).ok_or(E_NXIO)?;
    if end > capacity {
        return Err(E_NXIO);
    }
    Ok((lba, n))
}
