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

use super::super::globals::state::STATE;
use super::super::globals::{is_present, page_levels};
use super::super::tables::free::all_readable;
use super::super::tables::release::release_tables;
use super::super::tables::release_walk::child_table;
use super::super::tables::root::clear_context;
use super::super::types::{DomainId, VtdError, MAX_VTD_DOMAINS};
use super::super::unit::invalidate::invalidate_all_units;
use super::super::unit::report::probed;

/// Devices still bound are denied first, and the caches dropped, before the
/// slot is freed: a freed slot is reused by the next claim, and a device left
/// pointing at it would reach whatever that claim maps. Only then do the
/// domain's tables go back to the allocator, and only when every one
/// of them can be read. If they cannot, the slot stays taken, quarantined,
/// and the error reaches the caller.
pub fn destroy_domain(id: DomainId) -> Result<(), VtdError> {
    if !is_present() {
        return Err(VtdError::NotPresent);
    }
    let index = id.as_u16() as usize;
    if index >= MAX_VTD_DOMAINS {
        return Err(VtdError::DomainNotFound);
    }
    let mut state = STATE.lock();
    if !state.domains[index].used {
        return Err(VtdError::DomainNotFound);
    }
    for binding in state.bindings.iter().filter(|b| b.domain == id) {
        // An entry a failed attach already took back is denied already.
        match clear_context(binding.source) {
            Ok(()) | Err(VtdError::DeviceNotAttached) => {}
            Err(e) => return Err(e),
        }
    }
    if probed().is_some() {
        invalidate_all_units()?;
    }
    state.bindings.retain(|binding| binding.domain != id);
    // Never walked again, whatever happens next: a release that fails part
    // way has already freed some of these frames. The caches are dropped
    // above, so no unit walks them.
    let root = core::mem::take(&mut state.domains[index].root);
    if root != 0 {
        let levels = page_levels().ok_or(VtdError::DepthUnknown)?;
        if !all_readable(root, levels, &child_table) {
            return Err(VtdError::TableUnreachable);
        }
        release_tables(root, levels)?;
    }
    state.domains[index].used = false;
    Ok(())
}
