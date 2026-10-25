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

//! A release's first step (L66). A release drained a device's grants while a
//! sibling thread's map, past its check, had yet to record one: that grant
//! outlived the claim and reached the next claimant's device. So the claim
//! stops being granted against first, the grant paths already inside it are
//! waited out, and only then are its grants drained.

use super::stage_rule::retired;
use super::state::claims;
use super::types::{Claim, ClaimError};

/// The claim's epoch, once nothing more can be granted against it. A claim
/// still being made is the claiming call's to settle, one an exit is tearing
/// down the exit's; one a sibling's release already began is waited out
/// again, both drain the same grants, and the entry goes with the last.
pub(crate) fn retire(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    let epoch = {
        let mut claims = claims();
        let c = claims.iter_mut().find(|c| c.device_id == device_id);
        let c = c.ok_or(ClaimError::NotClaimed)?;
        if c.pid != pid {
            return Err(ClaimError::NotHolder);
        }
        c.stage = retired(c.stage)?;
        // Counted until its release, so the entry outlasts this teardown (L75).
        c.leavers = c.leavers.checked_add(1).ok_or(ClaimError::NotClaimed)?;
        c.epoch
    };
    let inside = |c: &Claim| c.device_id == device_id && c.epoch == epoch && c.inflight > 0;
    // Spun, never yielded: a thread killed where it yields is parked for good,
    // and this one would hold its count for ever (L77). No path inside a claim
    // yields and kernel code is never preempted, so each finishes on its own
    // CPU; shootdowns are answered meanwhile, as the claims' lock does.
    while claims().iter().any(inside) {
        crate::smp::serve_shootdowns();
        core::hint::spin_loop();
    }
    Ok(epoch)
}
