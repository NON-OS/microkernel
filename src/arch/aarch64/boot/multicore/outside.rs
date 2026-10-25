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

//! A CPU running that the roster does not hold: one past MAX_CPUS, one the
//! tree never listed, or one started by mistake. Any index it took would be
//! another CPU's, so it stops before it touches a per-CPU table.

use crate::sys::serial::Line;

/// Say which CPU, once, and wait for ever with interrupts masked. The line
/// goes through the console's own lock, which no per-CPU table backs.
pub fn park_outside_roster(affinity: u64) -> ! {
    Line::new()
        .str(b"[SMP] CPU with affinity ")
        .hex(affinity)
        .str(b" is not in the roster; parked")
        .end();
    loop {
        // SAFETY: masking and waiting only.
        unsafe { core::arch::asm!("msr daifset, #0xf", "wfi", options(nomem, nostack)) };
    }
}
