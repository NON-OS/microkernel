// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Stopping the CPU.

/// Every exception masked and waiting for an interrupt that cannot arrive,
/// for good.
#[inline(always)]
pub fn halt_forever() -> ! {
    loop {
        // SAFETY: the loader is done; nothing runs after this.
        unsafe { core::arch::asm!("msr daifset, #0xf", "wfi", options(nomem, nostack)) };
    }
}

/// No architected reset without knowing the PSCI conduit, which the device
/// tree names; until the loader reads it, a reset request halts instead.
#[inline(always)]
pub fn reset() -> ! {
    halt_forever()
}
