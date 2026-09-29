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

//! Signal dispositions and masks, recorded here and acted on in
//! `serve::deliver`: a handler is kept with its flags, restorer and mask, and
//! each thread has the mask of signals it holds back (signal_mask).
use crate::linux::abi::errno;
use crate::linux::guest::sigstate::{SigAction, NSIG, SIGKILL, SIGSTOP};
use crate::linux::guest::Guest;

/* `struct sigaction` is 32 bytes; a kernel sigset_t is 8. */
const SIGACTION_LEN: usize = 32;
pub const SIGSET_LEN: u64 = 8;

pub fn rt_sigaction(guest: &mut Guest, signum: u64, act: u64, old: u64, size: u64) -> u64 {
    let unchangeable = signum == u64::from(SIGKILL) || signum == u64::from(SIGSTOP);
    if size != SIGSET_LEN || signum == 0 || signum > NSIG as u64 || (act != 0 && unchangeable) {
        return errno::fail(errno::EINVAL);
    }
    let n = signum as usize;
    let was = guest.signals.action(n).unwrap_or_default();
    let new = match act {
        0 => None,
        at => match guest.read(at, SIGACTION_LEN) {
            Some(raw) => Some(decode(&raw)),
            None => return errno::fail(errno::EFAULT),
        },
    };
    if old != 0 && guest.write(old, &encode(was)) < SIGACTION_LEN as i64 {
        return errno::fail(errno::EFAULT);
    }
    if let Some(a) = new {
        guest.signals.set(n, a);
        /* A signal set to be ignored is dropped where it already waits. */
        if guest.signals.discards(n as u8) {
            guest.signals.discard(n as u8);
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
