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

use core::ops::Deref;

use super::stage_rule::{gone, grantable};
use super::state::claims;
use super::types::{Claim, Stage};

/// A ready claim, held by one grant path for as long as this lives. A
/// release waits for every one to go before it drains, so a grant whose
/// check passed is recorded before the drain looks for it (L66).
pub(crate) struct InUse(Claim);

impl Deref for InUse {
    type Target = Claim;
    fn deref(&self) -> &Claim {
        &self.0
    }
}

/// The last path out of a claim its exit has finished with takes the entry
/// away: the exit did not wait for it, and kept the entry so no newer claim
/// could be made while it still writes the device (L75).
impl Drop for InUse {
    fn drop(&mut self) {
        let key = (self.0.pid, self.0.device_id, self.0.epoch);
        let mut claims = claims();
        let at = claims.iter().position(|c| (c.pid, c.device_id, c.epoch) == key);
        let Some(at) = at else { return };
        let c = &mut claims[at];
        c.inflight = c.inflight.saturating_sub(1);
        if gone(c.stage, c.leavers, c.inflight) {
            super::gone::remove(claims, at);
        }
    }
}

/// The device's claim once it is made and until its release begins. One
/// still being made, or being released, reads as none, so nothing is mapped,
/// bound or written for it (L65, L66).
pub(crate) fn lookup(device_id: u64) -> Option<InUse> {
    let mut claims = claims();
    let c = claims.iter_mut().find(|c| c.device_id == device_id && grantable(c.stage))?;
    c.inflight = c.inflight.checked_add(1)?;
    Some(InUse(*c))
}

/// The claim a grant was made under, held by an unmap of that grant until its
/// pages are gone (L79), whatever its release has reached: a release waits it
/// out and the entry outlasts it, so neither the device's next holder nor a
/// claim taking its scanout is let in while those pages still show it.
pub(crate) fn inside(pid: u32, device_id: u64, epoch: u64) -> Option<InUse> {
    let mut claims = claims();
    let held = |c: &&mut Claim| (c.pid, c.device_id, c.epoch) == (pid, device_id, epoch);
    let c = claims.iter_mut().find(held)?;
    if !matches!(c.stage, Stage::Ready | Stage::Leaving | Stage::Exiting) {
        return None;
    }
    c.inflight = c.inflight.checked_add(1)?;
    Some(InUse(*c))
}
