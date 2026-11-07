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

//! What the router is told: locked with the fields where the pointer is
//! drawn, or let go. Told on each change and again every second while
//! locked, so a router that restarted hears it within a second.

use nonos_libc::mk_uptime_ms;

use crate::input_router_client::{session_lock, LockWord};
use crate::state::Context;

pub(super) const RETELL_MS: i64 = 1000;

pub(super) fn tell(ctx: &mut Context) {
    let word = if ctx.lock.screen.covers() { LockWord::Lock } else { LockWord::Release };
    let shown = super::shown(ctx);
    let rid = ctx.issue_request_id();
    if session_lock(ctx.ports.input_router_port, rid, word, &shown).is_ok() {
        ctx.lock.told_ms = mk_uptime_ms();
    }
}

/// Whether the router holds the session locked, asked at start: a shell
/// that restarted while locked finds it so. None when it did not answer.
pub(super) fn ask(ctx: &mut Context) -> Option<bool> {
    let rid = ctx.issue_request_id();
    session_lock(ctx.ports.input_router_port, rid, LockWord::Ask, &[]).ok()
}
