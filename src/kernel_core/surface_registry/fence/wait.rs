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

//! the project waits for its surface's count to move past `seen`.

use super::super::table::lookup_owned;
use super::super::types::{RegistryError, SurfaceSid};
use super::wait_rule::may_wait;
use super::{count, waiters};

/// The longest one call waits; a client asks again if it wants longer.
const MAX_WAIT_MS: u64 = 60_000;

/// `pid` (the thread that sleeps) of `holder` (the group that owns `sid`,
/// or supervises the guest that does: wait_rule.rs) waits until the count
/// is not `seen` or `timeout_ms` passed; 0 only reads it. Returns the count. A surface freed meanwhile is NotFound.
pub fn wait_fence(
    holder: u32,
    pid: u32,
    sid: SurfaceSid,
    seen: u32,
    timeout_ms: u64,
) -> Result<u32, RegistryError> {
    let idx = u32::try_from(sid).map_err(|_| RegistryError::InvalidArg)?;
    let start = crate::time::timestamp_millis();
    let deadline = start.saturating_add(timeout_ms.min(MAX_WAIT_MS));
    loop {
        // The wake count first, then armed, then the count read: a release
        // on another CPU after the read takes the armed waiter and moves
        // the wake count, so the sleep below does not happen.
        let token = crate::sched::wake_token(pid);
        waiters::arm(idx, pid);
        let owned = waits(holder, sid);
        let now = count::count(idx as usize).ok_or(RegistryError::InvalidArg);
        let late = crate::time::timestamp_millis() >= deadline;
        if owned.is_err() || now.is_err() || now != Ok(seen) || late || timeout_ms == 0 {
            waiters::disarm(idx, pid);
            owned?;
            return now;
        }
        crate::sched::sleep_until_unless_woken(pid, deadline, token);
        crate::sched::yield_now();
    }
}

/// Whether `holder` may wait on `sid`'s fence: NotOwner as before for any
/// group but its owner and, when a guest owns it, its supervisor.
fn waits(holder: u32, sid: SurfaceSid) -> Result<(), RegistryError> {
    let owner = match lookup_owned(holder, sid) {
        Ok(_) => return Ok(()),
        Err(RegistryError::NotOwner) => super::super::table::owner_of(sid)?,
        Err(e) => return Err(e),
    };
    let supervisor = crate::process::foreign::supervisor_of(owner);
    let supervisor = supervisor.map(crate::hardware::broker::holder_of);
    may_wait(holder, owner, supervisor).then_some(()).ok_or(RegistryError::NotOwner)
}
