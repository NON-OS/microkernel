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

//! A claim's wipe record, written as it is admitted (L78): the function's
//! last holder from here on, so its release owes the wipe. A withdrawn claim
//! writes none, so the record before it, kept or lost, stands unchanged.

use super::holder::Fresh;
use super::phase_rule::Phase;
use super::table::{Wipe, WIPES};

/// At admit: the claim's record, replacing a kept or lost one; false only
/// for a wipe still under way, which `admits` already refused.
pub(in crate::hardware::broker) fn held(device_id: u64, epoch: u64, f: &Fresh) -> bool {
    let mut all = WIPES.lock();
    if all.iter().any(|w| w.device_id == device_id && super::due_rule::pending(w.phase)) {
        return false;
    }
    all.retain(|w| w.device_id != device_id);
    all.push(Wipe {
        device_id,
        epoch,
        by: f.by,
        phase: Phase::Held,
        method: None,
        caps: f.caps,
        address: f.address,
        marker: None,
        saved: None,
        bme_off: false,
        busy: false,
    });
    true
}
