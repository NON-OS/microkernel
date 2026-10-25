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

//! The counters and clocks keygen stirs in: cycle counter, stack, PIT, RTC.

#[inline]
pub(super) fn read_cycle_counter() -> u64 {
    crate::arch::read_time_counter()
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub(super) fn get_stack_pointer() -> u64 {
    let rsp: u64;
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) rsp, options(nomem, nostack));
    }
    rsp
}
#[cfg(not(target_arch = "x86_64"))]
#[inline]
pub(super) fn get_stack_pointer() -> u64 {
    0
}

#[cfg(target_arch = "x86_64")]
pub(super) fn read_pit_counter() -> u16 {
    const PIT_CHANNEL0: u16 = 0x40;
    const PIT_COMMAND: u16 = 0x43;
    const LATCH_CHANNEL0: u8 = 0x00;
    unsafe {
        core::arch::asm!("out dx, al", in("dx") PIT_COMMAND, in("al") LATCH_CHANNEL0, options(nostack, preserves_flags, nomem));
        let low: u8;
        core::arch::asm!("in al, dx", out("al") low, in("dx") PIT_CHANNEL0, options(nostack, preserves_flags, nomem));
        let high: u8;
        core::arch::asm!("in al, dx", out("al") high, in("dx") PIT_CHANNEL0, options(nostack, preserves_flags, nomem));
        ((high as u16) << 8) | (low as u16)
    }
}
#[cfg(not(target_arch = "x86_64"))]
pub(super) fn read_pit_counter() -> u16 {
    0
}

pub(super) fn read_rtc_timestamp() -> u64 {
    crate::arch::wall_clock::unix_timestamp().unwrap_or(0)
}
