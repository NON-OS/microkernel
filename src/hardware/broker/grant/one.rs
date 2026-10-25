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

//! One grant by its id: found for an unmap to hold its claim, or taken out.

use super::table::GRANTS;
use super::types::{GrantError, MmioGrant};

// The device and claim of `pid`'s grant `grant_id`, as recorded, so an
// unmap can hold that claim while it takes the pages back (L79).
pub(in super::super) fn find(pid: u32, grant_id: u64) -> Option<(u64, u64)> {
    let grants = GRANTS.lock();
    let g = grants.iter().find(|g| g.grant_id == grant_id && g.pid == pid)?;
    Some((g.device_id, g.claim_epoch))
}

// Remove a single grant if `pid` is the holder. Returns the removed
// record on success.
pub(in super::super) fn remove(pid: u32, grant_id: u64) -> Result<MmioGrant, GrantError> {
    let mut grants = GRANTS.lock();
    let idx = grants.iter().position(|g| g.grant_id == grant_id).ok_or(GrantError::UnknownGrant)?;
    if grants[idx].pid != pid {
        return Err(GrantError::NotHolder);
    }
    Ok(grants.remove(idx))
}
