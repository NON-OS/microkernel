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

//! Every enumerated PCI function given a context entry before translation
//! starts: in the blocked domain, or in a domain of its own reserved memory.

use super::domain::reserved_domain;
use super::start::{start_for, Start};
use crate::arch::x86_64::iommu::device::bdf_to_source_id;
use crate::arch::x86_64::iommu::globals::state::{DeviceBinding, STATE};
use crate::arch::x86_64::iommu::tables::root::set_context;
use crate::arch::x86_64::iommu::types::{DomainId, VtdError};
use crate::arch::x86_64::iommu::unit::coverage::reserved_for;

/// How many functions got an entry, and how many of them reserved memory.
pub(super) fn assign_enumerated(
    (blocked, blocked_root): (DomainId, u64),
    levels: u8,
    address_width: u8,
) -> Result<(usize, usize), VtdError> {
    let devices = crate::bus::pci::enumerate_devices();
    let (mut count, mut reserved) = (0, 0);
    for dev in devices.iter() {
        let source = bdf_to_source_id(dev.bus, dev.device, dev.function);
        if STATE.lock().bindings.iter().any(|binding| binding.source == source) {
            continue;
        }
        let ranges = reserved_for(dev.bus, dev.device, dev.function);
        let own = (!ranges.is_empty()).then(|| reserved_domain(levels, &ranges).ok()).flatten();
        let (domain, root) = match (start_for(!ranges.is_empty(), own.is_some()), own) {
            (Start::Reserved, Some(own)) => own,
            _ => (blocked, blocked_root),
        };
        reserved += usize::from(domain != blocked);
        // Bookkeeping first: a device programmed but unrecorded could never be
        // detached again.
        if STATE.lock().bindings.push(DeviceBinding { source, domain }).is_err() {
            return Err(VtdError::DomainTableFull);
        }
        set_context(source, root, domain, address_width)?;
        count += 1;
    }
    Ok((count, reserved))
}
