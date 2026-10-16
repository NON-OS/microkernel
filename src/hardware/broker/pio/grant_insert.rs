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

//! Adding a grant. A port belongs to one device's claimant at a time: two
//! IDE functions may both report the primary channel in compatibility mode,
//! and only one of them may drive 0x1F0. Real BARs never overlap, so for
//! them the check costs a walk and changes nothing.

use super::grant::{PioGrant, GRANTS};
use super::overlap::meets;
use super::types::PioError;

pub(super) fn insert_exclusive(record: PioGrant) -> Result<(), PioError> {
    let mut grants = GRANTS.lock();
    // Only while its claim stands, so an exit's drain finds it (L69).
    if !crate::hardware::broker::claim::live(record.pid, record.device_id, record.claim_epoch) {
        return Err(PioError::NotClaimed);
    }
    let mine = window(&record);
    if grants.iter().any(|g| g.device_id != record.device_id && meets(mine, window(g))) {
        return Err(PioError::WindowHeld);
    }
    grants.push(record);
    Ok(())
}

fn window(g: &PioGrant) -> (u16, u16) {
    (g.port_base, g.port_count)
}
