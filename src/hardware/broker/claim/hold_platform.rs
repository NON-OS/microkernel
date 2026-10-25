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

//! Claims of the records firmware tables publish (L59). An ACPI or a device
//! tree record names what no PCI id does, and it was open to whichever
//! Driver-capped capsule asked first: once the I2C host driver had exited,
//! any capsule could take the touchpad's controller and its GPIO bank, and a
//! capsule holding the PS/2 ports reads every key. Each kind is held to the
//! service that drives it, by its bus's rule: an ACPI record by its class and
//! identity (`hold_acpi`), the VMBus root to the VMBus core (`hold_virt`), a
//! device-tree record by its interface (`hold_tree`). The ACPI rule's numbers
//! are re-exported here, so the proofs and `hold_ids.rs` read them from one
//! place. Pure, so the proofs run it on the host.

pub(crate) use super::hold_acpi::*;
use super::hold_rule::Hold;
use super::hold_tree::hold_for_tree;
pub(crate) use super::hold_tree::BUS_DEVICE_TREE;
use super::hold_virt::{hold_for_virt, BUS_VIRT};

/// The hold on a record of `bus`, by its class id, PCI base class and
/// identity; None for a bus another rule keys.
pub(crate) const fn hold_for_platform(
    bus: u8,
    class: u32,
    pci_class: u8,
    vendor: u16,
    device: u16,
) -> Option<Hold> {
    match bus {
        BUS_ACPI => Some(hold_for_acpi(class, pci_class, vendor, device)),
        BUS_VIRT => Some(hold_for_virt(vendor, device)),
        BUS_DEVICE_TREE => Some(hold_for_tree(device)),
        _ => None,
    }
}
