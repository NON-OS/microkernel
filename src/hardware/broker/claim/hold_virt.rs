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

//! Claims of the records the kernel itself publishes on its virtual bus. The
//! VMBus root (`vmbus.rs`) opens the SynIC's calls to whoever holds it: it is
//! the guest's line to the Hyper-V host, as VMMDev is VirtualBox's. Only the
//! process serving the VMBus core may claim it, so no other driver can post
//! to the host or read its messages in that core's name, before the core's
//! first claim or across its restart. Any other record on this bus is
//! refused until a unit names it. Pure, so the proofs run it on the host;
//! `hold_ids.rs` holds the numbers below to the kernel's own.

use super::hold_rule::Hold;

pub(crate) const VMBUS_SERVICE: &str = "driver.vmbus0";

/// The kernel's virtual bus kind.
pub(crate) const BUS_VIRT: u8 = 3;
/// The VMBus root's identity (`vmbus::VMBUS_VENDOR`, `vmbus::VMBUS_DEVICE`).
pub(crate) const VMBUS_VENDOR: u16 = 0x1414;
pub(crate) const VMBUS_DEVICE: u16 = 0x5642;

/// The hold on a record of the virtual bus, by its identity.
pub(crate) const fn hold_for_virt(vendor: u16, device: u16) -> Hold {
    match (vendor, device) {
        (VMBUS_VENDOR, VMBUS_DEVICE) => Hold::ForService(VMBUS_SERVICE),
        _ => Hold::Refused,
    }
}
