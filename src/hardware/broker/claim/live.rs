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

//! Whether a claim still stands, for the paths that act after their check
//! (L69). An exit moves its claims to Exiting, stops the devices and drains
//! the grant tables without waiting for those paths (L75), so a grant
//! entered, or a register written, after it would outlive the claim it was
//! made for.

use super::stage_rule::kept;
use super::state::claims;

fn stands(pid: u32, device_id: u64, epoch: u64) -> impl Fn(&super::Claim) -> bool {
    move |c| (c.pid, c.device_id, c.epoch) == (pid, device_id, epoch) && kept(c.stage)
}

/// For a grant table to ask under its own lock before it enters a grant: one
/// entered after a true answer is one the drain finds, and one refused is
/// undone by the path that made it.
pub(crate) fn live(pid: u32, device_id: u64, epoch: u64) -> bool {
    claims().iter().any(stands(pid, device_id, epoch))
}

/// `act` run under the claims' lock while the claim stands, `None` once it is
/// gone: a config write or port access lands before an exit moves the claim
/// on and stops the device, never after. `act` takes no claim lock itself.
pub(crate) fn while_live<R>(
    pid: u32,
    device_id: u64,
    epoch: u64,
    act: impl FnOnce() -> R,
) -> Option<R> {
    let claims = claims();
    let held = claims.iter().any(stands(pid, device_id, epoch));
    held.then(act)
}
