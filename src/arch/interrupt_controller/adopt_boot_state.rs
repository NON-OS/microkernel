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

//! Take over the boot CPU's controller state before the first IPI.
//!
//! On x86_64 the boot path set the local APIC up through `sys::apic`, and the
//! IPI path keeps its own copy of the mode and base, empty until adopted. The
//! GIC and the CLINT are reached the same way from every caller, so there is
//! nothing to take over and the answer is always yes.

/// Whether the controller is ready to send the first IPI.
pub(crate) fn adopt_boot_state() -> bool {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::interrupt::apic::adopt_bsp_state().is_ok();
    #[cfg(not(target_arch = "x86_64"))]
    return true;
}
