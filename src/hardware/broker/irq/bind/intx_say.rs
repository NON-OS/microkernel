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

//! The line an INTx bind programmed, said once, so a photo shows the pin,
//! the vector and how the IO-APIC senses it: level and active-low for a PCI
//! INTx above GSI 15, edge and active-high for an ISA line (ioapic line_mode).

use crate::arch::interrupt::ioapic::Rte;
use crate::hardware::broker::pci_index;

pub(super) fn say_route(device_id: u64, gsi: u32, rte: &Rte) {
    let trigger = if rte.level_trigger { "level" } else { "edge" };
    let polarity = if rte.active_low { "active-low" } else { "active-high" };
    match pci_index::lookup(device_id) {
        Some(h) => crate::log::info!(
            "[IRQ] {} intx gsi {} vector {:#x} {} {}",
            h.address,
            gsi,
            rte.vector,
            trigger,
            polarity
        ),
        None => crate::log::info!(
            "[IRQ] device {} intx gsi {} vector {:#x} {} {}",
            device_id,
            gsi,
            rte.vector,
            trigger,
            polarity
        ),
    }
}
