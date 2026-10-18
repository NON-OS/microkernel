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

//! A faulting device blocked (K12): its context entry cleared and every
//! unit's caches dropped, so it reaches nothing until its holder releases
//! it and claims it again. Only a device bound to a capsule's domain is
//! blocked; one firmware's reserved memory keeps is left as it is.

use super::super::globals::state::STATE;
use super::super::tables::root::clear_context;
use super::super::types::{SourceId, VtdError};
use super::super::unit::invalidate::invalidate_all_units;

/// True when a bound device was blocked. The binding stays, so the
/// release that follows detaches it as usual.
pub fn block_device(source: u16) -> Result<bool, VtdError> {
    let source = SourceId::new(source);
    let state = STATE.lock();
    if !state.bindings.iter().any(|binding| binding.source == source) {
        return Ok(false);
    }
    clear_context(source)?;
    invalidate_all_units()?;
    Ok(true)
}
