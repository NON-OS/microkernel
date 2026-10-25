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

//! rt_sigtimedwait's timeout, read out of the guest.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const NSEC: u64 = 1_000_000_000;

/// A relative timespec, in whole milliseconds rounded up.
pub(super) fn wait_ms(guest: &Guest, at: u64) -> Result<u64, u64> {
    let raw = guest.read(at, 16).ok_or(errno::fail(errno::EFAULT))?;
    let secs = u64::from_le_bytes(raw[..8].try_into().unwrap_or([0; 8]));
    let nanos = u64::from_le_bytes(raw[8..].try_into().unwrap_or([0; 8]));
    if nanos >= NSEC || secs > i64::MAX as u64 {
        return Err(errno::fail(errno::EINVAL));
    }
    Ok(secs.saturating_mul(1000).saturating_add(nanos.div_ceil(1_000_000)))
}
