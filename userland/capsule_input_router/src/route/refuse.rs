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
//! An event from an unauthenticated Bluetooth device, checked before it is
//! routed: refused on the secure path, the release of a refused press
//! with it, and the shell told that the device must be paired again.

use nonos_libc::InputEvent;

use crate::state::Context;

use super::bond::{due, refuse};
use super::preempt::Route;
use super::press_kinds::{is_release, slot};
use super::{chord, notice};

/// Whether `event` goes nowhere, as `route` would have sent it while `hold`
/// is up (or not) with `shell` the shell's pid.
pub(super) fn refused(
    ctx: &mut Context,
    event: &InputEvent,
    (hold, route, shell): (Option<u32>, Route, u32),
    now: i64,
) -> bool {
    let slot = slot(event);
    if is_release(event.kind) && ctx.bond.refused == Some(slot) {
        ctx.bond.refused = None;
        ctx.record(0);
        return true;
    }
    let attention = chord::is_secure_attention(event.kind, event.code, event.flags);
    if !refuse(attention, hold, route, shell) {
        return false;
    }
    if attention {
        ctx.bond.refused = Some(slot);
    }
    if due(ctx.bond.told_ms, now) && notice::tell_shell(shell) {
        ctx.bond.told_ms = Some(now);
    }
    ctx.record(0);
    true
}
