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

use super::set_irq::ioapic_set_irq;

// Convenience wrapper: route `irq` to `vector` on this CPU (the boot CPU at
// boot), no extra redir flags, edge-triggered active-high (ACPI ISOs still
// override). The destination used to be a literal 0, which is the boot CPU
// only where its APIC id happens to be 0; on a machine where it is not, every
// such line went to some other CPU or to none.
pub fn enable_irq(irq: u8, vector: u8) {
    let own = crate::sys::apic::local_apic_id().unwrap_or(0);
    let Some(dest) = crate::arch::x86_64::interrupt::apic::device_irq_dest(own) else {
        crate::sys::serial::println(b"[APIC] ERROR: no CPU addressable for an IOAPIC route");
        return;
    };
    ioapic_set_irq(irq, vector, dest as u8, 0);
}
