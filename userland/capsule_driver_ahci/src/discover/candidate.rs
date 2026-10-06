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

//! Whether a device record is an AHCI controller the driver can map.

use nonos_libc::{DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use super::rule::{abar_usable, is_ahci_function};
use crate::constants::{AHCI_ABAR_BAR, CLASS_BLOCK};

pub(super) fn is_candidate(r: &DeviceRecord) -> bool {
    let bar = r.bars[AHCI_ABAR_BAR as usize];
    r.bus_kind == BUS_KIND_PCI
        && r.class == CLASS_BLOCK
        && is_ahci_function(r.pci_class, r.pci_subclass, r.vendor, r.device)
        && r.bar_count > AHCI_ABAR_BAR
        // The IRQ routing is intentionally not part of the match. The I/O
        // path polls the port completion registers and never waits on the
        // interrupt, so a controller with no legacy PIC routing (irq_line
        // reported as 0xff, common on APIC/MSI laptop platforms) is still a
        // valid candidate. Requiring a routed line here rejected working
        // SATA controllers on real hardware.
        && bar.kind == BAR_KIND_MMIO
        // A 2 KiB ABAR at a base inside a page is common on Intel PCHs; the
        // broker maps it by its page.
        && abar_usable(bar.size)
}
