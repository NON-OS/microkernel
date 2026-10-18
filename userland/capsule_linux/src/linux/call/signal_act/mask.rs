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

//! The mask a waiting call waits under, read from the guest. Pure, over the
//! guest's memory, so the host proofs hold it.

use crate::linux::abi::errno;
use crate::linux::guest::memory::Memory;
use crate::linux::guest::sigstate::blockable;

pub const SIGSET_LEN: u64 = 8;

/// The mask a ppoll, pselect6, epoll_pwait or epoll_pwait2 waits under, as
/// set_user_sigmask reads it: none for a null pointer, EINVAL for a size
/// other than a kernel sigset_t's, EFAULT when it cannot be read, and never
/// SIGKILL or SIGSTOP.
pub fn wait_mask(mem: &impl Memory, at: u64, size: u64) -> Result<Option<u64>, i64> {
    if at == 0 {
        return Ok(None);
    }
    if size != SIGSET_LEN {
        return Err(errno::EINVAL);
    }
    let raw = mem.read_at(at, 8).ok_or(errno::EFAULT)?;
    let mask =
        u64::from_le_bytes(raw.get(..8).and_then(|b| b.try_into().ok()).ok_or(errno::EFAULT)?);
    Ok(Some(blockable(mask)))
}

/// The set an rt_sigtimedwait takes signals of, read from `at`: never
/// SIGKILL or SIGSTOP, which Linux takes out of it, so no wait can take
/// either in place of the end or the stop it brings.
pub fn wait_set(mem: &impl Memory, at: u64) -> Option<u64> {
    let raw = mem.read_at(at, 8)?;
    Some(blockable(u64::from_le_bytes(raw.get(..8)?.try_into().ok()?)))
}
