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

//! Signal dispositions, recorded here and delivered on the return path in
//! `serve::deliver`: a handler is kept with its flags, restorer and mask.
use crate::linux::abi::errno;
use crate::linux::guest::sigstate::{SigAction, NSIG};
use crate::linux::guest::Guest;

// SIGKILL/SIGSTOP cannot be caught; `struct sigaction` is 32 bytes.
const SIGKILL: u64 = 9;
const SIGSTOP: u64 = 19;
const SIGACTION_LEN: usize = 32;

pub fn rt_sigaction(guest: &mut Guest, signum: u64, act: u64, old: u64) -> u64 {
    if signum == 0 || signum > NSIG as u64 || signum == SIGKILL || signum == SIGSTOP {
        return errno::fail(errno::EINVAL);
    }
    let n = signum as usize;
    if old != 0
        && guest.write(old, &encode(guest.signals.action(n).unwrap_or_default()))
            < SIGACTION_LEN as i64
    {
        return errno::fail(errno::EFAULT);
    }
    if act != 0 {
        match guest.read(act, SIGACTION_LEN) {
            Some(raw) => guest.signals.set(n, decode(&raw)),
            None => return errno::fail(errno::EFAULT),
        }
    }
    errno::ok(0)
}

fn decode(raw: &[u8]) -> SigAction {
    let w = |i: usize| u64::from_le_bytes(raw[i..i + 8].try_into().unwrap_or([0; 8]));
    SigAction { handler: w(0), flags: w(8), restorer: w(16), mask: w(24) }
}
fn encode(a: SigAction) -> [u8; SIGACTION_LEN] {
    let mut b = [0u8; SIGACTION_LEN];
    b[0..8].copy_from_slice(&a.handler.to_le_bytes());
    b[8..16].copy_from_slice(&a.flags.to_le_bytes());
    b[16..24].copy_from_slice(&a.restorer.to_le_bytes());
    b[24..32].copy_from_slice(&a.mask.to_le_bytes());
    b
}

/// The old mask reads back empty: nothing is held back, delivery ignores it.
pub fn rt_sigprocmask(guest: &Guest, old: u64) -> u64 {
    if old != 0 && guest.write(old, &[0u8; 8]) < 8 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}

/// The old alternate stack reads back unset; a handler uses the own stack.
pub fn sigaltstack(guest: &Guest, old: u64) -> u64 {
    if old != 0 && guest.write(old, &[0u8; 24]) < 24 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}
