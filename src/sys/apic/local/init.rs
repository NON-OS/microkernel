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

use core::sync::atomic::Ordering;

use crate::sys::serial;

use super::constants::{LAPIC_ID, LAPIC_VERSION};
use super::regs::lapic_read_raw;
use super::state::{LAPIC_BASE, LAPIC_INIT, LAPIC_PHYS};

pub fn init_local_apic() {
    if LAPIC_INIT.load(Ordering::Relaxed) {
        return;
    }
    serial::println(b"[APIC] Initializing Local APIC...");

    // Detect x2APIC hand-off before touching any register: if firmware left
    // the APIC in x2APIC mode the accessors below must use MSRs, not MMIO.
    // This also reads where the register page is: firmware may move it off
    // 0xFEE00000, and IA32_APIC_BASE is the only place that says so.
    super::x2apic::detect_mode();

    // Boot phase: still on identity-mapped physical. VM init will republish
    // to the kernel-half UC virtual page via rebind.
    LAPIC_BASE.store(LAPIC_PHYS.load(Ordering::Acquire), Ordering::SeqCst);

    // SAFETY: the APIC is enabled in the latched mode, and in xAPIC mode the
    // base above is the identity-mapped register page.
    unsafe { super::program::program_local_vectors(true) };

    LAPIC_INIT.store(true, Ordering::SeqCst);

    let version = unsafe { lapic_read_raw(LAPIC_VERSION) };
    let raw_id = unsafe { lapic_read_raw(LAPIC_ID) };
    let x2 = super::x2apic::is_x2(Ordering::Acquire);
    let id = if x2 { raw_id } else { raw_id >> 24 };
    serial::print(b"[APIC] Local APIC enabled, ID=");
    serial::print_dec(id as u64);
    serial::print(b" Version=0x");
    serial::print_hex((version & 0xFF) as u64);
    serial::print(if x2 { b" mode=x2APIC" } else { b" mode=xAPIC" });
    serial::println(b"");
}
