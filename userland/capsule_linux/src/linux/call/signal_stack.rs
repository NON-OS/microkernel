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

//! `sigaltstack`: the calling thread's alternate signal stack, where a
//! handler asking for SA_ONSTACK runs. `stack_t` is ss_sp, a 4-byte ss_flags
//! and its padding, then ss_size.

use nonos_libc::{mk_foreign_context, ForeignRegs};

use crate::linux::abi::errno;
use crate::linux::guest::sigthread::{SS_DISABLE, SS_ONSTACK};
use crate::linux::guest::Guest;

const STACK_T_LEN: usize = 24;
/// MINSIGSTKSZ on x86-64.
const MINSIGSTKSZ: u64 = 2048;
/// SS_AUTODISARM, accepted and kept as Linux accepts it.
const SS_AUTODISARM: u64 = 1 << 31;
const RSP: usize = 15;

pub fn sigaltstack(guest: &mut Guest, tid: u32, ss: u64, old: u64) -> u64 {
    let [sp, flags, size] = guest.signals.thread(tid).alt;
    let mut regs: ForeignRegs = [0; 18];
    let rsp = if mk_foreign_context(tid, &mut regs) == 0 { regs[RSP] } else { 0 };
    let on = flags & SS_DISABLE == 0 && rsp.wrapping_sub(sp) < size;
    if old != 0 {
        let now = if on { SS_ONSTACK } else { flags & (SS_DISABLE | SS_AUTODISARM) };
        let mut b = [0u8; STACK_T_LEN];
        b[..8].copy_from_slice(&sp.to_le_bytes());
        b[8..12].copy_from_slice(&(now as u32).to_le_bytes());
        b[16..].copy_from_slice(&size.to_le_bytes());
        if guest.write(old, &b) < STACK_T_LEN as i64 {
            return errno::fail(errno::EFAULT);
        }
    }
    if ss == 0 {
        return errno::ok(0);
    }
    let Some(raw) = guest.read(ss, STACK_T_LEN) else {
        return errno::fail(errno::EFAULT);
    };
    let word = |i: usize| u64::from_le_bytes(raw[i..i + 8].try_into().unwrap_or([0; 8]));
    let new_flags = u64::from(u32::from_le_bytes(raw[8..12].try_into().unwrap_or([0; 4])));
    if on {
        return errno::fail(errno::EPERM);
    }
    /* SS_ONSTACK asks for the same as 0: an enabled stack. */
    let alt = match new_flags & !(SS_AUTODISARM | SS_ONSTACK) {
        SS_DISABLE => [0, SS_DISABLE, 0],
        0 if word(16) < MINSIGSTKSZ => return errno::fail(errno::ENOMEM),
        0 => [word(0), new_flags & SS_AUTODISARM, word(16)],
        _ => return errno::fail(errno::EINVAL),
    };
    guest.signals.thread(tid).alt = alt;
    errno::ok(0)
}
