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

//! A release's last steps (L66, L75). A claim being released stays in the
//! table, so no newer claim of the device can be made, until every release of
//! it and every path still inside it is done; the last of them removes it.

use super::stage_rule::gone;
use super::state::claims;
use super::types::{ClaimError, Stage};

type Key = (u32, u64, u64);

fn named(key: Key) -> impl Fn(&super::Claim) -> bool {
    move |c| {
        (c.pid, c.device_id, c.epoch) == key && matches!(c.stage, Stage::Leaving | Stage::Exiting)
    }
}

/// One release of the claim at `key` done: true when it was the last, and
/// the entry has gone. None when no claim being released is at `key`.
pub(super) fn settle(key: Key) -> Option<bool> {
    let mut claims = claims();
    let at = claims.iter().position(named(key))?;
    let c = &mut claims[at];
    c.leavers = c.leavers.saturating_sub(1);
    let last = gone(c.stage, c.leavers, c.inflight);
    if last {
        super::gone::remove(claims, at);
    }
    Some(last)
}

/// Release the claim `pid` retired at `epoch`, and that one only (L69). Its
/// device stops while the entry, which this release keeps, still names it,
/// so a newer claim's device is never the one stopped (L75).
pub fn release(pid: u32, device_id: u64, epoch: u64) -> Result<u64, ClaimError> {
    let key = (pid, device_id, epoch);
    if !claims().iter().any(named(key)) {
        return Err(ClaimError::NotClaimed);
    }
    super::quiesce::stop_bus_master(device_id);
    if settle(key) == Some(true) {
        crate::hardware::broker::confine::detach(pid, device_id);
        super::wipe::step_all(true);
    }
    Ok(epoch)
}

/// Stop a device a release holds from mastering the bus, before its grants
/// are torn down. The entry stays while this release is under way (L75), so
/// the device stopped is this claim's; false, and nothing written, when no
/// claim being released is at `epoch`.
pub fn quiesce_held(pid: u32, device_id: u64, epoch: u64) -> bool {
    let held = claims().iter().any(named((pid, device_id, epoch)));
    if held {
        super::quiesce::stop_bus_master_quietly(device_id);
    }
    held
}
