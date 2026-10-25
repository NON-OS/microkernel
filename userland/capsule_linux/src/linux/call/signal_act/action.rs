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

//! `struct sigaction` as the guest passes it, and the signal rt_sigaction
//! may name. Pure, over the guest's memory, so the host proofs hold it.

use crate::linux::abi::errno;
use crate::linux::guest::sigstate::{blockable, SigAction, NSIG, SIGKILL, SIGSTOP};

/// handler, flags, restorer, then a kernel sigset_t.
pub const SIGACTION_LEN: usize = 32;

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
