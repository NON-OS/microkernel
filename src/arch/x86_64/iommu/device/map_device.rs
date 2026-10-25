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

//! Assigning one PCI function to a domain. The binding is recorded first, the
//! context entry written, and the caches dropped before this returns, so the
//! hardware and the bookkeeping never disagree for long. Until the entry
//! exists the device is denied, the safe direction to fail in.

use core::cell::Cell;

use super::super::globals::is_enforcing;
use super::super::globals::state::{DeviceBinding, STATE};
use super::super::tables::root::{clear_context, set_context};
use super::super::types::{DomainId, VtdError, MAX_VTD_DOMAINS};
use super::super::unit::invalidate::invalidate_all_units;
use super::super::unit::report::probed;
use super::attach_steps::attach;
use super::bdf_to_source_id::bdf_to_source_id;

pub fn map_device(domain: DomainId, bus: u8, device: u8, function: u8) -> Result<(), VtdError> {
    // Not `is_present`: a described unit that is not translating would let
    // this write a context entry no hardware reads, and report success.
    if !is_enforcing() {
        return Err(VtdError::NotEnforcing);
    }
    let info = probed().ok_or(VtdError::NotPresent)?;
    let index = domain.as_u16() as usize;
    if index >= MAX_VTD_DOMAINS {
        return Err(VtdError::DomainNotFound);
    }
    let source = bdf_to_source_id(bus, device, function);

    let mut state = STATE.lock();
    if !state.domains[index].used {
        return Err(VtdError::DomainNotFound);
    }
    if state.bindings.iter().any(|binding| binding.source == source) {
        return Err(VtdError::DeviceAlreadyAttached);
    }
    let sl_root = state.domains[index].root;

    // Space for the binding first: a device programmed with nowhere to record
    // it could never be detached again.
    state.bindings.push(DeviceBinding { source, domain }).map_err(|_| VtdError::DomainTableFull)?;

    // Written, then the caches dropped; a failed drop takes the entry out and
    // drops again (K12). Kept on doubt, the binding has destroy_domain deny it.
    let aw = info.levels.context_aw();
    let (wrote, cleared, dropped) = (Cell::new(false), Cell::new(false), Cell::new(false));
    let programmed = attach(
        || set_context(source, sl_root, domain, aw).inspect(|_| wrote.set(true)),
        || invalidate_all_units().inspect(|_| dropped.set(true)),
        || clear_context(source).inspect(|_| cleared.set(true)),
    );
    if let Err(e) = programmed {
        if !wrote.get() || (cleared.get() && dropped.get()) {
            state.bindings.retain(|binding| binding.source != source);
        }
        return Err(e);
    }
    Ok(())
}
