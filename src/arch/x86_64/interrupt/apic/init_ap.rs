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

//! Per-CPU LAPIC bring-up for an application processor. Every CPU has its own
//! LAPIC that comes out of INIT/SIPI software-disabled, so each AP must enable
//! and program its own registers. The mode and the MMIO mapping are global and
//! were adopted from the BSP before the SIPI was sent; only the register
//! programming happens here, per CPU.
//!
//! The programming is `sys::apic`'s, the same the boot CPU ran, so the two
//! cannot drift. They did: this file used to set LINT0 to NMI, unmask thermal
//! and error onto 0x21 and 0x22 (the keyboard and cascade vectors, the second
//! with no gate at all), and in x2APIC mode enable directed-EOI suppression,
//! which strands every level-triggered I/O APIC line routed to an AP.

pub unsafe fn init_ap_lapic() {
    crate::sys::apic::init_ap_local_apic();
}
