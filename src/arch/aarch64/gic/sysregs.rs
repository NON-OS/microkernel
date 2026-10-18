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

/// `ID_AA64PFR0_EL1.GIC`, bits 27:24: non-zero when this CPU implements the
/// GICv3 system-register CPU interface.
pub(super) fn present() -> bool {
    let pfr0: u64;
    // SAFETY: an identification register, readable at EL1 with no side effects.
    unsafe {
        asm!("mrs {}, id_aa64pfr0_el1", out(reg) pfr0, options(nomem, nostack, preserves_flags));
    }
    (pfr0 >> 24) & 0xF != 0
}
