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
use super::super::tables::free::free_tables;
use super::super::tables::root::clear_context;
use super::super::tables::sl_pte::{SL_ADDR_MASK, SL_LARGE, SL_READ, SL_WRITE};
use super::super::types::{DomainId, VtdError, MAX_VTD_DOMAINS};
use super::super::unit::invalidate::invalidate_all_units;
use super::super::unit::report::probed;

/// Devices still bound are denied first, and the caches dropped, before the
/// slot is freed: a freed slot is reused by the next claim, and a device left
/// pointing at it would reach whatever that claim maps. Its tables then go
/// back to the frame allocator.
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
        clear_context(binding.source)?;
    }
    if probed().is_some() {
        invalidate_all_units()?;
    }
    state.bindings.retain(|binding| binding.domain != id);
    let root = core::mem::take(&mut state.domains[index].root);
    state.domains[index].used = false;
    // The caches are dropped above, so no unit walks these tables.
    if let (true, Some(levels)) = (root != 0, page_levels()) {
        if !free_tables(root, levels, table_below) {
            crate::sys::serial::println(b"[VT-D] a destroyed domain's tables kept: unreadable");
        }
    }
    Ok(())
}

/// The table a second-level entry at `level` points to: a present entry
/// above level 1 that is not a large page.
fn table_below(entry: u64, level: u8) -> Option<u64> {
    let present = entry & (SL_READ | SL_WRITE) != 0;
    (level > 1 && present && entry & SL_LARGE == 0).then_some(entry & SL_ADDR_MASK)
}
