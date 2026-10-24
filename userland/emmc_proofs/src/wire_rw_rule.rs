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

//! The head of a READ or WRITE as the wire takes it (PLAN.md section 4),
//! stated apart from the drivers' parser.

use crate::protocol::{E_INVAL, E_MSGSIZE, E_NXIO};

/// The rule: a short head is E_MSGSIZE, no sectors E_INVAL, more than the
/// buffer holds E_MSGSIZE, a run past the end (or wrapping) E_NXIO.
pub fn rule(body: &[u8], capacity: u64, max: u32) -> Result<(u64, u32), i32> {
    if body.len() < 12 {
        return Err(E_MSGSIZE);
    }
    let lba = u64::from_le_bytes(body[..8].try_into().unwrap());
    let n = u32::from_le_bytes(body[8..12].try_into().unwrap());
    match () {
        _ if n == 0 => Err(E_INVAL),
        _ if n > max => Err(E_MSGSIZE),
        _ if u128::from(lba) + u128::from(n) > u128::from(capacity) => Err(E_NXIO),
        _ => Ok((lba, n)),
    }
}
