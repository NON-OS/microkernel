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

/* The offset a call names, or the descriptor's own. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub(super) fn read_offset(guest: &Guest, ptr: u64) -> Result<Option<u64>, u64> {
    if ptr == 0 {
        return Ok(None);
    }
    let raw = guest.read(ptr, 8).ok_or(errno::fail(errno::EFAULT))?;
    let at = i64::from_le_bytes(raw.try_into().unwrap_or([0; 8]));
    if at < 0 {
        return Err(errno::fail(errno::EINVAL));
    }
    Ok(Some(at as u64))
}
