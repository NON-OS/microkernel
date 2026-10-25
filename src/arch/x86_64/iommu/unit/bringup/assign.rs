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
use crate::arch::x86_64::iommu::globals::state::{DeviceBinding, VtdState, STATE};
use crate::arch::x86_64::iommu::tables::root::set_context;
use crate::arch::x86_64::iommu::types::{DomainId, SourceId, VtdError};
use crate::arch::x86_64::iommu::unit::coverage::reserved_for;
use crate::memory::phys::touches_usable;

/// How many functions got an entry, and how many of them reserved memory.
pub(super) fn assign_enumerated(
    (blocked, blocked_root): (DomainId, u64),
    levels: u8,
    address_width: u8,
) -> Result<(usize, usize), VtdError> {
    let (mut count, mut reserved) = (0, 0);
    for dev in crate::bus::pci::enumerate_devices().iter() {
        let source = bdf_to_source_id(dev.bus, dev.device, dev.function);
        if STATE.lock().bindings.iter().any(|binding| binding.source == source) {
            continue;
        }
        // Made before STATE is held, since making one takes it. As on a claim
        // (L60), no region over usable RAM or the interrupt window is mapped.
        let mut ranges = reserved_for(dev.bus, dev.device, dev.function);
        ranges.retain(|&(b, e)| !(b < 0xFEF0_0000 && e > 0xFEE0_0000) && !touches_usable(b, e));
        let own = (!ranges.is_empty()).then(|| reserved_domain(levels, &ranges).ok()).flatten();
        let start = start_for(!ranges.is_empty(), own.is_some());
        let (domain, root) =
            own.filter(|_| start == Start::Reserved).unwrap_or((blocked, blocked_root));
        let added = bind(&mut STATE.lock(), source, root, domain, address_width)?;
        reserved += usize::from(added && domain != blocked);
        count += usize::from(added);
    }
    Ok((count, reserved))
}

/// Give `source` a context entry in `domain` unless it has one already; true
/// when it was added.
pub(super) fn bind(
    state: &mut VtdState,
    source: SourceId,
    root: u64,
    domain: DomainId,
    address_width: u8,
) -> Result<bool, VtdError> {
    if state.bindings.iter().any(|binding| binding.source == source) {
        return Ok(false);
    }
    // Bookkeeping first: a device programmed but unrecorded could never be
    // detached again.
    if state.bindings.push(DeviceBinding { source, domain }).is_err() {
        return Err(VtdError::DomainTableFull);
    }
    set_context(source, root, domain, address_width)?;
    Ok(true)
}
