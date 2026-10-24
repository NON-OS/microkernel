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

use core::arch::asm;

/// Counter ticks per second: the rate `init_timer` latched, and before it
/// runs, CNTFRQ_EL0 itself. Every conversion reads this, so a rate taken
/// from the device tree, when the register reads 0, reaches all of them.
pub fn frequency() -> u64 {
    match super::super::state::frequency() {
        0 => cntfrq(),
        hz => hz,
    }
}

/// CNTFRQ_EL0 as firmware left it. It is only a register firmware writes for
/// software to read (Arm ARM D11.1.2); the counter runs whatever it says.
pub(in crate::arch::aarch64::timer) fn cntfrq() -> u64 {
    let freq: u64;
    // SAFETY: CNTFRQ_EL0 is readable at EL1 and reading it has no effect.
    unsafe {
        asm!("mrs {}, cntfrq_el0", out(reg) freq, options(nomem, nostack, preserves_flags));
    }
    freq
}
