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

//! The devices firmware declares only in the ACPI namespace (DMAR ANDD with
//! a namespace device scope): an SD or eMMC host, LPSS I2C or UART, in ACPI
//! mode. PCI enumeration never sees them, so without this they would have no
//! context entry for a capsule's claim to move. Like every device the kernel
//! found, they start in the blocked domain and reach nothing until then.

use super::assign::bind;
use crate::arch::x86_64::acpi::parser::other::{namespace_devices, namespace_requester};
use crate::arch::x86_64::iommu::device::bdf_to_source_id;
use crate::arch::x86_64::iommu::globals::state::STATE;
use crate::arch::x86_64::iommu::types::{DomainId, VtdError};

pub(super) fn assign_named(
    root: u64,
    domain: DomainId,
    address_width: u8,
) -> Result<usize, VtdError> {
    let mut state = STATE.lock();
    let mut count = 0;
    for device in namespace_devices().iter() {
        // A path through bridges gives no requester id here; the device is
        // named at bring-up as left out, never guessed into a context entry.
        let Some((_, (bus, dev, func))) = namespace_requester(device.name()) else {
            continue;
        };
        let source = bdf_to_source_id(bus, dev, func);
        count += usize::from(bind(&mut state, source, root, domain, address_width)?);
    }
    Ok(count)
}
