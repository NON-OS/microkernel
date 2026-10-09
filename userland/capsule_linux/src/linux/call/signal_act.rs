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

//! `struct sigaction` as the guest passes it, the signal rt_sigaction may
//! name, and the mask a waiting call waits under. Pure, over the guest's
//! memory where it reads, so the host proofs hold all three.

use crate::linux::abi::errno;
use crate::linux::guest::memory::Memory;
use crate::linux::guest::sigstate::{blockable, SigAction, NSIG, SIGKILL, SIGSTOP};

/// handler, flags, restorer, then a kernel sigset_t.
pub const SIGACTION_LEN: usize = 32;
pub const SIGSET_LEN: u64 = 8;

/// The signal `signum` names, as the int it is, or EINVAL: a number outside
/// 1..=64, and a new action for SIGKILL or SIGSTOP, which only ever take
/// their default.
pub fn signal_of(signum: u64, changes: bool) -> Result<usize, i64> {
    let n = signum as u32 as i32;
    if !(1..=NSIG as i32).contains(&n) {
        return Err(errno::EINVAL);
    }
    if changes && (n == i32::from(SIGKILL) || n == i32::from(SIGSTOP)) {
        return Err(errno::EINVAL);
    }
    Ok(n as usize)
}

/// The action at `raw`, with SIGKILL and SIGSTOP taken out of its mask, as
/// Linux stores it, so an old action read back never claims to block them.
pub fn decode(raw: &[u8]) -> Option<SigAction> {
    let w = |i: usize| Some(u64::from_le_bytes(raw.get(i..i + 8)?.try_into().ok()?));
    Some(SigAction { handler: w(0)?, flags: w(8)?, restorer: w(16)?, mask: blockable(w(24)?) })
}

pub fn encode(a: SigAction) -> [u8; SIGACTION_LEN] {
    let mut b = [0u8; SIGACTION_LEN];
    b[0..8].copy_from_slice(&a.handler.to_le_bytes());
    b[8..16].copy_from_slice(&a.flags.to_le_bytes());
    b[16..24].copy_from_slice(&a.restorer.to_le_bytes());
    b[24..32].copy_from_slice(&a.mask.to_le_bytes());
    b
}

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
