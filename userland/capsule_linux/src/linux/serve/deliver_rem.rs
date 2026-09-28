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

//! A relative sleep cut short by a handler writes the time it had left, as
//! Linux's nanosleep and relative clock_nanosleep do; an absolute one writes
//! nothing, as on Linux.

use nonos_libc::ForeignRegs;

use super::deliver_restart::{R10, RAX, RSI};
use crate::linux::call::now_ms;
use crate::linux::guest::{Guest, Parked};

const CLOCK_MONOTONIC: u64 = 1;
const TIMER_ABSTIME: u64 = 1;
const NANOSLEEP: u64 = 35;
const CLOCK_NANOSLEEP: u64 = 230;

/// A relative sleep cut short writes what was left of it where it was asked.
pub fn write_rem(guest: &Guest, regs: &ForeignRegs, parked: Parked) {
    let Parked::Sleep(due) = parked else {
        return;
    };
    let rem = match regs[RAX] {
        NANOSLEEP => regs[RSI],
        CLOCK_NANOSLEEP if regs[RSI] & TIMER_ABSTIME == 0 => regs[R10],
        _ => 0,
    };
    let Some(now) = now_ms(CLOCK_MONOTONIC) else {
        return;
    };
    if rem == 0 {
        return;
    }
    let left = due.saturating_sub(now);
    let mut spec = [0u8; 16];
    spec[..8].copy_from_slice(&(left / 1000).to_le_bytes());
    spec[8..].copy_from_slice(&((left % 1000) * 1_000_000).to_le_bytes());
    let _ = guest.write(rem, &spec);
}
