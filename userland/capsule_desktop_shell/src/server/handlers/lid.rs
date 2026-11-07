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

//! The lid keys: what the lid asks of the screen (state/lid.rs), told to
//! the compositor now and, on the clock's tick, to one that has not heard.

use super::launcher_request::lookup_pid;
use crate::compositor_client::lid::lid;
use crate::state::lid::ON_CLOSE;
use crate::state::Context;

const COMPOSITOR: &[u8] = b"compositor";

/// KEY_LID_CLOSED (`closed`) or KEY_LID_OPENED, on its key-down.
pub fn key(ctx: &mut Context, closed: bool) {
    ctx.readings.lid.heard(closed, ON_CLOSE);
    // Locked before the screen goes dark, so it is locked when it wakes.
    if closed && crate::state::lid::locks(lock_on_lid(), ctx.lock.offered()) {
        crate::server::lock::lock_now(ctx);
    }
    tell(ctx);
}

/// The person's choice in Settings; on when the store cannot be asked,
/// which is also its default.
fn lock_on_lid() -> bool {
    let Some(port) = nonos_policy_client::lookup() else { return true };
    nonos_policy_client::get_bool(port, nonos_policy_proto::Field::LockOnLid) != Some(false)
}

/// Tell the compositor running now what the lid asks, if it has not heard.
pub fn tell(ctx: &mut Context) {
    let Some(pid) = lookup_pid(COMPOSITOR) else { return };
    let Some(dark) = ctx.readings.lid.due(pid) else { return };
    let rid = ctx.issue_request_id();
    // Any answer: only a missing one is asked again, on the next tick.
    if lid(ctx.ports.compositor_port, rid, dark).is_ok() {
        ctx.readings.lid.told(dark, pid);
    }
}
